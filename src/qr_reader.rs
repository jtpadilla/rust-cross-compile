
use std::{env, io, str};
use tokio_util::codec::{Decoder, Encoder};

use bytes::BytesMut;
use tokio_serial::{self, SerialStream, SerialPortBuilderExt, Error};

const DEFAULT_TTY: &str = "/dev/ttyACM0";

pub struct LineCodec;

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

pub fn get_tty_path() -> String {
    let mut args = env::args();
    args.nth(1).unwrap_or_else(|| DEFAULT_TTY.into())
}

pub fn get_serial_stream(tty_path: &str) -> Result<SerialStream, Error> {

    // Se construye un builder donde se nos requiere ademas la velocidad
    let builder = tokio_serial::new(tty_path, 115200);

    // Con el builder se termina de configurar y se crea el stream
    let mut serial_stream = builder.baud_rate(115200)
        .stop_bits(tokio_serial::StopBits::One)
        .data_bits(tokio_serial::DataBits::Eight)
        .parity(tokio_serial::Parity::None)
        .flow_control(tokio_serial::FlowControl::None)
        .open_native_async()?;

    // Se stream se configura para su uso exclusivo
    serial_stream.set_exclusive(false)
        .expect("Unable to set serial port exclusive to false");

    // Resultado
    Ok(serial_stream)

}

pub fn popo() {

    let serialStream = get_serial_stream("");

    let l = LineCodec{};

    l.framed(serialStream);


    let mut reader = LineCodec.framed(serial_stream);

}

