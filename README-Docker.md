# Kaspa Transaction Generator - Docker Setup

A Dockerized version of the Kaspa transaction generator for easy deployment and testing.

## 🚀 Quick Start

### 1. Clone and Setup

```bash
git clone <repository-url>
cd A-Simple-Transaction-Generator-for-Kaspa-Dockerized
```

### 2. Configure Environment

```bash
# Copy the example environment file
cp env.example .env

# Edit the .env file with your configuration
nano .env
```

**⚠️ IMPORTANT: You MUST set your private key in the .env file before running!**

### 3. Run with Docker Compose

#### Option A: Interactive Setup (Recommended)

```bash
# Run the interactive setup script
./setup.sh
```

#### Option B: Manual Commands

**Generate Kaspa Wallets:**

```bash
# Generate 10 wallets (default)
docker-compose run --rm kaspa-wallet-generator

# Generate custom number of wallets
docker-compose run --rm kaspa-wallet-generator -- --gen-wallets --count 5
```

**Run Transaction Generator:**

```bash
# Build and start the container
docker-compose up --build

# Run in background
docker-compose up -d --build

# View logs
docker-compose logs -f

# Stop the container
docker-compose down
```

## 📋 Configuration

### Required Settings

| Variable          | Description                    | Example           |
| ----------------- | ------------------------------ | ----------------- |
| `PRIVATE_KEY_HEX` | Your private key in hex format | `a1b2c3d4e5f6...` |

### Network Configuration

| Variable         | Description    | Options                | Default                   |
| ---------------- | -------------- | ---------------------- | ------------------------- |
| `KASPA_NETWORK`  | Network to use | `mainnet`, `testnet10` | `mainnet`                 |
| `KASPA_GRPC_URL` | gRPC endpoint  | Custom URL             | Auto-set based on network |

### Transaction Parameters

| Variable                  | Description                       | Default     | Notes                             |
| ------------------------- | --------------------------------- | ----------- | --------------------------------- |
| `TARGET_UTXO_COUNT`       | Target number of UTXOs to create  | `100`       | More UTXOs = higher potential TPS |
| `AMOUNT_PER_UTXO`         | Amount per UTXO in sompi          | `150000000` | 1.5 KAS                           |
| `OUTPUTS_PER_TRANSACTION` | Outputs per splitting transaction | `10`        |                                   |
| `SPAM_DURATION_SECONDS`   | Duration to run (0 = forever)     | `86400`     | 24 hours                          |

### Performance Tuning

| Variable            | Description                    | Default | Notes                           |
| ------------------- | ------------------------------ | ------- | ------------------------------- |
| `TARGET_TPS`        | Target transactions per second | `50`    |                                 |
| `UNLEASHED`         | Remove safety cap              | `true`  | Set to `false` for safety       |
| `MILLIS_PER_TICK`   | Pacing tick interval (ms)      | `10`    | Lower = smoother TPS            |
| `BASE_FEE_RATE`     | Base fee rate (sompi/gram)     | `1`     |                                 |
| `CLIENT_POOL_SIZE`  | gRPC client pool size          | `8`     | More clients = more parallelism |
| `UTXO_REFRESH_SECS` | UTXO refresh interval          | `1`     | How often to refresh UTXOs      |

## 🔐 Wallet Generation

The application now includes a built-in Kaspa wallet generator that can create secure wallets with:

- **🔐 24-word BIP39 mnemonics** for secure seed generation
- **📈 Custom quantity** via CLI prompt
- **🧠 HD derivation** using m/44'/111111'/0'/0/i path
- **⚡ Fast generation** (under 2 seconds for 50 wallets)
- **✨ Kaspa Bech32m addresses** using custom BLAKE2b logic
- **💾 Clean output** to wallets.txt file

### Generate Wallets

**Simple Usage (Recommended):**

```bash
# Generate 10 wallets with one command
docker-compose run --rm kaspa-wallet-generator
```

**Custom Options:**

```bash
# Generate 5 wallets
docker-compose run --rm kaspa-wallet-generator -- --gen-wallets --count 5

# Generate 20 wallets
docker-compose run --rm kaspa-wallet-generator -- --gen-wallets -c 20
```

**Interactive Setup:**

```bash
# Use the guided setup
./setup.sh
# Then select option 1
```

### Wallet Output

Generated wallets are saved to `wallets.txt` with the following format:

```
Wallet 1
Mnemonic: abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about
Private Key (hex): 0123456789abcdef...
Public Key (compressed hex): 02abcdef...
Kaspa Address: kaspa:qpx...
------------------------------------------------------------
```

## 🔧 Getting a Private Key

### Option 1: Use Built-in Generator (Recommended)

1. Run `docker-compose run --rm kaspa-wallet-generator`
2. Enter the number of wallets to generate
3. Use any private key from the generated `wallets.txt` file

### Option 2: Community Generator

1. Visit [Kaspa Wallet Generator](https://github.com/deepakdhaka-1/Kaspa-Wallet-Generate)
2. Follow instructions to get 24-word seed and private key
3. **Verify the seed** in kaspa-NG web wallet first
4. Use the **private key hex** (not the seed) in your .env file

### Option 3: K Social (Testnet)

1. Visit [K Social](https://ksocialnetwork.pages.dev/watching)
2. Create a testnet account
3. Extract the private key from the account

### Option 4: kaspa-NG Wallet

1. Create a wallet in kaspa-NG
2. Export the private key
3. Use the hex format

## 🌐 Networks

### Mainnet

- **Network ID**: `mainnet`
- **gRPC URL**: `grpc://n-mainnet.kaspa.ws:16110`
- **Address Prefix**: `kaspa:`
- **⚠️ Use real KAS - be careful!**

### Testnet-10

- **Network ID**: `testnet10`
- **gRPC URL**: `grpc://n-testnet-10.kaspa.ws:16210`
- **Address Prefix**: `kaspatest:`
- **✅ Safe for testing**

## 💰 Funding Your Address

1. **Generate your key pair** using one of the methods above
2. **Load the seed in kaspa-NG** to get your public address
3. **Send test coins** to that address:
   - **Testnet**: Use faucets or testnet exchanges
   - **Mainnet**: Send real KAS (be careful!)
4. **Verify the balance** in kaspa-NG before running the generator

## 🚦 How It Works

### Phase 1: UTXO Splitting

1. Analyzes your current UTXOs
2. If you have fewer than `TARGET_UTXO_COUNT`, it splits your largest UTXO
3. Creates many small UTXOs in batches of `OUTPUTS_PER_TRANSACTION`
4. This prepares a pool of spendable UTXOs for high-rate sending

### Phase 2: Transaction Spam

1. Sends 1-input 1-output self-payments at `TARGET_TPS`
2. Uses parallel processing and async queues for smooth throughput
3. Refreshes UTXOs regularly to maintain the pool
4. Tracks real-time TPS and statistics

## 📊 Monitoring

The container provides real-time statistics:

```
TPS (1s avg): 45.2 | sent: 45 | mempool(node): 1234 | inflight: 8 | local-pending: 12 | UTXOs left: 87 | runtime: 120s
```

- **TPS**: Transactions per second (1-second average)
- **sent**: Total transactions sent in current period
- **mempool(node)**: Node's mempool size
- **inflight**: Transactions currently being submitted
- **local-pending**: UTXOs reserved for pending transactions
- **UTXOs left**: Available UTXOs for new transactions
- **runtime**: Total runtime in seconds

## 🛠️ Troubleshooting

### Common Issues

1. **"PRIVATE_KEY_HEX environment variable is required"**

   - Make sure you've set `PRIVATE_KEY_HEX` in your `.env` file

2. **"Address prefix does not match selected network"**

   - Check that your `KASPA_NETWORK` matches your private key's network
   - Mainnet keys start with `kaspa:`, testnet keys start with `kaspatest:`

3. **"Connected node does not look like [network]"**

   - Verify your `KASPA_GRPC_URL` is correct for the selected network
   - Check that the node is online and accessible

4. **"Largest UTXO has X KAS, more is needed"**

   - You need at least 10 KAS to run the splitting phase
   - Send more funds to your address

5. **Low TPS performance**
   - Increase `CLIENT_POOL_SIZE` for more parallelism
   - Increase `TARGET_UTXO_COUNT` for more available UTXOs
   - Check network connectivity and node performance

### Debug Mode

Enable debug logging:

```bash
# In your .env file
RUST_LOG=debug
```

### Manual Testing

Test the container manually:

```bash
# Build the image
docker build -t kaspa-tx-generator .

# Run with custom environment
docker run -it --rm \
  -e PRIVATE_KEY_HEX=your_key_here \
  -e KASPA_NETWORK=testnet10 \
  -e TARGET_TPS=10 \
  kaspa-tx-generator
```

## 🔒 Security Notes

- **Never commit your private key** to version control
- **Use testnet first** to verify everything works
- **Start with low TPS** and gradually increase
- **Monitor your balance** to avoid running out of funds
- **Set `UNLEASHED=false`** initially for safety

## 📈 Performance Tips

1. **Start with testnet** to tune your configuration
2. **Increase UTXO count** for higher TPS potential
3. **Use more client connections** for better parallelism
4. **Monitor node performance** - your TPS is limited by the node
5. **Adjust fee rates** if transactions are being rejected

## 🆘 Support

- Check the [original README](README.md) for detailed technical information
- Visit the [Kaspa Discord](https://discord.gg/kaspa) for community support
- Report issues on the project repository

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.
