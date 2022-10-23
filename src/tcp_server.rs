
use tokio::net::{TcpListener, TcpStream};
use crate::qr_reader::QrReader;

pub struct TcpServer {
    qr_reader: QrReader
}

impl TcpServer {

    pub fn new(qr_reader: QrReader) -> TcpServer {
        TcpServer {
            qr_reader: qr_reader
        }
    }

    pub async fn accept(&mut self) {

        // Se enlaza un listener con la direccion y el puerto
        let listener = TcpListener::bind("127.0.0.1:2222").await.unwrap();

        loop {
            // La segunda entrada de la tupla contiene la direccion y el puerto del origen
            let (socket, _) = listener.accept().await.unwrap();
            self.process(socket).await;
        }


    }

    async fn process(&mut self, socket: TcpStream) {
        while let Some(line_result) = self.qr_reader.read_line().await {
            let line = line_result.expect("Failed to read line");
            println!("{}", line);
        }
    }
    

}

