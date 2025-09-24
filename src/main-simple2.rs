use std::{
    env,
    io::{self, Write},
    fs::File,
};

// Wallet generation dependencies
use blake2::{Blake2b, Digest};

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

struct Wallet {
    private_key: String,
    public_key: String,
    address: String,
}

fn generate_wallets(n: usize) -> Vec<Wallet> {
    let mut wallets = Vec::new();
    for _i in 0..n {
        // Generate random private key (32 bytes)
        let mut private_key = [0u8; 32];
        use rand::RngCore;
        rand::thread_rng().fill_bytes(&mut private_key);
        
        // For simplicity, we'll use the private key as the public key
        // In a real implementation, you'd derive the public key from the private key
        let public_key = private_key; // This is simplified
        
        let private_key_hex = hex::encode(&private_key);
        let public_key_hex = hex::encode(&public_key);
        let address = kaspa_address(&public_key);

        wallets.push(Wallet {
            private_key: private_key_hex,
            public_key: public_key_hex,
            address,
        });
    }
    wallets
}

fn generate_wallets_cli() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 Kaspa Wallet Generator (Simplified)");
    print!("👉 Enter number of wallets to generate: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let count: usize = match input.trim().parse() {
        Ok(n) if n > 0 => n,
        _ => {
            println!("❌ Please enter a valid positive integer.");
            return Ok(());
        }
    };

    println!("\n⏳ Generating {} Kaspa wallets...\n", count);
    let wallets = generate_wallets(count);

    let mut file = File::create("wallets.txt")?;

    for (idx, w) in wallets.iter().enumerate() {
        writeln!(file, "Wallet {}", idx + 1)?;
        writeln!(file, "Private Key (hex): {}", w.private_key)?;
        writeln!(file, "Public Key (hex): {}", w.public_key)?;
        writeln!(file, "Kaspa Address: {}", w.address)?;
        writeln!(file, "{}", "-".repeat(60))?;
    }

    println!("✅ {} wallets saved to wallets.txt", count);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();

    // Check if we should run in wallet generation mode
    if env::args().any(|arg| arg == "--generate-wallets" || arg == "--gen-wallets") {
        return generate_wallets_cli();
    }

    println!("🔐 Kaspa Wallet Generator (Simplified)");
    println!("Usage: kaspa-tx-generator --generate-wallets");
    println!("Or: kaspa-tx-generator --gen-wallets");
    Ok(())
}
