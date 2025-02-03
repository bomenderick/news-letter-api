use news_letter_api::run;
use std::net::TcpListener;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let tcp_listener = TcpListener::bind("127.0.0.1:8080")?;
    run(tcp_listener)?.await
}
