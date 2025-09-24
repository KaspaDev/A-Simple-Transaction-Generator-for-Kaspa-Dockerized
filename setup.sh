#!/bin/bash

# Kaspa Transaction Generator Setup Script
# This script helps you set up the environment for running the transaction generator

set -e

echo "🚀 Kaspa Transaction Generator Setup"
echo "===================================="
echo ""

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed. Please install Docker first:"
    echo "   https://docs.docker.com/get-docker/"
    exit 1
fi

# Check if Docker Compose is installed
if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose is not installed. Please install Docker Compose first:"
    echo "   https://docs.docker.com/compose/install/"
    exit 1
fi

echo "✅ Docker and Docker Compose are installed"
echo ""

# Check if .env file exists
if [ ! -f ".env" ]; then
    echo "📝 Creating .env file from template..."
    cp env.example .env
    echo "✅ Created .env file"
    echo ""
    echo "⚠️  IMPORTANT: You must edit .env and set your PRIVATE_KEY_HEX!"
    echo "   Open .env in your editor and replace 'ENTER_YOUR_PRIVATE_KEY_HERE'"
    echo "   with your actual private key hex string."
    echo ""
    read -p "Press Enter when you've updated the .env file..."
else
    echo "✅ .env file already exists"
fi

# Check if PRIVATE_KEY_HEX is set
if grep -q "ENTER_YOUR_PRIVATE_KEY_HERE" .env; then
    echo "❌ You haven't set your private key yet!"
    echo "   Please edit .env and replace 'ENTER_YOUR_PRIVATE_KEY_HERE'"
    echo "   with your actual private key hex string."
    exit 1
fi

echo "✅ Private key is configured"
echo ""

# Ask for network preference
echo "🌐 Which network would you like to use?"
echo "1) Testnet-10 (recommended for testing)"
echo "2) Mainnet (use real KAS - be careful!)"
echo ""
read -p "Enter your choice (1 or 2): " network_choice

case $network_choice in
    1)
        echo "Setting up for Testnet-10..."
        sed -i.bak 's/KASPA_NETWORK=mainnet/KASPA_NETWORK=testnet10/' .env
        sed -i.bak 's|KASPA_GRPC_URL=grpc://n-mainnet.kaspa.ws:16110|KASPA_GRPC_URL=grpc://n-testnet-10.kaspa.ws:16210|' .env
        echo "✅ Configured for Testnet-10"
        echo ""
        echo "💡 To get testnet KAS:"
        echo "   1. Visit https://ksocialnetwork.pages.dev/watching"
        echo "   2. Create a testnet account"
        echo "   3. Use the private key from there"
        echo "   4. Send testnet KAS to your address"
        ;;
    2)
        echo "Setting up for Mainnet..."
        echo "⚠️  WARNING: This will use real KAS!"
        read -p "Are you sure? Type 'yes' to continue: " confirm
        if [ "$confirm" != "yes" ]; then
            echo "Setup cancelled."
            exit 1
        fi
        echo "✅ Configured for Mainnet"
        ;;
    *)
        echo "Invalid choice. Using default (Testnet-10)..."
        sed -i.bak 's/KASPA_NETWORK=mainnet/KASPA_NETWORK=testnet10/' .env
        sed -i.bak 's|KASPA_GRPC_URL=grpc://n-mainnet.kaspa.ws:16110|KASPA_GRPC_URL=grpc://n-testnet-10.kaspa.ws:16210|' .env
        ;;
esac

echo ""
echo "🔧 Configuration Summary:"
echo "========================="
grep -E "^(KASPA_NETWORK|TARGET_TPS|TARGET_UTXO_COUNT|SPAM_DURATION_SECONDS)" .env | sed 's/^/   /'
echo ""

# Ask if user wants to start immediately
echo "🚀 Ready to start!"
echo ""
echo "What would you like to do?"
echo "1) Generate 10 Kaspa wallets (recommended first step)"
echo "2) Start transaction generator"
echo "3) Exit"
echo ""
read -p "Enter your choice (1, 2, or 3): " choice

case $choice in
    1)
        echo ""
        echo "🔐 Generating 10 Kaspa wallets..."
        echo "   This will create wallets.txt with private keys you can use"
        echo ""
        docker-compose run --rm kaspa-wallet-generator
        echo ""
        echo "✅ Wallets generated! Next steps:"
        echo "   1. Copy a private key from wallets.txt"
        echo "   2. Edit .env and set PRIVATE_KEY_HEX=<your_private_key>"
        echo "   3. Run option 2 to start the transaction generator"
        echo ""
        read -p "Press Enter to continue..."
        ;;
    2)
        echo ""
        echo "🏗️  Building and starting the transaction generator..."
        echo "   (This may take a few minutes on first run)"
        echo ""
        docker-compose up --build
        ;;
    3)
        echo ""
        echo "✅ Setup complete! To start later:"
        echo "   - Generate wallets: docker-compose run --rm kaspa-wallet-generator"
        echo "   - Start generator: docker-compose up --build"
        echo ""
        echo "📖 For more information, see README-Docker.md"
        ;;
    *)
        echo "Invalid choice. Exiting."
        exit 1
        ;;
esac
