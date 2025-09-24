# Kaspa Transaction Generator - Complete Index

## 📁 Project Structure

```
A-Simple-Transaction-Generator-for-Kaspa-Dockerized/
├── src/
│   └── main.rs                 # Main application code (environment-aware)
├── Cargo.toml                  # Rust dependencies and project config
├── Dockerfile                  # Multi-stage Docker build configuration
├── docker-compose.yml          # Docker Compose orchestration
├── .env.example               # Environment variables template
├── .dockerignore              # Docker build exclusions
├── setup.sh                   # Interactive setup script
├── README.md                  # Original project documentation
├── README-Docker.md           # Docker-specific documentation
├── INDEX.md                   # This comprehensive index
└── LICENSE                    # MIT License
```

## 🔧 Configuration Files

### Environment Variables (.env)

The application is configured through environment variables. Copy `env.example` to `.env` and customize:

#### Required Configuration

- **`PRIVATE_KEY_HEX`**: Your private key in hex format (REQUIRED)

#### Network Configuration

- **`KASPA_NETWORK`**: `mainnet` or `testnet10`
- **`KASPA_GRPC_URL`**: gRPC endpoint (auto-set based on network)

#### Transaction Parameters

- **`TARGET_UTXO_COUNT`**: Number of UTXOs to create (default: 100)
- **`AMOUNT_PER_UTXO`**: Amount per UTXO in sompi (default: 150000000 = 1.5 KAS)
- **`OUTPUTS_PER_TRANSACTION`**: Outputs per splitting transaction (default: 10)
- **`SPAM_DURATION_SECONDS`**: Duration to run (0 = forever, default: 86400)

#### Performance Tuning

- **`TARGET_TPS`**: Target transactions per second (default: 50)
- **`UNLEASHED`**: Remove safety cap (default: true)
- **`MILLIS_PER_TICK`**: Pacing tick interval in ms (default: 10)
- **`BASE_FEE_RATE`**: Base fee rate in sompi/gram (default: 1)
- **`CLIENT_POOL_SIZE`**: gRPC client pool size (default: 8)
- **`UTXO_REFRESH_SECS`**: UTXO refresh interval (default: 1)
- **`MIN_CHANGE_SOMPI`**: Minimum change amount (default: 1000000)
- **`MAX_PENDING_AGE_SECS`**: Max pending transaction age (default: 3600)

#### Logging

- **`RUST_LOG`**: Log level (error, warn, info, debug, trace, default: info)

### Docker Configuration

#### Dockerfile

- **Base Image**: `rust:1.75-slim` (builder), `debian:bookworm-slim` (runtime)
- **Multi-stage Build**: Optimizes image size
- **Security**: Runs as non-root user
- **Dependencies**: Installs required system libraries

#### docker-compose.yml

- **Service**: `kaspa-tx-generator`
- **Environment**: All configuration via environment variables
- **Networking**: Custom bridge network
- **Restart Policy**: `unless-stopped`

## 🚀 Quick Start Guide

### 1. Automated Setup (Recommended)

```bash
# Clone the repository
git clone <repository-url>
cd A-Simple-Transaction-Generator-for-Kaspa-Dockerized

# Run the interactive setup script
./setup.sh
```

### 2. Manual Setup

```bash
# Copy environment template
cp env.example .env

# Edit configuration
nano .env

# Build and run
docker-compose up --build
```

## 🔑 Getting a Private Key

### Method 1: Community Generator

1. Visit [Kaspa Wallet Generator](https://github.com/deepakdhaka-1/Kaspa-Wallet-Generate)
2. Generate 24-word seed and private key
3. **Verify in kaspa-NG** web wallet first
4. Use the **private key hex** (not seed) in .env

### Method 2: K Social Testnet

1. Visit [K Social](https://ksocialnetwork.pages.dev/watching)
2. Create testnet account
3. Extract private key

### Method 3: kaspa-NG Wallet

1. Create wallet in kaspa-NG
2. Export private key
3. Use hex format

## 🌐 Network Configuration

### Mainnet

- **Network ID**: `mainnet`
- **gRPC URL**: `grpc://n-mainnet.kaspa.ws:16110`
- **Address Prefix**: `kaspa:`
- **⚠️ Uses real KAS**

### Testnet-10

- **Network ID**: `testnet10`
- **gRPC URL**: `grpc://n-testnet-10.kaspa.ws:16210`
- **Address Prefix**: `kaspatest:`
- **✅ Safe for testing**

## 💰 Funding Your Address

1. **Generate key pair** using methods above
2. **Load seed in kaspa-NG** to get public address
3. **Send test coins**:
   - **Testnet**: Use faucets or testnet exchanges
   - **Mainnet**: Send real KAS (be careful!)
4. **Verify balance** before running generator

## 🔄 How the Application Works

### Phase 1: UTXO Analysis and Splitting

1. **Connects** to Kaspa node via gRPC
2. **Analyzes** current UTXOs for the address
3. **Splits** largest UTXO if needed to reach `TARGET_UTXO_COUNT`
4. **Creates** many small UTXOs in batches
5. **Waits** for confirmations

### Phase 2: High-Rate Transaction Spam

1. **Sends** 1-input 1-output self-payments
2. **Paces** at `TARGET_TPS` with smooth control
3. **Refreshes** UTXOs regularly to maintain pool
4. **Tracks** real-time statistics and performance
5. **Handles** errors and retries gracefully

## 📊 Monitoring and Statistics

### Real-time Output

```
TPS (1s avg): 45.2 | sent: 45 | mempool(node): 1234 | inflight: 8 | local-pending: 12 | UTXOs left: 87 | runtime: 120s
```

### Metrics Explained

- **TPS**: Transactions per second (1-second average)
- **sent**: Total transactions sent in current period
- **mempool(node)**: Node's mempool size
- **inflight**: Transactions currently being submitted
- **local-pending**: UTXOs reserved for pending transactions
- **UTXOs left**: Available UTXOs for new transactions
- **runtime**: Total runtime in seconds

### Additional Logging

- **10-second TPS average**: Rolling average for trend analysis
- **Total sent**: Cumulative transaction count
- **Error handling**: Failed transaction retry logic

## 🛠️ Troubleshooting

### Common Issues

#### Configuration Errors

- **"PRIVATE_KEY_HEX environment variable is required"**

  - Set `PRIVATE_KEY_HEX` in .env file

- **"Address prefix does not match selected network"**

  - Check `KASPA_NETWORK` matches your private key's network
  - Mainnet: `kaspa:`, Testnet: `kaspatest:`

- **"Connected node does not look like [network]"**
  - Verify `KASPA_GRPC_URL` is correct
  - Check node is online and accessible

#### Insufficient Funds

- **"Largest UTXO has X KAS, more is needed"**
  - Need at least 10 KAS for splitting phase
  - Send more funds to your address

#### Performance Issues

- **Low TPS performance**
  - Increase `CLIENT_POOL_SIZE` for more parallelism
  - Increase `TARGET_UTXO_COUNT` for more UTXOs
  - Check network connectivity and node performance

### Debug Mode

```bash
# Enable debug logging in .env
RUST_LOG=debug
```

### Manual Testing

```bash
# Build and test manually
docker build -t kaspa-tx-generator .
docker run -it --rm \
  -e PRIVATE_KEY_HEX=your_key_here \
  -e KASPA_NETWORK=testnet10 \
  -e TARGET_TPS=10 \
  kaspa-tx-generator
```

## 🔒 Security Best Practices

### Private Key Security

- **Never commit** private keys to version control
- **Use testnet first** to verify everything works
- **Start with low TPS** and gradually increase
- **Monitor balance** to avoid running out of funds

### Network Safety

- **Set `UNLEASHED=false`** initially for safety
- **Use testnet** for initial testing
- **Verify configuration** before running on mainnet

## 📈 Performance Optimization

### Tuning Guidelines

1. **Start with testnet** to tune configuration
2. **Increase UTXO count** for higher TPS potential
3. **Use more client connections** for better parallelism
4. **Monitor node performance** - TPS limited by node capacity
5. **Adjust fee rates** if transactions are rejected

### Recommended Settings by Use Case

#### Testing/Development

```env
KASPA_NETWORK=testnet10
TARGET_TPS=10
TARGET_UTXO_COUNT=50
UNLEASHED=false
```

#### Production (Mainnet)

```env
KASPA_NETWORK=mainnet
TARGET_TPS=100
TARGET_UTXO_COUNT=200
UNLEASHED=true
CLIENT_POOL_SIZE=16
```

#### High-Performance Testing

```env
TARGET_TPS=500
TARGET_UTXO_COUNT=1000
CLIENT_POOL_SIZE=32
MILLIS_PER_TICK=5
```

## 📚 Additional Resources

### Documentation

- **README.md**: Original project documentation
- **README-Docker.md**: Docker-specific documentation
- **INDEX.md**: This comprehensive index

### External Resources

- [Kaspa Discord](https://discord.gg/kaspa): Community support
- [Kaspa Wallet Generator](https://github.com/deepakdhaka-1/Kaspa-Wallet-Generate): Key generation
- [K Social](https://ksocialnetwork.pages.dev/watching): Testnet account creation
- [kaspa-NG Wallet](https://kaspa-ng.com): Web wallet for verification

### Technical References

- [Rusty Kaspa](https://github.com/kaspanet/rusty-kaspa): Core implementation
- [Kaspa Documentation](https://docs.kaspa.org): Official documentation
- [Docker Documentation](https://docs.docker.com): Container platform

## 🆘 Support and Contributing

### Getting Help

1. Check this index and documentation
2. Review troubleshooting section
3. Visit Kaspa Discord for community support
4. Report issues on the project repository

### Contributing

- Fork the repository
- Create a feature branch
- Make your changes
- Submit a pull request

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

**⚠️ Disclaimer**: This tool is for testing and development purposes. Use responsibly and at your own risk. Always test on testnet first before using on mainnet.
