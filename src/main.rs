pub mod ui;

fn main() {
    if let Err(err) = ui::rayso::ray() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
