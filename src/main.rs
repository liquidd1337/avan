pub mod avan;
pub mod client_http2;


fn main() {
    println!("{}", avan::read_config::get_percent_value());
}