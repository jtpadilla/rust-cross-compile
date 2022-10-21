
use log::{info};
use futures::stream::StreamExt;
use std::{env, io, str};
use tokio_util::codec::{Decoder, Encoder};

use bytes::BytesMut;
use tokio_serial::{self, SerialPortBuilderExt};

const DEFAULT_TTY: &str = "/dev/ttyACM0";

struct LineCodec;

impl Decoder for LineCodec {

    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let newline = src.as_ref().iter().position(|b| *b == b'\n');
        if let Some(n) = newline {
            let line = src.split_to(n + 1);
            return match str::from_utf8(line.as_ref()) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Invalid String")),
            };
        }
        Ok(None)
    }
}

impl Encoder<String> for LineCodec {
    type Error = io::Error;

    fn encode(&mut self, _item: String, _dst: &mut BytesMut) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[tokio::main]
async fn main() -> tokio_serial::Result<()> {

    // Se activa el log
    env_logger::init();

    // Se obtiene el nombre del device (puerto serie) que se utilizara
    let mut args = env::args();
    let tty_path = args.nth(1).unwrap_or_else(|| DEFAULT_TTY.into());
    info!("Se utilizara el puerto {}", tty_path);
    
    // Se construye un builder donde se nos requiere ademas la velocidad
    let builder = tokio_serial::new(tty_path, 115200);

    // Mediante el builder del serie se termina de configurar y
    // finalmente se crea el stream
    let mut serial_stream = builder.baud_rate(115200)
        .stop_bits(tokio_serial::StopBits::One)
        .data_bits(tokio_serial::DataBits::Eight)
        .parity(tokio_serial::Parity::None)
        .flow_control(tokio_serial::FlowControl::None)
        .open_native_async()?;

    // Se configura el stream para acceder de forma exclusiva
    serial_stream.set_exclusive(false)
        .expect("Unable to set serial port exclusive to false");

    // Se crea un Reader que asincronamente recibira lso eventos del puerto serie
    let mut reader = LineCodec.framed(serial_stream);

    // Bucle infinito para leer los datod que asincroonamente llegan por el reader
    info!("Esperando datos del puerto serie...");
    while let Some(line_result) = reader.next().await {
        let line = line_result.expect("Failed to read line");
        println!("{}", line);
    }

    Ok(())

}
