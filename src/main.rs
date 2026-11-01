use playman_rust::platform::window::{config, run};
#[macroquad::main(config)]
async fn main() {
    run().await;
}
