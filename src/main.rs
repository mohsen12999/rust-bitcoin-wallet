fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    dotenv::from_filename(".env").ok();
    dotenv::dotenv().ok();

    let descriptor = std::env::var("WALLET_DESCRIPTOR ")?;
    
    print!("Descriptor: {}", descriptor);
    dbg!(descriptor);

    Ok(())
}
