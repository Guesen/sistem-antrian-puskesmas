#!/bin/bash

echo "Setup Sistem Antrian Puskesmas Mrebet untuk Development Lokal"
echo ""

echo "1. Install dependencies..."
rm -rf node_modules package-lock.json
npm install

echo ""
echo "2. Setup database D1..."
npx wrangler d1 create sistem-antrian-dev

echo ""
echo "3. Jalankan migrasi database..."
npx wrangler d1 migrations apply sistem-antrian-dev --local

echo ""
echo "Setup selesai! Jalankan 'npm run dev' untuk memulai development server."
