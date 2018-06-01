use openssl::symm::{Cipher, Mode, Crypter};

use ring::digest::{Context, SHA256};

pub fn kdf(data: &[u8]) -> Vec<u8> {
    let mut context = Context::new(&SHA256);
    context.update(data);
    context.clone().finish().as_ref().into()
}

pub fn encode(key: &[u8], plaintext: &[u8]) -> Vec<u8> {
    if key.len() != 32 {
        panic!("invalid key length");
    }

    let iv = vec![0u8; 16];

    // Create a cipher context for encryption.
    let mut encrypter = Crypter::new(
        Cipher::aes_256_cbc(),
        Mode::Encrypt,
        key,
        Some(iv.as_slice()),
    ).unwrap();
    encrypter.pad(false);

    let block_size = Cipher::aes_256_cbc().block_size();
    let mut ciphertext = vec![0; plaintext.len() + block_size];

    let mut count = encrypter.update(plaintext, &mut ciphertext).unwrap();
    count += encrypter.finalize(&mut ciphertext[count..]).unwrap();
    ciphertext.truncate(count);

    ciphertext
}

pub fn decode(key: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    if key.len() != 32 {
        panic!("invalid key length")
    }

    let iv = vec![0u8; 16];
    // Create a cipher context for decryption.
    let mut decrypter = Crypter::new(
        Cipher::aes_256_cbc(),
        Mode::Decrypt,
        key,
        Some(iv.as_slice()),
    ).unwrap();
    decrypter.pad(false);

    let block_size = Cipher::aes_256_cbc().block_size();
    let mut plaintext = vec![0; ciphertext.len() + block_size];


    // Decrypt 2 chunks of ciphertexts successively.
    let mut count = decrypter.update(ciphertext, &mut plaintext).unwrap();
    count += decrypter.finalize(&mut plaintext[count..]).unwrap();
    plaintext.truncate(count);

    plaintext
}

#[test]
fn test_encode_decode() {
    let key = vec![2u8; 32];
    let plaintext = vec![1u8; 128];

    let ciphertext = encode(key.as_slice(), plaintext.as_slice());
    assert_ne!(plaintext, ciphertext);
    assert_eq!(plaintext.len(), ciphertext.len());

    let roundtrip = decode(key.as_slice(), ciphertext.as_slice());
    assert_eq!(plaintext, roundtrip);
}
