fn main() {
    println!("Hello, world!");

    dotenv::from_filename(".env").ok();
    dotenv::dotenv().ok();

    let result_descriptor = std::env::var("WALLET_DESCRIPTOR ");

    let descriptor = match result_descriptor {
        Ok(descriptor) => descriptor,
        Err(e) => panic!("Error: {}", e),
    };
    
    print!("Descriptor: {}", descriptor);
    dbg!(descriptor);
}
