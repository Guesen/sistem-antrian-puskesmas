@echo off
echo Setup Sistem Antrian Puskesmas Mrebet untuk Development Lokal
echo.

echo 1. Install dependencies...
if exist node_modules rmdir /s /q node_modules
if exist package-lock.json del package-lock.json
call npm install

echo.
echo 2. Setup database D1...
call npx wrangler d1 create sistem-antrian-dev

echo.
echo 3. Jalankan migrasi database...
call npx wrangler d1 migrations apply sistem-antrian-dev --local

echo.
echo Setup selesai! Jalankan 'npm run dev' untuk memulai development server.
echo.
pause
