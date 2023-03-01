const PKCS8_PRIVATE_KEY_PEM_GOOD: &str = include_str!("key/good.pem");
const PKCS8_PRIVATE_KEY_PEM_BAD: &str = include_str!("key/bad.pem");

fn main() -> Result<(), String> {
    println!("Parsing good.pem");
    let _ = PKCS8_PRIVATE_KEY_PEM_GOOD.parse::<p256::SecretKey>().map_err(|e| e.to_string())?;
    println!("Parsing good.pem succeeded.");

    println!("Parsing bad.pem");
    let _ = PKCS8_PRIVATE_KEY_PEM_BAD.parse::<p256::SecretKey>().map_err(|e| e.to_string())?;
    println!("Parsing bad.pem succeeded.");
    Ok(())
}
