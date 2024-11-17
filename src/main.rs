pub mod avan;
pub mod client_http2;

#[tokio::main]
async fn main() {

    //println!("{}", avan::read_config::get_smp_value());
    avan::parse_avan::parse_avan().await;
}