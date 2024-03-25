extern crate hex;
extern crate lambdaworks;

use hex::FromHex;
use lambdaworks::bls12_381::{Scalar, G1};

fn main() {
    // Given secret key (hex representation)
    let secret_key_hex = "6C616D6264617370";
    let secret_key_bytes = Vec::from_hex(secret_key_hex).unwrap();
    let secret_key = Scalar::from_bytes(&secret_key_bytes).unwrap();

    // Compute the public key
    let generator = G1::one();
    let public_key = generator * secret_key;

    // Print the X and Y coordinates of the public key
    println!("Public Key (X): {}", public_key.x().to_hex());
    println!("Public Key (Y): {}", public_key.y().to_hex());
}
