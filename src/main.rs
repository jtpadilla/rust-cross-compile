use arm_cross_compile::qr_reader::QrReader;
use arm_cross_compile::tcp_server::TcpServer;

#[tokio::main]
async fn main() -> tokio_serial::Result<()> {

    env_logger::init();

    let mut reader = QrReader::new()?;

    let tcp_server = TcpServer::new(reader);

    Ok(())

}
