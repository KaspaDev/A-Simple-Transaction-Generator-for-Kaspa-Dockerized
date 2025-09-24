#!/bin/bash

# Test script for Docker setup
# This script tests the Docker build and basic functionality

set -e

echo "🧪 Testing Kaspa Transaction Generator Docker Setup"
echo "=================================================="
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

# Test 2: Check if image was created
if docker image inspect kaspa-tx-generator-test > /dev/null 2>&1; then
    echo "✅ Docker image created successfully"
else
    echo "❌ Docker image not found"
    exit 1
fi

# Test 3: Test container startup (without private key - should fail gracefully)
echo "🚀 Testing container startup (should fail without private key)..."
if docker run --rm kaspa-tx-generator-test 2>&1 | grep -q "PRIVATE_KEY_HEX environment variable is required"; then
    echo "✅ Container correctly requires private key"
else
    echo "❌ Container should require private key but didn't"
    exit 1
fi

# Test 4: Test with invalid private key
echo "🔑 Testing with invalid private key..."
if docker run --rm -e PRIVATE_KEY_HEX=invalid_key kaspa-tx-generator-test 2>&1 | grep -q "Invalid private key"; then
    echo "✅ Container correctly validates private key format"
else
    echo "❌ Container should validate private key format"
    exit 1
fi

# Test 5: Test environment variable parsing
echo "⚙️  Testing environment variable parsing..."
if docker run --rm \
    -e PRIVATE_KEY_HEX=0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef \
    -e KASPA_NETWORK=testnet10 \
    -e TARGET_TPS=1 \
    -e SPAM_DURATION_SECONDS=1 \
    kaspa-tx-generator-test 2>&1 | grep -q "Starting spam at 1 TPS"; then
    echo "✅ Environment variables parsed correctly"
else
    echo "❌ Environment variables not parsed correctly"
    exit 1
fi

# Cleanup
echo "🧹 Cleaning up test image..."
docker rmi kaspa-tx-generator-test > /dev/null 2>&1

echo ""
echo "🎉 All tests passed! Docker setup is working correctly."
echo ""
echo "Next steps:"
echo "1. Run ./setup.sh to configure your environment"
echo "2. Set your private key in .env"
echo "3. Run docker-compose up --build to start the generator"
