use rand::{Rng, thread_rng};
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;

// Type alias for AES-256-CBC with PKCS7 padding
type Aes256Cbc = Cbc<Aes256, Pkcs7>;

fn encrypt_text(key: &[u8], iv: &[u8], plaintext: &str) -> Vec<u8> {
    // Generate a random IV if not provided
    let mut rng = thread_rng();
    let iv = if iv.is_empty() { rng.gen::<[u8; 16]>() } else { *iv };

    // Create the cipher instance
    let cipher = Aes256Cbc::new_var(key, &iv).expect("Failed to create AES cipher");

    // Encrypt the plaintext
    let ciphertext = cipher.encrypt_vec(plaintext.as_bytes());

    // Combine IV and ciphertext
    let mut result = Vec::from(iv);
    result.extend_from_slice(&ciphertext);

    result
}

fn decrypt_text(key: &[u8], ciphertext: &[u8]) -> Result<String, &'static str> {
    // Extract IV from the beginning of the ciphertext
    let iv = &ciphertext[..16];
    let ciphertext = &ciphertext[16..];

    // Create the cipher instance
    let cipher = Aes256Cbc::new_var(key, iv).expect("Failed to create AES cipher");

    // Decrypt the ciphertext
    let plaintext = cipher.decrypt_vec(ciphertext);

    // Convert the result to a String (or handle errors)
    match plaintext {
        Ok(plain) => Ok(String::from_utf8_lossy(&plain).to_string()),
        Err(_) => Err("Decryption failed"),
    }
}