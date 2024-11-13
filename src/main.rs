pub mod avan;
pub mod client_http2;

#[tokio::main]
async fn main() {
    println!("{}", avan::read_config::get_trade_url());
}