use rust_newsletter_system::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind the port");
    println!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());
    run(listener)?.await
}
