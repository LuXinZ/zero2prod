use zero2prod::{run, configuration::{get_configuration}};
use std::net::TcpListener;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    

    run(listener)?.await
}
