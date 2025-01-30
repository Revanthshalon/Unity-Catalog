use config::ApplicationConfig;

mod config;
mod errors;
mod service;

fn main() {
    let app_config = ApplicationConfig::load_config();
    match app_config {
        Ok(conf) => println!("{:#?}", conf),
        Err(e) => println!("{:#?}", e),
    }
}
