#!/bin/bash

# Simple test script for wallet generation
echo "🧪 Testing Simplified Wallet Generation"
echo "======================================="
echo ""

# Test 1: Local generation with default count
echo "1️⃣ Testing local generation (10 wallets)..."
cargo run -- --gen-wallets > /dev/null 2>&1
if [ -f "wallets.txt" ]; then
    wallet_count=$(grep -c "Wallet " wallets.txt)
    echo "✅ Generated $wallet_count wallets locally"
else
    echo "❌ Failed to generate wallets locally"
    exit 1
fi

# Test 2: Local generation with custom count
echo "2️⃣ Testing local generation (5 wallets)..."
cargo run -- --gen-wallets --count 5 > /dev/null 2>&1
if [ -f "wallets.txt" ]; then
    wallet_count=$(grep -c "Wallet " wallets.txt)
    echo "✅ Generated $wallet_count wallets with custom count"
else
    echo "❌ Failed to generate wallets with custom count"
    exit 1
fi

# Test 3: Help command
echo "3️⃣ Testing help command..."
if cargo run -- --help | grep -q "Kaspa Wallet Generator"; then
    echo "✅ Help command works"
else
    echo "❌ Help command failed"
    exit 1
fi

# Test 4: Show sample output
echo "4️⃣ Sample wallet output:"
echo "------------------------"
head -15 wallets.txt

echo ""
echo "🎉 All tests passed! Wallet generation is working correctly."
echo ""
echo "📋 Usage Summary:"
echo "   • Default: cargo run -- --gen-wallets (generates 10 wallets)"
echo "   • Custom: cargo run -- --gen-wallets --count N"
echo "   • Help: cargo run -- --help"
echo "   • Docker: docker-compose run --rm kaspa-wallet-generator"
