// Offload: linked computers, pairing and unattended access, backed by
// `main_offload_call` in the Rust core (src/offload/api.rs).

import 'dart:async';
import 'dart:convert';

import 'package:flutter/foundation.dart';
import 'package:flutter_hbb/models/platform_model.dart';

class OffloadDevice {
  final String id;
  final String name;
  final String platform;
  final int addedAt;
  final int lastSeen;
  bool? online;

  OffloadDevice({
    required this.id,
    required this.name,
    required this.platform,
    required this.addedAt,
    required this.lastSeen,
    this.online,
  });

  factory OffloadDevice.fromJson(Map<String, dynamic> j) => OffloadDevice(
        id: j['id'] ?? '',
        name: j['name'] ?? '',
        platform: j['platform'] ?? '',
        addedAt: j['added_at'] ?? 0,
        lastSeen: j['last_seen'] ?? 0,
      );

  bool get isWindows => platform.toLowerCase().contains('windows');
  bool get isMac => platform.toLowerCase().contains('mac');
}

class OffloadResult {
  final bool ok;
  final String error;
  final Map<String, dynamic> data;
  OffloadResult(this.ok, this.error, this.data);

  Map<String, dynamic> get status =>
      (data['status'] as Map?)?.cast<String, dynamic>() ?? const {};
  String get state => status['state'] ?? '';
}

Future<OffloadResult> offloadCall(String method,
    [Map<String, dynamic> args = const {}]) async {
  try {
    final s =
        await bind.mainOffloadCall(method: method, args: jsonEncode(args));
    final m = (jsonDecode(s) as Map).cast<String, dynamic>();
    return OffloadResult(m['ok'] == true, m['error'] ?? '', m);
  } catch (e) {
    return OffloadResult(false, '$e', const {});
  }
}

class OffloadModel with ChangeNotifier {
  static const _onlineEvent = 'callback_query_onlines';
  static const _listenerKey = 'OffloadModel';

  List<OffloadDevice> devices = [];
  bool unattended = false;
  String myId = '';
  String? selectedId;
  String error = '';
  bool loaded = false;
  Timer? _timer;
  int _users = 0;

  OffloadDevice? get selected {
    for (final d in devices) {
      if (d.id == selectedId) return d;
    }
    return devices.isEmpty ? null : devices.first;
  }

  void attach() {
    if (_users++ > 0) return;
    platformFFI.registerEventHandler(_onlineEvent, _listenerKey, (evt) async {
      final on = '${evt['onlines'] ?? ''}'.split(',').toSet();
      final off = '${evt['offlines'] ?? ''}'.split(',').toSet();
      var changed = false;
      for (final d in devices) {
        final v = on.contains(d.id) ? true : (off.contains(d.id) ? false : d.online);
        if (v != d.online) {
          d.online = v;
          changed = true;
        }
      }
      if (changed) notifyListeners();
    });
    refresh();
    _timer = Timer.periodic(const Duration(seconds: 5), (_) => refresh());
  }

  void detach() {
    if (--_users > 0) return;
    _timer?.cancel();
    _timer = null;
    platformFFI.unregisterEventHandler(_onlineEvent, _listenerKey);
  }

  Future<void> refresh() async {
    final r = await offloadCall('devices');
    if (!r.ok) {
      error = r.error;
      notifyListeners();
      return;
    }
    error = '';
    final previous = {for (final d in devices) d.id: d.online};
    devices = ((r.data['devices'] as List?) ?? [])
        .map((e) => OffloadDevice.fromJson((e as Map).cast<String, dynamic>()))
        .toList();
    for (final d in devices) {
      d.online = previous[d.id];
    }
    devices.sort((a, b) => a.name.toLowerCase().compareTo(b.name.toLowerCase()));
    unattended = r.data['unattended'] == true;
    myId = r.data['my_id'] ?? '';
    loaded = true;
    if (devices.isNotEmpty) {
      bind.queryOnlines(ids: devices.map((d) => d.id).toList());
    }
    notifyListeners();
  }

  void select(String id) {
    selectedId = id;
    notifyListeners();
  }

  Future<OffloadResult> remove(String id) async {
    final r = await offloadCall('device_remove', {'id': id});
    await refresh();
    return r;
  }

  Future<OffloadResult> rename(String id, String name) async {
    final r = await offloadCall('device_rename', {'id': id, 'name': name});
    await refresh();
    return r;
  }

  Future<OffloadResult> removeAll() async {
    final r = await offloadCall('devices_clear');
    await refresh();
    return r;
  }

  Future<OffloadResult> setUnattended(bool on) async {
    final r = await offloadCall('unattended_set', {'on': on});
    await refresh();
    return r;
  }

  Future<OffloadResult> disconnectAll() => offloadCall('disconnect');
}

final offloadModel = OffloadModel();
