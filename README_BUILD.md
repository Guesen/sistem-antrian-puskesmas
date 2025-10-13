# Panduan Build Aplikasi Sistem Antrian Puskesmas

## 📦 File Installer yang Tersedia

### Mac (Sudah Tersedia)

- **DMG**: `src-tauri/target/release/bundle/dmg/Sistem Antrian Puskesmas_1.0.0_aarch64.dmg`
- **App**: `src-tauri/target/release/bundle/macos/Sistem Antrian Puskesmas.app`

### Windows (Perlu Build)

- **MSI**: `src-tauri\target\release\bundle\msi\Sistem Antrian Puskesmas_1.0.0_x64_en-US.msi`
- **NSIS**: `src-tauri\target\release\bundle\nsis\Sistem Antrian Puskesmas_1.0.0_x64-setup.exe`

---

## 🚀 Cara 1: Build Windows Menggunakan GitHub Actions (RECOMMENDED)

Ini adalah cara paling mudah untuk build aplikasi Windows tanpa perlu komputer Windows.

### Langkah-langkah:

#### 1. Push ke GitHub

```bash
cd "/Users/adivadg/Documents/Progamming/Sistem Antrian Puskesmas"

# Initialize git jika belum
git init

# Add semua file
git add .

# Commit
git commit -m "Initial commit - Sistem Antrian Puskesmas"

# Create repository di GitHub (buat dulu di github.com)
# Lalu connect:
git remote add origin https://github.com/USERNAME/sistem-antrian-puskesmas.git
git branch -M main
git push -u origin main
```

#### 2. GitHub Actions Akan Otomatis Build

Setelah push, GitHub Actions akan otomatis:

- ✅ Build untuk Windows (MSI & NSIS)
- ✅ Build untuk Mac (Intel & Apple Silicon)
- ✅ Upload artifacts ke GitHub

#### 3. Download Installer

Setelah build selesai (sekitar 10-15 menit):

1. Buka repository di GitHub
2. Klik tab **Actions**
3. Klik workflow run terbaru
4. Scroll ke bawah ke bagian **Artifacts**
5. Download:
   - `windows-installer` (untuk Windows)
   - `macos-aarch64-installer` (untuk Mac M1/M2/M3)
   - `macos-x86_64-installer` (untuk Mac Intel)

#### 4. Atau Buat Release Tag (Optional)

Untuk membuat release dengan download link permanent:

```bash
# Create tag
git tag v1.0.0
git push origin v1.0.0

# GitHub Actions akan otomatis:
# 1. Build semua platform
# 2. Create Release di GitHub
# 3. Upload installer sebagai release assets
```

Installer akan tersedia di: `https://github.com/USERNAME/sistem-antrian-puskesmas/releases`

---

## 💻 Cara 2: Build di Komputer Windows

### Prerequisites:

#### 1. Install Rust

```powershell
# Download dan install dari: https://rustup.rs/
# Atau gunakan winget:
winget install --id Rustlang.Rustup
```

#### 2. Install Node.js

```powershell
# Download dari: https://nodejs.org/
# Atau gunakan winget:
winget install --id OpenJS.NodeJS
```

#### 3. Install Visual Studio Build Tools

```powershell
# Download dari: https://visualstudio.microsoft.com/downloads/
# Pilih "Desktop development with C++"
# Atau gunakan winget:
winget install --id Microsoft.VisualStudio.2022.BuildTools
```

### Build Steps:

```powershell
# 1. Clone atau copy project ke Windows
cd "C:\Users\YourName\Documents"
# Copy folder "Sistem Antrian Puskesmas" ke sini

# 2. Masuk ke folder project
cd "Sistem Antrian Puskesmas"

# 3. Install dependencies
npm install

# 4. Build aplikasi
npm run build:desktop

# 5. Installer akan ada di:
# - MSI: src-tauri\target\release\bundle\msi\Sistem Antrian Puskesmas_1.0.0_x64_en-US.msi
# - NSIS: src-tauri\target\release\bundle\nsis\Sistem Antrian Puskesmas_1.0.0_x64-setup.exe
```

---

## 🔄 Cara 3: Cross-Compilation dari Mac (ADVANCED - Tidak Recommended)

**Warning:** Cross-compilation dari Mac ke Windows sangat kompleks dan sering gagal.

### Prerequisites:

```bash
# Install mingw-w64
brew install mingw-w64

# Add Windows target
rustup target add x86_64-pc-windows-gnu
```

### Configure:

Create `~/.cargo/config.toml`:

```toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
ar = "x86_64-w64-mingw32-ar"
```

### Build:

```bash
cd "/Users/adivadg/Documents/Progamming/Sistem Antrian Puskesmas"
npx tauri build --target x86_64-pc-windows-gnu
```

**Note:** Cara ini sering gagal karena dependency Windows yang tidak tersedia di Mac.

---

## 📋 Troubleshooting

### Windows Build Issues:

#### Error: "link.exe not found"

**Solution:**

- Install Visual Studio Build Tools
- Restart terminal
- Jalankan dari "Developer Command Prompt for VS"

#### Error: "VCRUNTIME140.dll not found"

**Solution:**

- Download dan install Visual C++ Redistributable
- https://aka.ms/vs/17/release/vc_redist.x64.exe

#### Error: "Failed to bundle project"

**Solution:**

```powershell
# 1. Update Rust
rustup update

# 2. Clean build
Remove-Item -Recurse -Force node_modules
Remove-Item -Recurse -Force src-tauri\target

# 3. Reinstall
npm install
npm run build:desktop
```

### GitHub Actions Issues:

#### Actions tidak berjalan

**Solution:**

1. Pastikan GitHub Actions diaktifkan di repository settings
2. Push ke branch `main` atau `master`
3. Atau create tag dengan `git tag v1.0.0 && git push origin v1.0.0`

#### Build gagal

**Solution:**

1. Check logs di tab Actions
2. Pastikan semua dependencies di package.json benar
3. Pastikan src-tauri/Cargo.toml tidak ada error

---

## 🎯 Distribusi Installer

### Windows:

1. Share file `.msi` atau `.exe` ke user
2. User double-click untuk install
3. Aplikasi akan muncul di Start Menu
4. Jalankan dari Start Menu

### Mac:

1. Share file `.dmg` ke user
2. User double-click DMG
3. Drag aplikasi ke Applications folder
4. Jalankan dari Applications atau Launchpad

---

## 📝 Catatan Penting

### Untuk GitHub Actions:

- ✅ **Kelebihan:**

  - Build otomatis untuk semua platform
  - Tidak perlu komputer Windows
  - Gratis untuk public repository
  - Professional dan reliable

- ❌ **Kekurangan:**
  - Perlu account GitHub
  - Build time 10-15 menit
  - Perlu koneksi internet

### Untuk Build Lokal di Windows:

- ✅ **Kelebihan:**

  - Full control
  - Lebih cepat jika sudah setup
  - Tidak perlu internet untuk build

- ❌ **Kekurangan:**
  - Perlu komputer Windows
  - Setup environment lebih kompleks
  - Troubleshooting lebih sulit

---

## 🔐 Code Signing (Optional)

Untuk production release, sebaiknya sign installer:

### Windows:

```powershell
# Generate certificate
New-SelfSignedCertificate -Type CodeSigningCert -Subject "CN=Puskesmas Mrebet"

# Sign installer
signtool sign /f certificate.pfx /p password installer.msi
```

### Mac:

```bash
# Perlu Apple Developer Account ($99/year)
# Sign dengan Developer ID
codesign --force --sign "Developer ID Application: Your Name" app.app
```

---

## 📞 Support

Jika ada masalah saat build:

1. Check error message di terminal/logs
2. Lihat troubleshooting section di atas
3. Google error message spesifik
4. Check Tauri documentation: https://tauri.app/

---

## 🎉 Quick Start Recommendations

**Paling Mudah:** Gunakan GitHub Actions

- Push ke GitHub
- Download installer dari Actions artifacts
- Selesai!

**Paling Cepat (jika punya Windows):** Build lokal di Windows

- Install prerequisites
- Run `npm run build:desktop`
- Selesai!

**Untuk Production:** Gunakan GitHub Actions + Code Signing

- Setup GitHub repository
- Add signing certificates
- Create release tags
- Professional dan reliable!
