use anvil_core::totp::generate_token;

#[test]
fn test_generate_token_returns_6_digit_string() {
    let secret = "JBSWY3DPEHPK3PXP".to_string();
    let token = generate_token(secret);

    assert_eq!(token.len(), 6);
    assert!(token.chars().all(|c| c.is_ascii_digit()));
}
