use arm_cross_compile::qr_reader;

#[tokio::main]
async fn main() -> tokio_serial::Result<()> {

    env_logger::init();

    let mut reader = qr_reader::QrReader::new()?;

    while let Some(line_result) = reader.read_line().await {
        let line = line_result.expect("Failed to read line");
        println!("{}", line);
    }

    Ok(())

}
