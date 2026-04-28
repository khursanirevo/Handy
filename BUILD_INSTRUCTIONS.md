# Build & Install Handy on macOS

This fork includes SherpaOnnx Malaysian language transcription support on top of the upstream Handy app.

## Prerequisites

1. **Xcode Command Line Tools**
   ```bash
   xcode-select --install
   ```

2. **Rust** (via rustup)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source "$HOME/.cargo/env"
   ```

3. **Node.js** (v18+)
   ```bash
   # Using nvm (recommended)
   curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash
   nvm install --lts
   ```
   Or install directly from https://nodejs.org

## Build Steps

### 1. Clone
```bash
git clone https://github.com/khursanirevo/Handy.git
cd Handy
```

### 2. Install frontend dependencies
```bash
npm install
```

### 3. Build for production
```bash
npm run tauri build
```

This compiles the Rust backend and React frontend, then generates a macOS `.dmg` bundle.

The first build takes a while (10-20 min) since it compiles all Rust dependencies including sherpa-rs. Subsequent builds are much faster.

### 4. Install

The built app is at:
```
src-tauri/target/release/bundle/dmg/Handy_0.8.3_aarch64.dmg
```

Double-click the DMG, drag Handy to your Applications folder.

## Dev Mode (optional)

To run the app in dev mode with hot-reload:

```bash
npm run tauri dev
```

## Troubleshooting

- **"cargo not found"**: Make sure Rust is installed and restart your terminal
- **Build fails with sherpa-rs errors**: Make sure Xcode Command Line Tools are installed (`xcode-select --install`)
- **Accessibility permission**: On first launch, macOS will prompt for Accessibility and Microphone permissions. Grant both for the app to work correctly.
