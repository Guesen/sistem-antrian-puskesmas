#!/bin/bash

echo "Memperbaiki dependency conflicts..."
echo ""

echo "1. Menghapus node_modules dan package-lock.json..."
rm -rf node_modules
rm -f package-lock.json

echo ""
echo "2. Install ulang dependencies..."
npm install --legacy-peer-deps

echo ""
echo "Selesai! Sekarang jalankan 'npm run dev'"
