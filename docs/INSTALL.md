# Installing and using Offload

## Windows 11

1. Run **Offload-Setup.exe** and choose **Install**. Windows asks for permission once
   (the background service needs administrator rights to show the login screen and approve
   prompts remotely).
2. Offload starts and appears in the Start menu and the system tray.

If the installer is unsigned, Windows SmartScreen says "Windows protected your PC": click
**More info → Run anyway**. Signed releases do not show this.

Uninstall from **Settings → Apps → Installed apps → Offload → Uninstall**. This removes the
service, shortcuts and program files.

## macOS (Apple Silicon)

1. Open **Offload.dmg** and drag **Offload** to **Applications**.
2. Open Offload. If macOS says it cannot verify the developer (unsigned builds), open
   **System Settings → Privacy & Security** and click **Open Anyway**. Notarized releases open
   normally.
3. To let your other computers control this Mac, Offload asks you to allow
   **Screen Recording** and **Accessibility** (and optionally **Microphone** for sound) in
   System Settings, and to install its background helper. macOS requires you to grant these
   yourself; Offload cannot and does not bypass them.

To remove: quit Offload, delete it from Applications. The helper is removed by
**Offload → Settings → Advanced → Uninstall** (or the uninstall entry in the app menu).

## Link two computers (once)

1. On the computer you want to control (for example the Windows PC), open Offload and choose
   **THIS COMPUTER'S CODE**. A code like `K7QM-4ZP9` appears. It works once and expires in five
   minutes.
2. On the other computer (the Mac), choose **LINK A COMPUTER** and type the code.
3. Back on the first computer, Offload asks: *"MacBook Pro wants to link with this computer."*
   Click **APPROVE**.

Both computers now list each other under **My Computers**. You never need the code again.

## Everyday use

Select a computer under **My Computers**, then:

* **CONNECT** — see and control its screen.
* **TRANSFER FILES** — your files on the left, the other computer's on the right. Drag, or
  select and use the arrows. Offload asks before replacing a file.
* **REMOTE COMPUTING** — open a terminal on the other computer, start Claude Code in a project
  folder there, or install Claude Code on it. The other computer does the work; this one shows
  the results. Terminals keep running if the connection drops; reconnect to pick them up.

Unless you allowed unattended access on the computer you connect to, someone there clicks
**Accept** for each session.

## Using a computer when nobody is there

On that computer: **Settings → Unattended access → Allow my trusted devices to connect when I
am away**. Offload asks you to confirm and then asks for your computer password. From then on
your linked computers connect without anyone accepting. It only applies to computers you
linked; turn it off at any time.

## Security controls

* **Settings → Trusted devices → Remove** — that computer is disconnected now and can never
  connect again unless you link it again.
* **Remove all trusted devices** — removes every link and turns unattended access off.
* **Disconnect everyone now** — ends every session immediately.
* While someone is connected, Offload's connection window is shown on that computer.
