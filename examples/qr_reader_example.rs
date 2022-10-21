use log::info;
use arm_cross_compile::qr_reader;


#[tokio::main]
async fn main() -> tokio_serial::Result<()> {

    env_logger::init();

    let tty_path = qr_reader::get_tty_path();
    let serial_stream = qr_reader::get_serial_stream(&tty_path)?;

    // Se crea un Reader que asincronamente recibira lso eventos del puerto serie
    let mut reader = qr_reader::LineCodec.framed(serial_stream);

    // Bucle infinito para leer los datod que asincroonamente llegan por el reader
    info!("Se utilizara el puerto {}", tty_path);
    info!("Esperando datos del puerto serie...");
    while let Some(line_result) = reader.next().await {
        let line = line_result.expect("Failed to read line");
        println!("{}", line);
    }

    Ok(())

}
