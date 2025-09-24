use bip39::{Mnemonic, Language, Seed};
use bip32::{XPrv, DerivationPath, Prefix};
use blake2::{Blake2b, Digest};
use std::io::{self, Write};
use std::fs::File;
use std::str::FromStr;

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
    let mut hasher = Blake2b::new();
    hasher.update(pub_key_bytes);
    let hash = hasher.finalize();
    let pub_key_hash = &hash[..20];
    let data = convertbits(pub_key_hash, 8, 5, true);
    bech32_encode("kaspa", &data)
}

struct Wallet {
    mnemonic: String,
    private_key: String,
    public_key: String,
    address: String,
}

fn generate_wallets(n: usize) -> Vec<Wallet> {
    let mut wallets = Vec::new();
    for i in 0..n {
        let mnemonic = Mnemonic::generate_in(Language::English, 24).unwrap();
        let mnemonic_phrase = mnemonic.phrase().to_string();
        let seed = Seed::new(&mnemonic, "");
        let path_str = format!("m/44'/111111'/0'/0/{}", i);
        let derivation_path = DerivationPath::from_str(&path_str).unwrap();
        let xprv = XPrv::new(seed.as_bytes()).unwrap();
        let child_xprv = xprv.derive_path(&derivation_path).unwrap();
        let priv_key_bytes = child_xprv.to_bytes();
        let priv_key_hex = hex::encode(&priv_key_bytes);
        let pub_key = child_xprv.public_key();
        let pub_key_bytes = pub_key.to_bytes();
        let pub_key_hex = hex::encode(&pub_key_bytes);
        let address = kaspa_address(&pub_key_bytes);

        wallets.push(Wallet {
            mnemonic: mnemonic_phrase,
            private_key: priv_key_hex,
            public_key: pub_key_hex,
            address,
        });
    }
    wallets
}

fn main() {
    println!("🔐 Kaspa Wallet Generator CLI");
    print!("👉 Enter number of wallets to generate: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        println!("❌ Failed to read input.");
        return;
    }
    let count: usize = match input.trim().parse() {
        Ok(n) if n > 0 => n,
        _ => {
            println!("❌ Please enter a valid positive integer.");
            return;
        }
    };

    println!("\n⏳ Generating {} Kaspa wallets...\n", count);
    let wallets = generate_wallets(count);

    let mut file = match File::create("wallets.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("❌ Failed to create wallets.txt: {}", e);
            return;
        }
    };

    for (idx, w) in wallets.iter().enumerate() {
        writeln!(file, "Wallet {}", idx + 1).unwrap();
        writeln!(file, "Mnemonic: {}", w.mnemonic).unwrap();
        writeln!(file, "Private Key (hex): {}", w.private_key).unwrap();
        writeln!(file, "Public Key (compressed hex): {}", w.public_key).unwrap();
        writeln!(file, "Kaspa Address: {}", w.address).unwrap();
        writeln!(file, "{}", "-".repeat(60)).unwrap();
    }

    println!("✅ {} wallets saved to wallets.txt", count);
}