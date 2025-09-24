#!/bin/bash

# Test script for wallet generation functionality
# This script tests the wallet generation mode

set -e

echo "🧪 Testing Kaspa Wallet Generation"
echo "=================================="
echo ""

# Check if Docker is available
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed"
    exit 1
fi

if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose is not installed"
    exit 1
fi

echo "✅ Docker and Docker Compose are available"
echo ""

# Test 1: Build the Docker image
echo "🔨 Testing Docker build..."
if docker build -t kaspa-tx-generator-test . > /dev/null 2>&1; then
    echo "✅ Docker build successful"
else
    echo "❌ Docker build failed"
    exit 1
fi

# Test 2: Test wallet generation mode
echo "🔐 Testing wallet generation mode..."
if echo "3" | docker run --rm -i kaspa-tx-generator-test --generate-wallets 2>&1 | grep -q "wallets saved to wallets.txt"; then
    echo "✅ Wallet generation mode works correctly"
else
    echo "❌ Wallet generation mode failed"
    exit 1
fi

# Test 3: Test with docker-compose
echo "🐳 Testing with docker-compose..."
if echo "2" | docker-compose run --rm kaspa-wallet-generator 2>&1 | grep -q "wallets saved to wallets.txt"; then
    echo "✅ Docker Compose wallet generation works"
else
    echo "❌ Docker Compose wallet generation failed"
    exit 1
fi

# Cleanup
echo "🧹 Cleaning up test image..."
docker rmi kaspa-tx-generator-test > /dev/null 2>&1

echo ""
echo "🎉 All wallet generation tests passed!"
echo ""
echo "Next steps:"
echo "1. Run ./setup.sh to use the interactive setup"
echo "2. Or run: docker-compose run --rm kaspa-wallet-generator"
echo "3. Generated wallets will be saved to wallets.txt"
