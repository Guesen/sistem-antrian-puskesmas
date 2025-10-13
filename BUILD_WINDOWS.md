# Build Aplikasi untuk Windows

## Cara 1: Build di Komputer Windows (Recommended)

### Prerequisites:

1. **Install Rust**

   ```powershell
   # Download dari https://rustup.rs/
   # Atau jalankan:
   winget install --id Rustlang.Rustup
   ```

2. **Install Node.js**

   ```powershell
   # Download dari https://nodejs.org/
   # Atau jalankan:
   winget install --id OpenJS.NodeJS
   ```

3. **Install Visual Studio Build Tools**
   ```powershell
   # Download dari https://visualstudio.microsoft.com/downloads/
   # Pilih "Desktop development with C++"
   ```

### Build Steps:

```powershell
# 1. Clone atau copy project ke Windows
cd "path\to\Sistem Antrian Puskesmas"

# 2. Install dependencies
npm install

# 3. Build aplikasi
npm run build:desktop

# 4. File installer akan ada di:
# src-tauri\target\release\bundle\msi\Sistem Antrian Puskesmas_1.0.0_x64_en-US.msi
```

## Cara 2: Cross-Compilation dari Mac (Advanced)

### Prerequisites:

```bash
# Install mingw-w64
brew install mingw-w64

# Add Windows target
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-pc-windows-msvc
```

### Configure Cargo:

Create or edit `~/.cargo/config.toml`:

```toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
ar = "x86_64-w64-mingw32-ar"

[target.x86_64-pc-windows-msvc]
linker = "rust-lld"
```

### Build:

```bash
cd "/Users/adivadg/Documents/Progamming/Sistem Antrian Puskesmas"

# Build untuk Windows
npx tauri build --target x86_64-pc-windows-gnu
```

**Note:** Cross-compilation dari Mac ke Windows sering bermasalah. Cara terbaik adalah build di komputer Windows.

## Cara 3: Menggunakan GitHub Actions (Otomatis)

Buat file `.github/workflows/build.yml`:

```yaml
name: Build Apps

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  build:
    strategy:
      matrix:
        os: [macos-latest, windows-latest]

    runs-on: ${{ matrix.os }}

    steps:
      - uses: actions/checkout@v3

      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: "18"

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install dependencies (Ubuntu)
        if: matrix.os == 'ubuntu-latest'
        run: |
          sudo apt-get update
          sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf

      - name: Install dependencies
        run: npm install

      - name: Build
        run: npm run build:desktop

      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: app-${{ matrix.os }}
          path: |
            src-tauri/target/release/bundle/
```

## File Output

### Windows:

- **MSI Installer**: `src-tauri\target\release\bundle\msi\Sistem Antrian Puskesmas_1.0.0_x64_en-US.msi`
- **NSIS Installer**: `src-tauri\target\release\bundle\nsis\Sistem Antrian Puskesmas_1.0.0_x64-setup.exe`

### Mac:

- **DMG**: `src-tauri/target/release/bundle/dmg/Sistem Antrian Puskesmas_1.0.0_aarch64.dmg`
- **App**: `src-tauri/target/release/bundle/macos/Sistem Antrian Puskesmas.app`

## Troubleshooting

### Error: "link.exe not found"

**Solution:** Install Visual Studio Build Tools dengan "Desktop development with C++"

### Error: "VCRUNTIME140.dll not found"

**Solution:** Install Visual C++ Redistributable dari Microsoft

### Error: "Failed to bundle project"

**Solution:**

1. Pastikan Rust dan Node.js terinstall
2. Jalankan `rustup update`
3. Hapus folder `node_modules` dan `src-tauri/target`
4. Jalankan `npm install` lagi

## Testing

Setelah build selesai, test aplikasi:

```powershell
# Windows
.\src-tauri\target\release\app.exe

# Atau install MSI dan jalankan dari Start Menu
```

## Distribution

1. Upload file MSI ke GitHub Releases
2. Atau host di server sendiri
3. User tinggal download dan install
4. Aplikasi akan berjalan offline tanpa perlu internet

## Notes

- Build Windows dari Mac sangat kompleks dan sering error
- Recommended: Gunakan komputer Windows atau GitHub Actions
- File MSI adalah installer standar Windows yang mudah didistribusikan
