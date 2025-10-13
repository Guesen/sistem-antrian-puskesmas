@echo off
echo Memperbaiki dependency conflicts...
echo.

echo 1. Menghapus node_modules dan package-lock.json...
if exist node_modules rmdir /s /q node_modules
if exist package-lock.json del package-lock.json

echo.
echo 2. Install ulang dependencies...
call npm install --legacy-peer-deps

echo.
echo Selesai! Sekarang jalankan 'npm run dev'
echo.
pause
