# 🚀 Quick Start: Build Aplikasi Windows

## Cara Tercepat: GitHub Actions (5 Menit Setup)

### 1. Buat Repository di GitHub

1. Buka https://github.com/new
2. Nama repository: `sistem-antrian-puskesmas`
3. Buat repository (public atau private)

### 2. Push Kode ke GitHub

```bash
cd "/Users/adivadg/Documents/Progamming/Sistem Antrian Puskesmas"

# Initialize git
git init

# Add all files
git add .

# Commit
git commit -m "Initial commit"

# Add remote (ganti USERNAME dengan username GitHub Anda)
git remote add origin https://github.com/USERNAME/sistem-antrian-puskesmas.git

# Push
git branch -M main
git push -u origin main
```

### 3. Buat Release

```bash
# Jalankan script release
./release.sh 1.0.0
```

Atau manual:

```bash
git tag v1.0.0
git push origin v1.0.0
```

### 4. Download Installer

1. Buka: `https://github.com/USERNAME/sistem-antrian-puskesmas/actions`
2. Tunggu build selesai (~10-15 menit)
3. Klik workflow run terbaru
4. Download `windows-installer` di bagian Artifacts

**File yang akan Anda dapatkan:**

- `Sistem Antrian Puskesmas_1.0.0_x64_en-US.msi` (Windows installer)
- `Sistem Antrian Puskesmas_1.0.0_x64-setup.exe` (Windows installer alternatif)

---

## Atau: Build di Komputer Windows

### 1. Install Tools (One-time setup)

```powershell
# Install Rust
winget install --id Rustlang.Rustup

# Install Node.js
winget install --id OpenJS.NodeJS

# Install Visual Studio Build Tools
winget install --id Microsoft.VisualStudio.2022.BuildTools
```

### 2. Build

```powershell
# Copy folder project ke Windows
# Lalu:
cd "C:\Users\YourName\Documents\Sistem Antrian Puskesmas"

npm install
npm run build:desktop
```

### 3. Installer ada di:

- `src-tauri\target\release\bundle\msi\`
- `src-tauri\target\release\bundle\nsis\`

---

## 📦 File Installer Saat Ini

### ✅ Mac (Sudah Ada)

```
src-tauri/target/release/bundle/dmg/Sistem Antrian Puskesmas_1.0.0_aarch64.dmg
```

### ⏳ Windows (Perlu Build)

Gunakan salah satu cara di atas untuk mendapatkan installer Windows.

---

## 💡 Tips

**Paling Mudah:** GitHub Actions

- Tidak perlu komputer Windows
- Build otomatis
- Gratis untuk public repo

**Paling Cepat:** Build lokal (jika punya Windows)

- Lebih cepat setelah setup
- Full control

---

## ❓ Troubleshooting

### GitHub Actions tidak jalan

- Pastikan Actions enabled di repo settings
- Push ke branch `main` atau `master`
- Atau create tag dengan `git tag v1.0.0`

### Build Windows gagal

- Install semua prerequisites
- Restart terminal setelah install
- Gunakan "Developer Command Prompt for VS"

---

Untuk detail lengkap, lihat: **README_BUILD.md**
