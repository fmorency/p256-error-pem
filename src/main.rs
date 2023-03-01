const PKCS8_PRIVATE_KEY_PEM: &str = include_str!("key/id-2.pem");

fn main() -> Result<(), String> {
    let _secret_key = PKCS8_PRIVATE_KEY_PEM.parse::<p256::SecretKey>().map_err(|e| e.to_string())?;
    Ok(())
}
