extern crate present;

use present::*;

#[test]
fn test_encryption_ecb() {
    let key = Key80Bit::new([0xA, 0xC0, 0xA6, 0xE7, 0x63, 0x26, 0xBC, 0x7E, 0x82, 0x80]);
    let op_mode = OpMode::ECB;

    let to_encrypt = "this is a test string →in UTF8←";
    let (encrypted, iv) = encrypt_str(to_encrypt, &key, &op_mode);
    assert_eq!(encrypted.len(), 40);
    assert!(iv.is_none());
    let decrypt_result = decrypt_str(&encrypted, &key, &op_mode, None);
    assert_eq!(decrypt_result.unwrap(), to_encrypt);

    let to_encrypt = "ö";
    let (encrypted, iv) = encrypt_str(to_encrypt, &key, &op_mode);
    assert_eq!(encrypted.len(), 8);
    assert!(iv.is_none());
    let decrypt_result = decrypt_str(&encrypted, &key, &op_mode, None);
    assert_eq!(decrypt_result.unwrap(), to_encrypt);
}

#[test]
#[should_panic]
fn test_encryption_fails_with_differing_keys_ecb() {
    let to_encrypt = "foo bar baz ²³";
    let (encrypted, _) = encrypt_str(to_encrypt, &Key80Bit::new([0xAB; 10]), &OpMode::ECB);
    let decrypt_result = decrypt_str(&encrypted, &Key80Bit::new([0xAC; 10]), &OpMode::ECB, None);
    assert_eq!(decrypt_result.unwrap(), to_encrypt);
}

#[test]
fn test_encryption_cbc() {
    let key = Key80Bit::new([0xA, 0xC0, 0xA6, 0xE7, 0x63, 0x26, 0xBC, 0x7E, 0x82, 0x80]);
    let op_mode = OpMode::CBC;

    let to_encrypt = "this is a test string →in UTF8←";
    let (encrypted, iv) = encrypt_str(to_encrypt, &key, &op_mode);
    assert_eq!(encrypted.len(), 40);
    assert!(iv.is_some());
    let decrypt_result = decrypt_str(&encrypted, &key, &op_mode, iv);
    assert_eq!(decrypt_result.unwrap(), to_encrypt);

    let to_encrypt = "ö";
    let (encrypted, iv) = encrypt_str(to_encrypt, &key, &op_mode);
    assert_eq!(encrypted.len(), 8);
    assert!(iv.is_some());
    let decrypt_result = decrypt_str(&encrypted, &key, &op_mode, iv);
    assert_eq!(decrypt_result.unwrap(), to_encrypt);
}

#[test]
#[should_panic]
fn test_encryption_fails_with_differing_keys_cbc() {
    let to_encrypt = "foo bar baz ²³";
    let (encrypted, iv) = encrypt_str(to_encrypt, &Key80Bit::new([0xAB; 10]), &OpMode::CBC);
    let decrypt_result = decrypt_str(&encrypted, &Key80Bit::new([0xAC; 10]), &OpMode::CBC, iv);
    assert_eq!(decrypt_result.unwrap(), to_encrypt);
}

#[test]
#[should_panic]
fn test_encryption_fails_with_wrong_iv_cbc() {
    let to_encrypt = "foo bar baz ²³";
    let key = Key80Bit::new([0x23; 10]);
    let (encrypted, _) = encrypt_str(to_encrypt, &key, &OpMode::CBC);
    let decrypt_result = decrypt_str(&encrypted, &key, &OpMode::CBC, Some(Block::new(0u64)));
    assert_eq!(decrypt_result.unwrap(), to_encrypt);
}
