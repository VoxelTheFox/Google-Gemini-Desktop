# Google Gemini Desktop

### **Being honest with you all, this app is just pure vibecoded but I made it since there was no good app like this. The one I was looking at using before I made this turned out to have MALWARE and then was taken down**
## **Here is a Read Me to give you this gist of this app**

A lightweight, cross-platform desktop wrapper for the Google Gemini website built with **Tauri v2**.

This app runs as a background utility (like Spotlight or Alfred), allowing you to toggle Gemini instantly with a global hotkey without cluttering your Dock or Taskbar.

## ✨ Features

* **⚡️ Instant Access:** Toggle the window globally from anywhere.
  * **macOS:** `Command` + `G`
  * **Windows:** `Alt` + `Space`


* **🫥 Out of the Way:** Runs purely in the System Tray / Menu Bar.
  * Hidden from the macOS Dock and App Switcher (`Cmd+Tab`).
  * Hidden from the Windows Taskbar.


* **🎨 Native Feel:**
  * **macOS:** Supports native "Template" menu bar icons (auto-adjusts to Light/Dark mode).
  * **Windows:** Uses standard colored tray icons.
  * **UI:** Frameless window with rounded corners and drop shadows.


* **🖱️ Custom Dragging:** Custom implementation to allow dragging near the top edge of the window.
* **🚀 Auto-Start:** Integrated "Run on Startup" option accessible directly from the tray icon's menu/menu bar icon's menu.

## 🛠️ Installation & Development

### Prerequisites

* **Rust:** [Install Rust](https://www.rust-lang.org/tools/install) (`cargo`).
* **Node.js:** Installed.
* **Build Tools:**
* *Windows:* C++ Build Tools (via Visual Studio Installer).
* *macOS:* Xcode Command Line Tools (`xcode-select --install`).



### Running Locally

1. **Clone the repository:**
```bash
git clone https://github.com/VoxelTheFox/Google-Gemini-Desktop.git
cd Google-Gemini-Desktop

```


2. **Install dependencies:**
```bash
npm install

```


3. **Run in Development Mode:**
```bash
npm run tauri dev

```


*Note: On macOS, you may need to approve accessibility permissions if prompted for the global hotkey.*

### Building the App

To create the final executable (`.exe` for Windows, `.dmg` or `.app` for macOS):

```bash
npm run tauri build

```

The output will be in `src-tauri/target/release/bundle/`.


## 📄 License

MIT
