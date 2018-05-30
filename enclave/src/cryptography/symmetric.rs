use tiny_keccak::Keccak;
use secp256k1::{PublicKey, SecretKey, SharedSecret};
use ring::digest;
use ring::aead;
use ring::rand::{SystemRandom, SecureRandom};
use std::string::String;
use std::vec::Vec;
use std::option::Option;

static AES_MODE: &aead::Algorithm = &aead::AES_256_GCM;

pub fn encrypt(message: &Vec<u8>, key: &[u8], _iv: Option<[u8; 12]>) -> Vec<u8> {

    let mut iv: [u8; 12];
    match _iv {
        Some(x) => {iv = x;},
        None => {
            iv = [0; 12];
            let r = SystemRandom::new();
            r.fill(&mut iv);
        }
    }
    let additional_data: [u8; 0] = [];

    let enc_key = digest::digest(&digest::SHA256, &key);
    let aes_encrypt = aead::SealingKey::new(&AES_MODE, enc_key.as_ref()).unwrap();

    let mut in_out = message.clone();
    let tag_size = AES_MODE.tag_len();
    for _ in 0..tag_size {
        in_out.push(0);
    }
    let seal_size = aead::seal_in_place(&aes_encrypt, &iv, &additional_data, &mut in_out, tag_size).expect(&"AES encryption failed");
    in_out.append(&mut iv.to_vec());
    in_out
}

pub fn decrypt(cipheriv: &Vec<u8>, key: &[u8]) {
    let enc_key = digest::digest(&digest::SHA256, &key);
    let aes_decrypt = aead::OpeningKey::new(&AES_MODE, enc_key.as_ref()).unwrap();
    let additional_data: [u8; 0] = [];
    let mut ciphertext = cipheriv.clone();
    let mut iv: [u8; 12] = [0; 12];
    for _i in (0..iv.len()).rev() {
        iv[_i] = ciphertext.pop().unwrap();
    }
    println!("CipherText: {:?}", ciphertext);
    println!("{:?}", iv);

    let decrypted_data = aead::open_in_place(&aes_decrypt, &iv, &additional_data, 0, &mut ciphertext).expect(&"AES decryption failed");
    println!("Decrypted: {:?}",String::from_utf8_lossy(decrypted_data));
}