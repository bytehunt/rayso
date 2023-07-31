pub mod ui;

#[tokio::main]
async fn main() {
    if let Err(err) = ui::rayso::ray().await {
        println!("Error: {}", err);
        std::process::exit(1);
    }
}
