use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Hello, world!");
}
