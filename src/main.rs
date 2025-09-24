use std::{
    env,
    io::{self, Write},
    fs::File,
    path::Path,
    sync::Arc,
};

// Wallet generation dependencies
use blake2::{Blake2b, Digest};
use serde::{Serialize, Deserialize};

// ----------------------- wallet generation -----------------------
/// Bech32m charset for Kaspa
const CHARSET: &[u8] = b"qpzry9x8gf2tvdw0s3jn54khce6mua7l";

fn bech32_polymod(values: &[u8]) -> u32 {
    let gen = [0x3b6a57b2, 0x26508e6d, 0x1ea119fa, 0x3d4233dd, 0x2a1462b3];
    let mut chk: u32 = 1;
    for v in values {
        let b = chk >> 25;
        chk = ((chk & 0x1ffffff) << 5) ^ (*v as u32);
        for i in 0..5 {
            if ((b >> i) & 1) != 0 {
                chk ^= gen[i];
            }
        }
    }
    chk
}

fn bech32_hrp_expand(hrp: &str) -> Vec<u8> {
    let mut v = Vec::new();
    for b in hrp.bytes() {
        v.push(b >> 5);
    }
    v.push(0);
    for b in hrp.bytes() {
        v.push(b & 0x1f);
    }
    v
}

fn bech32_create_checksum(hrp: &str, data: &[u8]) -> Vec<u8> {
    let mut values = bech32_hrp_expand(hrp);
    values.extend_from_slice(data);
    let polymod = bech32_polymod(&[&values[..], &[0, 0, 0, 0, 0, 0]].concat()) ^ 0x2bc830a3;
    (0..6).map(|i| ((polymod >> (5 * (5 - i))) & 0x1f) as u8).collect()
}

fn bech32_encode(hrp: &str, data: &[u8]) -> String {
    let mut combined = Vec::from(data);
    combined.extend(bech32_create_checksum(hrp, data));
    let chars: String = combined.iter().map(|&d| CHARSET[d as usize] as char).collect();
    format!("{}:{}", hrp, chars)
}

fn convertbits(data: &[u8], frombits: u32, tobits: u32, pad: bool) -> Vec<u8> {
    let mut acc: u32 = 0;
    let mut bits: u32 = 0;
    let maxv: u32 = (1 << tobits) - 1;
    let mut ret = Vec::new();
    for &value in data {
        acc = (acc << frombits) | (value as u32);
        bits += frombits;
        while bits >= tobits {
            bits -= tobits;
            ret.push(((acc >> bits) & maxv) as u8);
        }
    }
    if pad && bits > 0 {
        ret.push(((acc << (tobits - bits)) & maxv) as u8);
    }
    ret
}

fn kaspa_address(pub_key_bytes: &[u8]) -> String {
    let mut hasher = Blake2b::<blake2::digest::consts::U32>::new();
    hasher.update(pub_key_bytes);
    let hash = hasher.finalize();
    let pub_key_hash = &hash[..20];
    let data = convertbits(pub_key_hash, 8, 5, true);
    bech32_encode("kaspa", &data)
}

#[derive(Serialize, Deserialize, Debug)]
struct Wallet {
    id: u32,
    private_key: String,
    public_key: String,
    address: String,
}

#[derive(Debug, Clone)]
enum OutputFormat {
    Txt,
    Csv,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "txt" | "text" => Ok(OutputFormat::Txt),
            "csv" => Ok(OutputFormat::Csv),
            _ => Err(format!("Unknown format: {}. Supported formats: txt, csv", s)),
        }
    }
}

fn generate_wallets(n: usize) -> Vec<Wallet> {
    let mut wallets = Vec::new();
    for i in 0..n {
        // Generate deterministic private key based on index for testing
        // In production, use proper cryptographic random number generation
        let mut private_key = [0u8; 32];
        
        // Create a more varied private key based on index
        for j in 0..32 {
            private_key[j] = ((i * 256 + j + (i * i * 7)) % 256) as u8;
        }
        
        // Add more variation to make each wallet unique
        private_key[0] = (i % 256) as u8;
        private_key[1] = ((i * 3) % 256) as u8;
        private_key[30] = ((i * 5) % 256) as u8;
        private_key[31] = ((i * 7) % 256) as u8;
        
        // Generate a different public key based on the private key
        // This is a simplified approach - in production, use proper ECDSA
        let mut public_key = [0u8; 33]; // Compressed public key format
        public_key[0] = 0x02; // Compressed key prefix
        
        // Generate public key bytes based on private key
        for j in 0..32 {
            public_key[j + 1] = private_key[j] ^ ((i * 11 + j * 13) % 256) as u8;
        }
        
        let private_key_hex = hex::encode(&private_key);
        let public_key_hex = hex::encode(&public_key);
        let address = kaspa_address(&public_key);

        wallets.push(Wallet {
            id: (i + 1) as u32,
            private_key: private_key_hex,
            public_key: public_key_hex,
            address,
        });
    }
    wallets
}

fn save_wallets_txt(wallets: &[Wallet], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(filename)?;
    
    for wallet in wallets {
        writeln!(file, "Wallet {}", wallet.id)?;
        writeln!(file, "ID: {}", wallet.id)?;
        writeln!(file, "Private Key (hex): {}", wallet.private_key)?;
        writeln!(file, "Public Key (hex): {}", wallet.public_key)?;
        writeln!(file, "Kaspa Address: {}", wallet.address)?;
        writeln!(file, "{}", "-".repeat(60))?;
    }
    
    Ok(())
}

fn save_wallets_csv(wallets: &[Wallet], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut wtr = csv::Writer::from_path(filename)?;
    
    // Write header
    wtr.write_record(&["ID", "Private Key", "Public Key", "Address"])?;
    
    // Write wallet data
    for wallet in wallets {
        wtr.write_record(&[
            &wallet.id.to_string(),
            &wallet.private_key,
            &wallet.public_key,
            &wallet.address,
        ])?;
    }
    
    wtr.flush()?;
    Ok(())
}


fn save_wallets(wallets: &[Wallet], format: OutputFormat, base_filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let filename = match format {
        OutputFormat::Txt => format!("{}.txt", base_filename),
        OutputFormat::Csv => format!("{}.csv", base_filename),
    };
    
    match format {
        OutputFormat::Txt => save_wallets_txt(wallets, &filename)?,
        OutputFormat::Csv => save_wallets_csv(wallets, &filename)?,
    }
    
    Ok(filename)
}

fn generate_wallets_cli() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let mut count = 10; // Default to 10 wallets
    let mut format = OutputFormat::Txt; // Default to TXT format
    let mut output_name = "wallets".to_string(); // Default output name
    
    // Parse arguments
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--count" | "-c" => {
                if i + 1 < args.len() {
                    if let Ok(n) = args[i + 1].parse::<usize>() {
                        count = n;
                    }
                    i += 1;
                }
            }
            arg if arg.starts_with("--count=") => {
                if let Some(n_str) = arg.strip_prefix("--count=") {
                    if let Ok(n) = n_str.parse::<usize>() {
                        count = n;
                    }
                }
            }
            "--format" | "-f" => {
                if i + 1 < args.len() {
                    if let Ok(f) = args[i + 1].parse::<OutputFormat>() {
                        format = f;
                    }
                    i += 1;
                }
            }
            arg if arg.starts_with("--format=") => {
                if let Some(f_str) = arg.strip_prefix("--format=") {
                    if let Ok(f) = f_str.parse::<OutputFormat>() {
                        format = f;
                    }
                }
            }
            "--output" | "-o" => {
                if i + 1 < args.len() {
                    output_name = args[i + 1].clone();
                    i += 1;
                }
            }
            arg if arg.starts_with("--output=") => {
                if let Some(o_str) = arg.strip_prefix("--output=") {
                    output_name = o_str.to_string();
                }
            }
            _ => {}
        }
        i += 1;
    }

    println!("🔐 Kaspa Wallet Generator");
    println!("⏳ Generating {} Kaspa wallets in {:?} format...\n", count, format);
    
    let wallets = generate_wallets(count);
    let filename = save_wallets(&wallets, format.clone(), &output_name)?;

    println!("✅ {} wallets saved to {}", count, filename);
    println!("\n📋 Usage Instructions:");
    println!("   • Use any private key from {} in your .env file", filename);
    println!("   • Set PRIVATE_KEY_HEX=<private_key_hex> in .env");
    println!("   • Run transaction generator with: docker-compose up --build");
    
    // Show sample data based on format
    match format {
        OutputFormat::Txt => {
            println!("\n📄 Sample output (first 3 wallets):");
            println!("{}", "-".repeat(50));
            for wallet in wallets.iter().take(3) {
                println!("Wallet {}", wallet.id);
                println!("ID: {}", wallet.id);
                println!("Private Key (hex): {}", wallet.private_key);
                println!("Public Key (hex): {}", wallet.public_key);
                println!("Kaspa Address: {}", wallet.address);
                println!("{}", "-".repeat(30));
            }
        }
        OutputFormat::Csv => {
            println!("\n📊 CSV format with headers: ID, Private Key, Public Key, Address");
        }
    }
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();

    // Check if we should run in wallet generation mode
    if env::args().any(|arg| arg == "--generate-wallets" || arg == "--gen-wallets") {
        return generate_wallets_cli();
    }

    // Show help if no arguments or help requested
    if env::args().len() == 1 || env::args().any(|arg| arg == "--help" || arg == "-h") {
        println!("🔐 Kaspa Wallet Generator");
        println!("========================");
        println!();
        println!("📋 Commands:");
        println!("   --generate-wallets    Generate 10 wallets in TXT format (default)");
        println!("   --gen-wallets         Same as --generate-wallets");
        println!();
        println!("⚙️  Options:");
        println!("   --count N             Generate N wallets (default: 10)");
        println!("   -c N                  Same as --count N");
        println!("   --format FORMAT       Output format: txt, csv (default: txt)");
        println!("   -f FORMAT             Same as --format FORMAT");
        println!("   --output NAME         Output filename without extension (default: wallets)");
        println!("   -o NAME               Same as --output NAME");
        println!("   --help                Show this help");
        println!("   -h                    Same as --help");
        println!();
        println!("💡 Examples:");
        println!("   kaspa-tx-generator --gen-wallets");
        println!("   kaspa-tx-generator --gen-wallets --count 5 --format csv");
        println!("   kaspa-tx-generator --gen-wallets -c 20 -f csv -o my_wallets");
        println!("   kaspa-tx-generator --gen-wallets --format=csv --count=100");
        println!();
        println!("📁 Output Formats:");
        println!("   TXT      - Human-readable text with separators");
        println!("   CSV      - Comma-separated values with headers");
        println!();
        println!("📊 Output Columns:");
        println!("   ID, Private Key, Public Key, Address");
        return Ok(());
    }

    // If we get here, show default help
    println!("🔐 Kaspa Wallet Generator");
    println!("Use --help for usage information");
    Ok(())
}
