// Offload settings: trusted devices, unattended access, quality, privacy, startup, about.
// Everything else lives under Advanced, which opens RustDesk's full settings.

import 'package:flutter/material.dart';
import 'package:flutter_hbb/common.dart';
import 'package:flutter_hbb/consts.dart';
import 'package:flutter_hbb/desktop/pages/desktop_tab_page.dart';
import 'package:flutter_hbb/models/offload_model.dart';
import 'package:flutter_hbb/models/platform_model.dart';
import 'package:provider/provider.dart';
import 'package:url_launcher/url_launcher.dart';

const kOffloadSourceUrl = String.fromEnvironment('OFFLOAD_SOURCE_URL',
    defaultValue: 'https://github.com/rustdesk/rustdesk');

class OffloadSettingsPage extends StatefulWidget {
  const OffloadSettingsPage({Key? key}) : super(key: key);

  @override
  State<OffloadSettingsPage> createState() => _OffloadSettingsPageState();
}

class _OffloadSettingsPageState extends State<OffloadSettingsPage> {
  String _quality = bind.mainGetUserDefaultOption(key: kOptionImageQuality);
  String _version = '';
  String _fingerprint = '';

  @override
  void initState() {
    super.initState();
    offloadModel.attach();
    () async {
      final v = await bind.mainGetVersion();
      final f = await bind.mainGetFingerprint();
      if (mounted) {
        setState(() {
          _version = v;
          _fingerprint = f;
        });
      }
    }();
  }

  @override
  void dispose() {
    offloadModel.detach();
    super.dispose();
  }

  Widget _section(String title, List<Widget> children) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 22),
      child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
        Text(title,
            style: const TextStyle(fontSize: 16, fontWeight: FontWeight.w700)),
        const SizedBox(height: 8),
        Material(
          color: Theme.of(context).cardColor,
          borderRadius: BorderRadius.circular(12),
          child: Padding(
            padding: const EdgeInsets.symmetric(vertical: 6),
            child: Column(children: children),
          ),
        ),
      ]),
    );
  }

  Future<bool> _confirm(String title, String text, String action) async {
    bool result = false;
    await gFFI.dialogManager.show<bool>((setState, close, context) {
      return CustomAlertDialog(
        title: Text(title),
        content: Text(text),
        actions: [
          dialogButton('Cancel', onPressed: close, isOutline: true),
          dialogButton(action, onPressed: () {
            result = true;
            close();
          }),
        ],
        onCancel: close,
      );
    });
    return result;
  }

  Future<void> _setUnattended(bool on) async {
    if (on) {
      final yes = await _confirm(
          'Allow unattended access?',
          'Your linked computers will be able to connect to this computer without anyone here approving each session. '
              'Only turn this on for computers you own.',
          'Allow');
      if (!yes) return;
      // The operating system confirms the change (administrator password, Touch ID or UAC).
      if (!await callMainCheckSuperUserPermission()) return;
    }
    final r = await offloadModel.setUnattended(on);
    if (!r.ok) showToast(r.error);
  }

  Future<void> _removeAll() async {
    final yes = await _confirm(
        'Remove all linked computers?',
        'Every linked computer will be disconnected and will need a new code to link again.',
        'Remove all');
    if (!yes) return;
    if (!await callMainCheckSuperUserPermission()) return;
    final r = await offloadModel.removeAll();
    showToast(r.ok ? 'All linked computers removed' : r.error);
  }

  Widget _qualityTile(String value, String title, String subtitle) {
    return RadioListTile<String>(
      value: value,
      groupValue: _quality.isEmpty ? kRemoteImageQualityBalanced : _quality,
      title: Text(title),
      subtitle: Text(subtitle),
      onChanged: (v) async {
        if (v == null) return;
        await bind.mainSetUserDefaultOption(key: kOptionImageQuality, value: v);
        setState(() => _quality = v);
      },
    );
  }

  Widget _permissionSwitch(String key, String title, String subtitle) {
    final value = mainGetBoolOptionSync(key);
    return SwitchListTile(
      value: value,
      title: Text(title),
      subtitle: Text(subtitle),
      onChanged: (v) async {
        await mainSetBoolOption(key, v);
        setState(() {});
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    return ChangeNotifierProvider.value(
      value: offloadModel,
      child: Consumer<OffloadModel>(builder: (context, model, _) {
        return ListView(
          padding: const EdgeInsets.fromLTRB(28, 24, 28, 24),
          children: [
            const Text('Settings',
                style: TextStyle(fontSize: 24, fontWeight: FontWeight.w800)),
            const SizedBox(height: 18),
            _section('Trusted devices', [
              if (model.devices.isEmpty)
                const ListTile(title: Text('No computers are linked yet.')),
              for (final d in model.devices)
                ListTile(
                  leading: Icon(d.isWindows
                      ? Icons.desktop_windows_outlined
                      : (d.isMac ? Icons.laptop_mac : Icons.computer)),
                  title: Text(d.name),
                  subtitle: Text('${d.platform} · ID ${d.id}'),
                  trailing: TextButton(
                    child: const Text('Remove', style: TextStyle(color: Colors.red)),
                    onPressed: () async {
                      final yes = await _confirm('Remove ${d.name}?',
                          '${d.name} will be disconnected and will need a new code to link again.',
                          'Remove');
                      if (!yes) return;
                      final r = await model.remove(d.id);
                      showToast(r.ok ? 'Removed ${d.name}' : r.error);
                    },
                  ),
                ),
              if (model.devices.isNotEmpty)
                ListTile(
                  leading: const Icon(Icons.delete_sweep_outlined, color: Colors.red),
                  title: const Text('Remove all trusted devices',
                      style: TextStyle(color: Colors.red)),
                  onTap: _removeAll,
                ),
            ]),
            _section('Unattended access', [
              SwitchListTile(
                value: model.unattended,
                title: const Text('Allow my trusted devices to connect when I am away'),
                subtitle: const Text(
                    'Off: someone at this computer approves every session. On: your linked computers connect directly.'),
                onChanged: _setUnattended,
              ),
              ListTile(
                leading: const Icon(Icons.power_settings_new),
                title: const Text('Disconnect everyone now'),
                subtitle: const Text('Ends every remote session on this computer.'),
                onTap: () async {
                  final r = await model.disconnectAll();
                  showToast(r.ok
                      ? 'Disconnected ${r.data['closed'] ?? 0} session(s)'
                      : r.error);
                },
              ),
            ]),
            _section('Streaming quality', [
              _qualityTile(kRemoteImageQualityLow, 'FAST',
                  'Lowest delay. Best on slow or mobile connections.'),
              _qualityTile(kRemoteImageQualityBalanced, 'BALANCED',
                  'Good picture and responsive. Recommended.'),
              _qualityTile(kRemoteImageQualityBest, 'HIGH QUALITY',
                  'Sharpest picture. Needs a fast connection.'),
            ]),
            _section('When others connect to this computer', [
              _permissionSwitch(kOptionEnableKeyboard, 'Allow control',
                  'Linked computers can use the keyboard and mouse.'),
              _permissionSwitch(kOptionEnableFileTransfer, 'Allow file transfer',
                  'Linked computers can copy files to and from this computer. Existing files are never replaced without asking.'),
              _permissionSwitch(kOptionEnableClipboard, 'Share clipboard',
                  'Copy on one computer, paste on the other.'),
              _permissionSwitch(kOptionEnableAudio, 'Send sound',
                  'Play this computer\'s sound on the connected computer.'),
            ]),
            _section('Privacy and security', [
              ListTile(
                title: const Text('This computer'),
                subtitle: SelectableText(
                    'ID ${model.myId}\nIdentity fingerprint $_fingerprint'),
              ),
              const ListTile(
                title: Text('How Offload protects you'),
                subtitle: Text(
                    'Sessions and transfers are end-to-end encrypted. Only computers you linked with a one-time code can connect, '
                    'and each one proves its identity with its own key every time.'),
              ),
            ]),
            _section('Startup', [
              ListTile(
                title: const Text('Start automatically'),
                subtitle: Text(isWindows
                    ? 'Once installed, Offload runs in the background when Windows starts so your linked computers can reach it.'
                    : 'Offload starts its background helper when you log in, after you grant Screen Recording and Accessibility.'),
              ),
            ]),
            _section('Advanced', [
              ListTile(
                leading: const Icon(Icons.tune),
                title: const Text('Advanced settings'),
                subtitle: const Text('Network, display, codec and other technical options.'),
                onTap: onAddAdvancedSetting,
              ),
            ]),
            _section('About', [
              ListTile(
                title: Text('Offload $_version'),
                subtitle: const Text(
                    'Free software under the GNU AGPL v3. Based on RustDesk by Purslane Ltd. and contributors.'),
              ),
              ListTile(
                leading: const Icon(Icons.code),
                title: const Text('Source code'),
                subtitle: const Text(kOffloadSourceUrl),
                onTap: () => launchUrl(Uri.parse(kOffloadSourceUrl)),
              ),
            ]),
          ],
        );
      }),
    );
  }
}
