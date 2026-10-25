#!/bin/bash
# Headless end-to-end test of Offload's pairing and device-signature login on one Linux host.
#
# Two "computers" are simulated by two Linux accounts (separate configs, keys and IPC), each
# running the real Offload service binary against a real Offload server (hbbs, hbbr and the
# pairing directory). Everything goes through the production code paths: rendezvous, hole
# punching or relay, the end-to-end encrypted channel, pairing, pinning, signature logins,
# unattended policy and revocation. What it cannot cover: screen capture and input (no
# display), Windows/macOS specifics, and different networks.
#
#   sudo ./headless-e2e.sh /path/to/offload-binary
set -uo pipefail

BIN="${1:?usage: sudo $0 /path/to/offload-binary}"
[[ $EUID -eq 0 ]] || { echo "run as root"; exit 2; }
WORK=/opt/offload-e2e
mkdir -p "$WORK"
install -m 0755 "$BIN" "$WORK/offload"
PASS=0
FAIL=0
RESULTS=()

for u in offa offb; do
  id "$u" >/dev/null 2>&1 || useradd -m -s /bin/bash "$u"
done

as() { local u="$1"; shift; sudo -u "$u" -H env HOME="/home/$u" XDG_RUNTIME_DIR="/tmp/rt-$u" "$WORK/offload" "$@"; }
oc() { local u="$1"; shift; as "$u" --offload "$@" 2>/dev/null | tail -1; }
field() { python3 -c 'import json,sys; d=json.loads(sys.argv[1]);
for k in sys.argv[2].split("."):
    d = d.get(k, {}) if isinstance(d, dict) else {}
print(d if not isinstance(d, (dict, list)) else json.dumps(d))' "$1" "$2"; }
has_device() { python3 -c 'import json,sys; d=json.loads(sys.argv[1]); sys.exit(0 if any(x.get("id")==sys.argv[2] for x in d.get("devices",[])) else 1)' "$1" "$2"; }
check() {
  local name="$1" cond="$2" detail="${3:-}"
  if [[ "$cond" == 1 ]]; then PASS=$((PASS+1)); RESULTS+=("PASS  $name"); echo "PASS  $name";
  else FAIL=$((FAIL+1)); RESULTS+=("FAIL  $name  $detail"); echo "FAIL  $name  $detail"; fi
}

cleanup() {
  pkill -u offa -f "$WORK/offload" 2>/dev/null
  pkill -u offb -f "$WORK/offload" 2>/dev/null
}
trap cleanup EXIT
cleanup
# Both accounts share this machine's network card, from which device ids are derived, so each
# simulated computer is given its own id up front, as two real computers would have.
for u in offa offb; do
  rm -rf "/home/$u/.config/offload" "/home/$u/.config/Offload" "/tmp/rt-$u"
  mkdir -p "/tmp/rt-$u" "/home/$u/.config/offload" && chmod 700 "/tmp/rt-$u"
  echo "id = '$(shuf -i 700000000-799999999 -n 1)'" > "/home/$u/.config/offload/Offload.toml"
  chown -R "$u" "/tmp/rt-$u" "/home/$u/.config"
  as "$u" --server >"/tmp/offload-$u.log" 2>&1 &
done

echo "== waiting for both services to register with the server"
for _ in $(seq 1 60); do
  A_ID=$(field "$(oc offa info)" my_id); B_ID=$(field "$(oc offb info)" my_id)
  [[ -n "$A_ID" && -n "$B_ID" ]] && break
  sleep 1
done
echo "A=$A_ID B=$B_ID"
check "both devices have distinct ids" "$([[ -n $A_ID && -n $B_ID && $A_ID != "$B_ID" ]] && echo 1 || echo 0)"

pair_code() {
  local r
  for _ in $(seq 1 30); do
    r=$(oc offa pair_start)
    [[ "$(field "$r" ok)" == "True" ]] && { field "$r" status.code; return; }
    sleep 2
  done
  echo "ERR:$r"
}

echo "== TEST 3/4/5: pair with a code, approve, persistent trust"
CODE=$(pair_code)
check "A shows a code" "$([[ $CODE =~ ^[0-9A-Z]{4}-[0-9A-Z]{4}$ ]] && echo 1 || echo 0)" "$CODE"
oc offb link "{\"code\":\"$CODE\"}" > /tmp/link-b.json &
LINK_PID=$!
W=$(oc offa pair_wait '{"wait":60}')
check "A is asked to approve B" "$([[ $(field "$W" status.state) == approval && $(field "$W" status.requester.id) == "$B_ID" ]] && echo 1 || echo 0)" "$W"
D=$(oc offa pair_decide "{\"id\":\"$B_ID\",\"approve\":true}")
check "A approves" "$([[ $(field "$D" ok) == True ]] && echo 1 || echo 0)" "$D"
wait $LINK_PID
L=$(cat /tmp/link-b.json)
check "B reports linked with A" "$([[ $(field "$L" status.state) == linked && $(field "$L" status.id) == "$A_ID" ]] && echo 1 || echo 0)" "$L"
check "A trusts B" "$(has_device "$(oc offa devices)" "$B_ID" && echo 1 || echo 0)"
check "B trusts A" "$(has_device "$(oc offb devices)" "$A_ID" && echo 1 || echo 0)"
R=$(oc offb link "{\"code\":\"$CODE\"}")
check "a used code cannot be used again" "$([[ $(field "$R" status.state) == failed ]] && echo 1 || echo 0)" "$R"

echo "== attended: a linked device still needs someone to accept"
P=$(oc offb probe "{\"id\":\"$A_ID\",\"mode\":\"login\",\"wait\":15}")
check "without unattended access nobody accepted, so no session" "$([[ $(field "$P" ok) != True ]] && echo 1 || echo 0)" "$P"

echo "== TEST 12: unattended access"
U=$(oc offa unattended_set '{"on":true}')
check "A turns unattended access on" "$([[ $(field "$U" ok) == True ]] && echo 1 || echo 0)" "$U"
P=$(oc offb probe "{\"id\":\"$A_ID\",\"mode\":\"login\",\"wait\":30}")
check "B logs in to A (file session) without approval" "$([[ $(field "$P" ok) == True && $(field "$P" secured) == True ]] && echo 1 || echo 0)" "$P"
echo "     transport=$(field "$P" transport) direct=$(field "$P" direct) peer=$(field "$P" peer)"

echo "== TEST 14 (Linux stand-in): run a computation on A from B"
P=$(oc offb probe "{\"id\":\"$A_ID\",\"mode\":\"run\",\"command\":\"echo result=\$((6*7)) on \$(hostname)\",\"wait\":30}")
check "command runs on A and its output comes back" "$(field "$P" output | grep -q "result=42" && echo 1 || echo 0)" "$(field "$P" error)"

echo "== TEST 11: forced relay"
P=$(oc offb probe "{\"id\":\"$A_ID\",\"mode\":\"login\",\"wait\":30,\"relay\":true}")
check "session works through the relay" "$([[ $(field "$P" ok) == True && $(field "$P" direct) == False ]] && echo 1 || echo 0)" "$P"
echo "     transport=$(field "$P" transport)"

echo "== TEST 9: reconnect without pairing again"
P=$(oc offb probe "{\"id\":\"$A_ID\",\"mode\":\"login\",\"wait\":30}")
check "second session needs no code" "$([[ $(field "$P" ok) == True ]] && echo 1 || echo 0)" "$P"

echo "== reverse direction (TEST 7 stand-in): A connects to B"
oc offb unattended_set '{"on":true}' >/dev/null
P=$(oc offa probe "{\"id\":\"$B_ID\",\"mode\":\"login\",\"wait\":30}")
check "A opens a session on B" "$([[ $(field "$P" ok) == True ]] && echo 1 || echo 0)" "$P"

echo "== wrong codes"
CODE2=$(pair_code)
SLOT=${CODE2%%-*}
for g in 0000 0001 0002; do
  R=$(oc offb link "{\"code\":\"$SLOT-$g\"}")
done
check "three wrong codes lock the code" "$(field "$R" status.error | grep -qi "too many\|not right" && echo 1 || echo 0)" "$R"
S=$(oc offa pair_status)
check "A shows the code as locked" "$([[ $(field "$S" status.state) == locked ]] && echo 1 || echo 0)" "$S"
oc offa pair_cancel >/dev/null

echo "== TEST 13: revoke"
X=$(oc offa device_remove "{\"id\":\"$B_ID\"}")
check "A removes B" "$([[ $(field "$X" ok) == True ]] && echo 1 || echo 0)" "$X"
P=$(oc offb probe "{\"id\":\"$A_ID\",\"mode\":\"login\",\"wait\":20}")
check "B is refused after revocation" "$([[ $(field "$P" ok) != True ]] && field "$P" error | grep -qi "not linked" && echo 1 || echo 0)" "$P"

echo "== remove all also turns unattended off"
C=$(oc offb devices_clear)
check "B removes all devices" "$([[ $(field "$C" ok) == True && $(field "$(oc offb devices)" unattended) == False ]] && echo 1 || echo 0)" "$C"

echo
printf '%s\n' "${RESULTS[@]}"
echo "passed $PASS, failed $FAIL"
[[ $FAIL -eq 0 ]]
