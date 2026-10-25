// Offload main screen: linked computers, the three actions, and linking.

import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_hbb/common.dart';
import 'package:flutter_hbb/desktop/pages/connection_page.dart';
import 'package:flutter_hbb/models/offload_model.dart';
import 'package:flutter_hbb/models/platform_model.dart';
import 'package:get/get.dart';
import 'package:provider/provider.dart';

const _accent = Color(0xFF3D5AFE);

class OffloadHome extends StatefulWidget {
  const OffloadHome({Key? key}) : super(key: key);

  @override
  State<OffloadHome> createState() => _OffloadHomeState();
}

class _OffloadHomeState extends State<OffloadHome> {
  bool _keyConfigured = true;

  @override
  void initState() {
    super.initState();
    offloadModel.attach();
    offloadCall('info').then((r) {
      if (mounted && r.data['key_configured'] == false) {
        setState(() => _keyConfigured = false);
      }
    });
  }

  @override
  void dispose() {
    offloadModel.detach();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return ChangeNotifierProvider.value(
      value: offloadModel,
      child: Consumer<OffloadModel>(builder: (context, model, _) {
        return Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Expanded(
              child: SingleChildScrollView(
                padding: const EdgeInsets.fromLTRB(28, 24, 28, 12),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    Text('OFFLOAD',
                        style: TextStyle(
                            fontSize: 26,
                            fontWeight: FontWeight.w800,
                            letterSpacing: 4,
                            color: Theme.of(context).textTheme.titleLarge?.color)),
                    const SizedBox(height: 20),
                    if (!_keyConfigured)
                      Container(
                        margin: const EdgeInsets.only(bottom: 16),
                        padding: const EdgeInsets.all(12),
                        decoration: BoxDecoration(
                            color: Colors.red.withOpacity(0.1),
                            borderRadius: BorderRadius.circular(10)),
                        child: const Text(
                            'This copy of Offload was built without a server, so it cannot connect to other computers. '
                            'Install a build made for your Offload server.',
                            style: TextStyle(color: Colors.red)),
                      ),
                    if (!model.loaded)
                      const Padding(
                        padding: EdgeInsets.all(40),
                        child: Center(child: CircularProgressIndicator()),
                      )
                    else if (model.devices.isEmpty)
                      _FirstRun(model: model)
                    else
                      _Devices(model: model),
                    if (model.error.isNotEmpty)
                      Padding(
                        padding: const EdgeInsets.only(top: 12),
                        child: Text(model.error,
                            style: const TextStyle(color: Colors.red)),
                      ),
                  ],
                ),
              ),
            ),
            const Divider(height: 1),
            OnlineStatusWidget().marginOnly(left: 12, right: 12, bottom: 6),
          ],
        );
      }),
    );
  }
}

class _FirstRun extends StatelessWidget {
  final OffloadModel model;
  const _FirstRun({required this.model});

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Text('Link your computers once. After that they connect with one click.',
            style: TextStyle(
                fontSize: 15,
                color: Theme.of(context).textTheme.bodyMedium?.color?.withOpacity(0.7))),
        const SizedBox(height: 24),
        Wrap(
          spacing: 16,
          runSpacing: 16,
          children: [
            _BigChoice(
              icon: Icons.link,
              title: 'LINK A COMPUTER',
              subtitle: 'Type the code shown on your other computer.',
              onTap: () => showLinkDialog(context),
            ),
            _BigChoice(
              icon: Icons.password,
              title: "THIS COMPUTER'S CODE",
              subtitle: 'Show a code to type on your other computer.',
              onTap: () => showMyCodeDialog(context),
            ),
          ],
        ),
      ],
    );
  }
}

class _BigChoice extends StatelessWidget {
  final IconData icon;
  final String title;
  final String subtitle;
  final VoidCallback onTap;
  const _BigChoice(
      {required this.icon,
      required this.title,
      required this.subtitle,
      required this.onTap});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 260,
      height: 150,
      child: Material(
        color: Theme.of(context).cardColor,
        borderRadius: BorderRadius.circular(14),
        child: InkWell(
          borderRadius: BorderRadius.circular(14),
          onTap: onTap,
          child: Padding(
            padding: const EdgeInsets.all(18),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Icon(icon, size: 34, color: _accent),
                const Spacer(),
                Text(title,
                    style: const TextStyle(
                        fontSize: 16, fontWeight: FontWeight.w700, letterSpacing: 1)),
                const SizedBox(height: 4),
                Text(subtitle,
                    style: TextStyle(
                        fontSize: 13,
                        color: Theme.of(context)
                            .textTheme
                            .bodyMedium
                            ?.color
                            ?.withOpacity(0.6))),
              ],
            ),
          ),
        ),
      ),
    );
  }
}

IconData _platformIcon(OffloadDevice d) {
  if (d.isWindows) return Icons.desktop_windows_outlined;
  if (d.isMac) return Icons.laptop_mac;
  return Icons.computer;
}

class _Devices extends StatelessWidget {
  final OffloadModel model;
  const _Devices({required this.model});

  @override
  Widget build(BuildContext context) {
    final selected = model.selected;
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text('My Computers',
            style: TextStyle(fontSize: 16, fontWeight: FontWeight.w600)),
        const SizedBox(height: 10),
        Material(
          color: Theme.of(context).cardColor,
          borderRadius: BorderRadius.circular(12),
          clipBehavior: Clip.antiAlias,
          child: Column(
            children: [
              for (final d in model.devices) ...[
                if (d != model.devices.first) const Divider(height: 1),
                ListTile(
                  selected: d.id == selected?.id,
                  selectedTileColor: _accent.withOpacity(0.08),
                  leading: Icon(_platformIcon(d)),
                  title: Text(d.name,
                      style: const TextStyle(fontWeight: FontWeight.w500)),
                  trailing: _OnlineBadge(online: d.online),
                  onTap: () => model.select(d.id),
                  onLongPress: () => showDeviceMenu(context, d),
                ),
              ],
            ],
          ),
        ),
        if (selected != null) ...[
          const SizedBox(height: 18),
          Text(selected.name,
              style: TextStyle(
                  fontSize: 13,
                  color: Theme.of(context).textTheme.bodyMedium?.color?.withOpacity(0.6))),
          const SizedBox(height: 8),
          Wrap(
            spacing: 10,
            runSpacing: 10,
            children: [
              _ActionButton(
                  icon: Icons.screen_share_outlined,
                  label: 'CONNECT',
                  primary: true,
                  onPressed: () => connect(context, selected.id)),
              _ActionButton(
                  icon: Icons.folder_copy_outlined,
                  label: 'TRANSFER FILES',
                  onPressed: () =>
                      connect(context, selected.id, isFileTransfer: true)),
              _ActionButton(
                  icon: Icons.terminal,
                  label: 'REMOTE COMPUTING',
                  onPressed: () => showRemoteComputingDialog(context, selected)),
              IconButton(
                tooltip: 'Rename or remove',
                icon: const Icon(Icons.more_horiz),
                onPressed: () => showDeviceMenu(context, selected),
              ),
            ],
          ),
        ],
        const SizedBox(height: 26),
        Wrap(spacing: 10, runSpacing: 10, children: [
          OutlinedButton.icon(
            icon: const Icon(Icons.add_link),
            label: const Text('LINK ANOTHER COMPUTER'),
            onPressed: () => showLinkDialog(context),
          ),
          TextButton.icon(
            icon: const Icon(Icons.password),
            label: const Text("THIS COMPUTER'S CODE"),
            onPressed: () => showMyCodeDialog(context),
          ),
        ]),
      ],
    );
  }
}

class _OnlineBadge extends StatelessWidget {
  final bool? online;
  const _OnlineBadge({required this.online});

  @override
  Widget build(BuildContext context) {
    final text = online == null ? '…' : (online! ? 'Online' : 'Offline');
    final color = online == true ? Colors.green : Colors.grey;
    return Row(mainAxisSize: MainAxisSize.min, children: [
      Container(
          width: 8,
          height: 8,
          decoration: BoxDecoration(color: color, shape: BoxShape.circle)),
      const SizedBox(width: 6),
      Text(text, style: TextStyle(color: color, fontSize: 13)),
    ]);
  }
}

class _ActionButton extends StatelessWidget {
  final IconData icon;
  final String label;
  final bool primary;
  final VoidCallback onPressed;
  const _ActionButton(
      {required this.icon,
      required this.label,
      required this.onPressed,
      this.primary = false});

  @override
  Widget build(BuildContext context) {
    final style = ButtonStyle(
      padding: WidgetStateProperty.all(
          const EdgeInsets.symmetric(horizontal: 18, vertical: 16)),
    );
    return primary
        ? ElevatedButton.icon(
            style: style.copyWith(
                backgroundColor: WidgetStateProperty.all(_accent),
                foregroundColor: WidgetStateProperty.all(Colors.white)),
            icon: Icon(icon),
            label: Text(label),
            onPressed: onPressed)
        : OutlinedButton.icon(
            style: style, icon: Icon(icon), label: Text(label), onPressed: onPressed);
  }
}

// ---------------------------------------------------------------- dialogs

String _linkStateText(String state) {
  switch (state) {
    case 'resolving':
      return 'Finding the other computer…';
    case 'connecting':
      return 'Connecting securely…';
    case 'waiting':
      return 'Waiting for approval on the other computer…';
    default:
      return '';
  }
}

/// "LINK A COMPUTER": type the code shown on the other computer.
void showLinkDialog(BuildContext context) {
  final controller = TextEditingController();
  String state = 'idle';
  String message = '';
  Timer? poll;

  gFFI.dialogManager.show((setState, close, context) {
    void stop() {
      poll?.cancel();
      poll = null;
    }

    Future<void> start() async {
      final code = controller.text.trim();
      if (code.isEmpty) return;
      final r = await offloadCall('link_start', {'code': code});
      setState(() {
        state = r.state.isEmpty ? 'failed' : r.state;
        message = r.ok ? '' : r.error;
      });
      stop();
      poll = Timer.periodic(const Duration(milliseconds: 700), (_) async {
        final s = await offloadCall('link_status');
        final st = s.state;
        setState(() {
          state = st;
          if (st == 'linked') {
            message = 'Linked with ${s.status['name']}.';
          } else if (st == 'failed') {
            message = '${s.status['error'] ?? 'Linking failed.'}';
          }
        });
        if (st == 'linked' || st == 'failed' || st == 'idle') {
          stop();
          offloadModel.refresh();
        }
      });
    }

    void cancel() {
      stop();
      if (state != 'linked') offloadCall('link_cancel');
      close();
    }

    final busy = state == 'resolving' || state == 'connecting' || state == 'waiting';
    return CustomAlertDialog(
      title: const Text('Link a computer'),
      content: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text(
              "On your other computer, open Offload and choose THIS COMPUTER'S CODE. Type the code here."),
          const SizedBox(height: 8),
          SelectableText(
              'This computer: ID ${offloadModel.myId}  ·  fingerprint ${offloadModel.myFingerprint}',
              style: const TextStyle(color: Colors.grey, fontSize: 12)),
          const SizedBox(height: 16),
          TextField(
            controller: controller,
            enabled: !busy && state != 'linked',
            autofocus: true,
            textAlign: TextAlign.center,
            textCapitalization: TextCapitalization.characters,
            inputFormatters: [
              LengthLimitingTextInputFormatter(11),
              FilteringTextInputFormatter.allow(RegExp(r'[0-9A-Za-z\- ]')),
            ],
            style: const TextStyle(
                fontSize: 28, letterSpacing: 6, fontWeight: FontWeight.w600),
            decoration: const InputDecoration(hintText: 'XXXX-XXXX'),
            onSubmitted: (_) => busy ? null : start(),
          ),
          const SizedBox(height: 14),
          if (busy)
            Row(children: [
              const SizedBox(
                  width: 16,
                  height: 16,
                  child: CircularProgressIndicator(strokeWidth: 2)),
              const SizedBox(width: 10),
              Flexible(child: Text(_linkStateText(state))),
            ]),
          if (message.isNotEmpty)
            Text(message,
                style: TextStyle(
                    color: state == 'linked' ? Colors.green : Colors.red)),
        ],
      ),
      actions: [
        if (state == 'linked')
          dialogButton('Done', onPressed: cancel)
        else ...[
          dialogButton('Cancel', onPressed: cancel, isOutline: true),
          dialogButton('Link', onPressed: busy ? null : start),
        ],
      ],
      onCancel: cancel,
    );
  });
}

/// "THIS COMPUTER'S CODE": show a one-time code and approve the request it brings.
void showMyCodeDialog(BuildContext context) {
  Map<String, dynamic> status = {'state': 'starting'};
  String error = '';
  Timer? poll;
  bool started = false;

  gFFI.dialogManager.show((setState, close, context) {
    void stop() {
      poll?.cancel();
      poll = null;
    }

    Future<void> newCode() async {
      stop();
      setState(() {
        status = {'state': 'starting'};
        error = '';
      });
      final r = await offloadCall('pair_start');
      if (!r.ok) {
        setState(() {
          status = {'state': 'error'};
          error = r.error;
        });
        return;
      }
      setState(() => status = r.status);
      poll = Timer.periodic(const Duration(seconds: 1), (_) async {
        final s = await offloadCall('pair_status');
        if (!s.ok) return;
        setState(() => status = s.status);
        final st = s.state;
        if (st == 'approved') offloadModel.refresh();
        if (st != 'waiting' && st != 'approval') stop();
      });
    }

    if (!started) {
      started = true;
      Future.microtask(newCode);
    }

    void finish() {
      stop();
      if (status['state'] != 'approved') offloadCall('pair_cancel');
      close();
    }

    Future<void> decide(bool approve) async {
      final req = (status['requester'] as Map?) ?? {};
      final r = await offloadCall(
          'pair_decide', {'id': '${req['id'] ?? ''}', 'approve': approve});
      if (!r.ok) setState(() => error = r.error);
    }

    final state = '${status['state']}';
    Widget body;
    List<Widget> actions;
    switch (state) {
      case 'waiting':
        final secs = (status['expires_in'] ?? 0) as int;
        body = Column(mainAxisSize: MainAxisSize.min, children: [
          const Text('Type this code on your other computer:'),
          const SizedBox(height: 16),
          SelectableText('${status['code']}',
              style: const TextStyle(
                  fontSize: 44, letterSpacing: 8, fontWeight: FontWeight.w700)),
          const SizedBox(height: 10),
          Text(
              'Works once. Expires in ${secs ~/ 60}:${(secs % 60).toString().padLeft(2, '0')}.',
              style: const TextStyle(color: Colors.grey)),
        ]);
        actions = [dialogButton('Cancel', onPressed: finish, isOutline: true)];
        break;
      case 'approval':
        final req = (status['requester'] as Map?) ?? {};
        body = Column(mainAxisSize: MainAxisSize.min, children: [
          const Icon(Icons.devices, size: 40, color: _accent),
          const SizedBox(height: 12),
          Text(
              '${req['name']}${(req['platform'] ?? '') != '' ? ' (${req['platform']})' : ''} wants to link with this computer.',
              textAlign: TextAlign.center,
              style: const TextStyle(fontSize: 17, fontWeight: FontWeight.w600)),
          const SizedBox(height: 10),
          SelectableText(
              'ID ${req['id']}\nFingerprint ${req['fingerprint'] ?? ''}',
              textAlign: TextAlign.center,
              style: const TextStyle(fontFamily: 'monospace')),
          const SizedBox(height: 8),
          const Text(
              'Check that this ID and fingerprint match the ones shown on the computer you are linking. '
              'Only approve a computer you own: linked computers can ask to control this one.',
              textAlign: TextAlign.center,
              style: TextStyle(color: Colors.grey)),
        ]);
        actions = [
          dialogButton('DENY', onPressed: () => decide(false), isOutline: true),
          dialogButton('APPROVE', onPressed: () => decide(true)),
        ];
        break;
      case 'approved':
        final req = (status['requester'] as Map?) ?? {};
        body = Text('Linked with ${req['name']}.',
            style: const TextStyle(color: Colors.green, fontSize: 16));
        actions = [dialogButton('Done', onPressed: finish)];
        break;
      case 'denied':
      case 'expired':
      case 'locked':
        body = Text(
            state == 'denied'
                ? 'The link was not approved.'
                : state == 'expired'
                    ? 'This code has expired.'
                    : 'Too many wrong codes were tried, so this code was cancelled.',
            textAlign: TextAlign.center);
        actions = [
          dialogButton('Close', onPressed: finish, isOutline: true),
          dialogButton('New code', onPressed: newCode),
        ];
        break;
      case 'error':
        body = Text(error, style: const TextStyle(color: Colors.red));
        actions = [
          dialogButton('Close', onPressed: finish, isOutline: true),
          dialogButton('Try again', onPressed: newCode),
        ];
        break;
      default:
        body = const Padding(
            padding: EdgeInsets.all(20), child: CircularProgressIndicator());
        actions = [dialogButton('Cancel', onPressed: finish, isOutline: true)];
    }
    return CustomAlertDialog(
      title: const Text("This computer's code"),
      content: SizedBox(
          width: 380,
          child: Column(mainAxisSize: MainAxisSize.min, children: [
            body,
            if (error.isNotEmpty && state != 'error')
              Text(error, style: const TextStyle(color: Colors.red)),
          ])),
      actions: actions,
      onCancel: finish,
    );
  });
}

void showDeviceMenu(BuildContext context, OffloadDevice d) {
  final name = TextEditingController(text: d.name);
  gFFI.dialogManager.show((setState, close, context) {
    return CustomAlertDialog(
      title: Text(d.name),
      content: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            TextField(
                controller: name,
                decoration: const InputDecoration(labelText: 'Name')),
            const SizedBox(height: 12),
            Text('${d.platform} · ID ${d.id}',
                style: const TextStyle(color: Colors.grey)),
            const SizedBox(height: 16),
            TextButton.icon(
              icon: const Icon(Icons.link_off, color: Colors.red),
              label: const Text('Remove this computer',
                  style: TextStyle(color: Colors.red)),
              onPressed: () async {
                close();
                final r = await offloadModel.remove(d.id);
                showToast(r.ok ? 'Removed ${d.name}' : r.error);
              },
            ),
          ]),
      actions: [
        dialogButton('Cancel', onPressed: close, isOutline: true),
        dialogButton('Save', onPressed: () async {
          close();
          if (name.text.trim().isNotEmpty && name.text.trim() != d.name) {
            await offloadModel.rename(d.id, name.text.trim());
          }
        }),
      ],
      onCancel: close,
    );
  });
}

String _quotePosix(String s) => "'${s.replaceAll("'", "'\\''")}'";
String _quotePowerShell(String s) => "'${s.replaceAll("'", "''")}'";

/// Builds the command typed into the remote terminal to start a tool in a project folder.
String offloadProjectCommand(OffloadDevice d, String folder, String tool) {
  if (folder.trim().isEmpty) return tool;
  return d.isWindows
      ? 'Set-Location -LiteralPath ${_quotePowerShell(folder.trim())}; $tool'
      : 'cd ${_quotePosix(folder.trim())} && $tool';
}

String offloadClaudeInstallCommand(OffloadDevice d) => d.isWindows
    ? 'irm https://claude.ai/install.ps1 | iex'
    : 'curl -fsSL https://claude.ai/install.sh | bash';

/// Opens a terminal on `d` and types `command` once it is ready.
Future<void> openRemoteTerminal(
    BuildContext context, OffloadDevice d, String command) async {
  await bind.mainSetLocalOption(
      key: 'offload-terminal-command-${d.id}', value: command);
  if (context.mounted) await connect(context, d.id, isTerminal: true);
}

void showRemoteComputingDialog(BuildContext context, OffloadDevice d) {
  final folderKey = 'offload-project-${d.id}';
  final folder = TextEditingController(text: bind.mainGetLocalOption(key: folderKey));
  gFFI.dialogManager.show((setState, close, context) {
    void run(String tool) {
      bind.mainSetLocalOption(key: folderKey, value: folder.text.trim());
      close();
      openRemoteTerminal(context, d, offloadProjectCommand(d, folder.text, tool));
    }

    return CustomAlertDialog(
      title: Text('Remote computing on ${d.name}'),
      content: SizedBox(
        width: 460,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
                'Commands run on ${d.name}, using its processor, memory and files. This computer shows the results.',
                style: const TextStyle(color: Colors.grey)),
            const SizedBox(height: 14),
            TextField(
              controller: folder,
              decoration: InputDecoration(
                  labelText: 'Project folder on ${d.name}',
                  hintText: d.isWindows ? r'C:\Users\you\projects\app' : '~/projects/app'),
            ),
            const SizedBox(height: 18),
            _ComputeAction(
                icon: Icons.terminal,
                title: 'Open a terminal',
                subtitle: 'A shell on the remote computer, in the project folder.',
                onTap: () => run('')),
            _ComputeAction(
                icon: Icons.smart_toy_outlined,
                title: 'Run Claude Code',
                subtitle: 'Starts `claude` in the project folder on the remote computer.',
                onTap: () => run('claude')),
            _ComputeAction(
                icon: Icons.download_outlined,
                title: 'Install Claude Code on ${d.name}',
                subtitle: offloadClaudeInstallCommand(d),
                onTap: () {
                  close();
                  openRemoteTerminal(context, d, offloadClaudeInstallCommand(d));
                }),
            _ComputeAction(
                icon: Icons.folder_copy_outlined,
                title: 'Transfer project files',
                subtitle: 'Copy files between this computer and the project.',
                onTap: () {
                  close();
                  connect(context, d.id, isFileTransfer: true);
                }),
          ],
        ),
      ),
      actions: [dialogButton('Close', onPressed: close, isOutline: true)],
      onCancel: close,
    );
  });
}

class _ComputeAction extends StatelessWidget {
  final IconData icon;
  final String title;
  final String subtitle;
  final VoidCallback onTap;
  const _ComputeAction(
      {required this.icon,
      required this.title,
      required this.subtitle,
      required this.onTap});

  @override
  Widget build(BuildContext context) {
    return ListTile(
      contentPadding: EdgeInsets.zero,
      leading: Icon(icon, color: _accent),
      title: Text(title),
      subtitle: Text(subtitle, maxLines: 2, overflow: TextOverflow.ellipsis),
      onTap: onTap,
    );
  }
}
