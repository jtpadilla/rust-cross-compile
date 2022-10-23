use arm_cross_compile::qr_reader::QrReader;
use arm_cross_compile::tcp_server::TcpServer;

#[tokio::main]
async fn main() -> tokio_serial::Result<()> {

    env_logger::init();

    let mut reader = QrReader::new()?;

    let tcp_server = TcpServer::new(reader);

/*    
    while let Some(line_result) = reader.read_line().await {
        let line = line_result.expect("Failed to read line");
        println!("{}", line);
    }
*/

    Ok(())

}
