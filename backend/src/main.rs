use crate::config::CONFIG;

mod config;

fn main() {
    println!(
        "host: {}, port: {}",
        CONFIG.read().unwrap().host,
        CONFIG.read().unwrap().port
    );
}
