use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio_rustls::{server::TlsStream, TlsAcceptor};
use tokio_tungstenite::tungstenite::Error;
use tokio_tungstenite::tungstenite::Message::{self, Binary};

pub struct TlsClient {
    writer: TlsClientWriter,
    reader: TlsClientReader
}

pub struct TlsClientWriter {
    writer: WriteHalf<TlsStream<TcpStream>>
}

pub struct TlsClientReader {
    reader: ReadHalf<TlsStream<TcpStream>>
}

impl TlsClient {
    pub async fn accept(incoming: TcpStream, acceptor: TlsAcceptor) -> Result<TlsClient, Error> {
        let (reader, writer) = io::split(acceptor.accept(incoming).await?);
        Ok(TlsClient { writer: TlsClientWriter{ writer }, reader: TlsClientReader{ reader } })
    }

    pub async fn send(&mut self, item: Message) -> Result<(), Error> {
        self.writer.send(item).await
    }

    pub async fn next(&mut self) -> Option<Result<Message, Error>> {
        self.reader.next().await
    }

    pub fn split(self) -> (TlsClientWriter, TlsClientReader) {
        (self.writer, self.reader)
    }
}

impl TlsClientWriter {
    pub async fn send(&mut self, item: Message) -> Result<(), Error> {
        if let Binary(binary) = item {
            self.writer.write(&binary).await?;
        }
        Ok(())
    }
}

impl TlsClientReader {
    pub async fn next(&mut self) -> Option<Result<Message, Error>> {
        let mut msg_buf = [0u8; 16];
        let len = match self.reader.read(&mut msg_buf).await {
            Err(_) => return Some(Err(Error::AlreadyClosed)),
            Ok(len) => len
        };
        Some(Ok(Binary(
            msg_buf[..len].to_vec()
        )))
    }
}