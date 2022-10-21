use log::debug;
use arm_cross_compile::qr_reader;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() -> tokio_serial::Result<()> {

    env_logger::init();

    let mut reader = qr_reader::create_reader()?;

    debug!("Esperando datos del puerto serie...");

    while let Some(line_result) = reader.next().await {
        let line = line_result.expect("Failed to read line");
        println!("{}", line);
    }

    Ok(())

}
