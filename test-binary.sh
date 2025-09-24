#!/bin/bash

# Comprehensive test script for the Kaspa Wallet Generator Binary
echo "🧪 Testing Kaspa Wallet Generator Binary"
echo "========================================"
echo ""

# Test 1: Help command
echo "1️⃣ Testing help command..."
cargo run -- --help > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✅ Help command works"
else
    echo "❌ Help command failed"
    exit 1
fi

# Test 2: Default generation (TXT)
echo "2️⃣ Testing default generation (10 wallets, TXT)..."
cargo run -- --gen-wallets > /dev/null 2>&1
if [ -f "wallets.txt" ]; then
    wallet_count=$(grep -c "Wallet " wallets.txt)
    echo "✅ Generated $wallet_count wallets in TXT format"
else
    echo "❌ Failed to generate TXT wallets"
    exit 1
fi

# Test 3: CSV generation
echo "3️⃣ Testing CSV generation (5 wallets)..."
cargo run -- --gen-wallets --count 5 --format csv > /dev/null 2>&1
if [ -f "wallets.csv" ]; then
    wallet_count=$(tail -n +2 wallets.csv | wc -l)
    echo "✅ Generated $wallet_count wallets in CSV format"
else
    echo "❌ Failed to generate CSV wallets"
    exit 1
fi

# Test 4: Custom output filename
echo "4️⃣ Testing custom output filename..."
cargo run -- --gen-wallets --count 3 --format txt --output test_wallets > /dev/null 2>&1
if [ -f "test_wallets.txt" ]; then
    wallet_count=$(grep -c "Wallet " test_wallets.txt)
    echo "✅ Generated $wallet_count wallets with custom filename"
else
    echo "❌ Failed to generate wallets with custom filename"
    exit 1
fi

# Test 5: Short form arguments
echo "5️⃣ Testing short form arguments..."
cargo run -- --gen-wallets -c 2 -f csv -o short_test > /dev/null 2>&1
if [ -f "short_test.csv" ]; then
    wallet_count=$(tail -n +2 short_test.csv | wc -l)
    echo "✅ Generated $wallet_count wallets with short arguments"
else
    echo "❌ Failed to generate wallets with short arguments"
    exit 1
fi

# Test 6: Equals syntax
echo "6️⃣ Testing equals syntax..."
cargo run -- --gen-wallets --count=4 --format=csv --output=equals_test > /dev/null 2>&1
if [ -f "equals_test.csv" ]; then
    wallet_count=$(tail -n +2 equals_test.csv | wc -l)
    echo "✅ Generated $wallet_count wallets with equals syntax"
else
    echo "❌ Failed to generate wallets with equals syntax"
    exit 1
fi

# Show sample outputs
echo ""
echo "📄 Sample TXT Output (first 2 wallets):"
echo "----------------------------------------"
head -15 wallets.txt

echo ""
echo "📊 Sample CSV Output (first 3 rows):"
echo "------------------------------------"
head -4 wallets.csv

echo ""
echo "🎉 All binary tests passed!"
echo ""
echo "📋 Available Commands:"
echo "   cargo run -- --gen-wallets                    # 10 wallets, TXT format"
echo "   cargo run -- --gen-wallets --count 5 --format csv"
echo "   cargo run -- --gen-wallets -c 20 -f csv -o my_wallets"
echo "   cargo run -- --gen-wallets --count=100 --format=csv"
echo "   cargo run -- --help                           # Show all options"
echo ""
echo "🐳 Docker Usage:"
echo "   docker-compose run --rm kaspa-wallet-generator -- --gen-wallets --count 10 --format csv"
