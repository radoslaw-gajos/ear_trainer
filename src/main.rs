use ear_trainer::app::App;

#[tokio::main]
async fn main() {
    let app = App::new();
    app.run();
}
