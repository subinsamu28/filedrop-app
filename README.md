# filedrop-app
📤 FileDrop  Local File Sharing Over LAN  A fast, secure, and cross-platform desktop app to send and receive files over your local network. Built in Rust with a lightweight native GUI. No internet, no cloud, no setup  just drop and share.


<h1 align="center">📤 FileDrop</h1>
<p align="center">
  <strong>Blazing fast, privacy-first file sharing over your local network.</strong><br>
  No cloud. No setup. Just drop and send.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/built%20with-rust-orange?style=for-the-badge&logo=rust&logoColor=white"/>
  <img src="https://img.shields.io/badge/platform-desktop-blueviolet?style=for-the-badge"/>
  <img src="https://img.shields.io/badge/license-MIT-green?style=for-the-badge"/>
</p>

---

## ⚡ Overview

**FileDrop** is a modern desktop app that lets you send and receive files instantly across devices on the same local network — without needing internet, third-party servers, or configuration.

Designed  privacy, built with performance in mind, and crafted in Rust for maximum reliability.

---

## 🧠 Why FileDrop?

- ✅ **Fast** – transfers large files in seconds using direct TCP
- 🔒 **Private** – your files never leave your network
- 🖥️ **Cross-platform** – built in Rust, runs on Windows, macOS, Linux
- 🌐 **Zero config** – just launch, select, and send
- 🧊 **Lightweight** – no Electron bloat, native GUI with `egui`
- 💾 **Offline** – works perfectly without an internet connection

---

## 📸 Interface Preview

> *(Coming soon – screenshots of GUI sender/receiver panels here)*

---

## 🚀 Features

| Feature               | Status    |
|-----------------------|-----------|
| 🖱️ Native file picker  | ✅ Done    |
| 📤 Sender dashboard    | ✅ Done    |
| 📥 Receiver dashboard  | ✅ Done    |
| 📦 Chunked file transfer | ✅ Done |
| 🔐 Offline, P2P only    | ✅ Done    |
| 🌍 mDNS peer discovery  | 🔜 Planned |
| 🧩 Drag and drop UI     | 🔜 Planned |
| 🔁 Resume interrupted transfer | 🔜 Planned |

---

## 🛠️ Built With

| Technology | Purpose |
|------------|---------|
| **Rust** | Core logic, networking, threading |
| **Tokio** | Asynchronous runtime |
| **Egui + Eframe** | Native GUI |
| **Serde** | JSON metadata encoding |
| **RFD** | Native file dialogs |
| **Cargo** | Build & dependency management |

---

## 📦 Installation

### 🧰 Prerequisites

- Rust 1.70 or higher (install via [rustup.rs](https://rustup.rs))

### 🧪 Build & Run

```bash
git clone https://github.com/yourusername/filedrop.git
cd filedrop
cargo run
```

---

## 🧪 Usage

### 📤 Send a File

1. Launch the app
2. Switch to the **Send** tab
3. Select a file using the native picker
4. Enter the IP address of the receiving device
5. Hit **Send**

### 📥 Receive a File

1. Launch the app
2. Switch to the **Receive** tab
3. Click **Start Receiver**
4. Incoming files will be saved to:

```bash
~/Downloads/filedrop/
```

---

## 🧩 CLI Mode (Optional)

> Prefer terminal? You can use FileDrop from the command line too.

```bash
cargo run -- send 192.168.1.42 ./example.txt
cargo run -- receive
```

---

## 🛡️ Privacy & Security

- No data ever leaves your network
- No third-party servers or cloud services
- Everything is direct peer-to-peer via TCP
- Optional file hash verification (coming soon)
- Planned: trusted-device mode, whitelist, encryption

---

## 🧭 Roadmap

- [ ] Transfer progress indicators in GUI
- [ ] Auto device discovery via mDNS
- [ ] Resume failed transfers
- [ ] Drag & drop support
- [ ] Mobile companion app (Flutter/Rust bridge)

---

## 🙋‍♂️ Author

**Subin Samu**  
_MSc Applied Computer Science | Cyber Forensics | Systems Programmer_

- 🌐 [LinkedIn](https://linkedin.com/in/yourprofile)
- 🧑‍💻 [GitHub](https://github.com/yourusername)

---

## 🪪 License

This project is licensed under the [MIT License](LICENSE).

---

> ⚡ **Built in Rust. Fast by default. Trusted by design.**
