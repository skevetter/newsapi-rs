use env_logger::Env;
use newsapi_rs::constant::DEFAULT_LOG_LEVEL;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or(DEFAULT_LOG_LEVEL)).init();
    println!("NewsAPI Rust Client");
    println!("Please check the examples directory for usage examples");
    println!("Run with: cargo run --example top_headlines");
    println!("Run with: cargo run --example everything_search");
}
