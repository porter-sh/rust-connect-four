//! Contains the TlsClient struct and its split TlsClientReader and TlsClientWriter
//! Provides an abstraction over tokio and tokio::rust_ls to match the futures trait interfaces
//! used when feature use-certificate is not activated

/*
 * This file is part of Rust-Connect-Four
 * Copyright (C) 2022 Alexander Broihier <alexanderbroihier@gmail.com>
 * Copyright (C) 2022 Porter Shawver <portershawver@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio_rustls::{server::TlsStream, TlsAcceptor};
use tokio_tungstenite::tungstenite::Error;
use tokio_tungstenite::tungstenite::Message::{self, Binary};

/// A TlsClient to communicate with
pub struct TlsClient {
    writer: TlsClientWriter,
    reader: TlsClientReader
}

/// Write half of the TlsClient
pub struct TlsClientWriter {
    writer: WriteHalf<TlsStream<TcpStream>>
}

/// Read half of the TlsClient
pub struct TlsClientReader {
    reader: ReadHalf<TlsStream<TcpStream>>
}

impl TlsClient {
    /// Tries to create a TlsClient from an incoming TcpStream and a TlsAcceptor
    /// If successful, returns the TlsClient as a result
    pub async fn accept(incoming: TcpStream, acceptor: TlsAcceptor) -> Result<TlsClient, Error> {
        let (reader, writer) = io::split(acceptor.accept(incoming).await?);
        Ok(TlsClient { writer: TlsClientWriter{ writer }, reader: TlsClientReader{ reader } })
    }

    /// Sends a message through the TlsStream
    pub async fn send(&mut self, item: Message) -> Result<(), Error> {
        self.writer.send(item).await
    }

    /// Gets the next message from the TlsStream
    /// Always returns Some variant, Option returned to mirror futures library
    pub async fn next(&mut self) -> Option<Result<Message, Error>> {
        self.reader.next().await
    }

    /// Splits the TlsClient into its writer and reader components
    pub fn split(self) -> (TlsClientWriter, TlsClientReader) {
        (self.writer, self.reader)
    }
}

impl TlsClientWriter {
    /// Sends a message through the TlsStream
    pub async fn send(&mut self, item: Message) -> Result<(), Error> {
        if let Binary(binary) = item {
            self.writer.write(&binary).await?;
        }
        Ok(())
    }
}

impl TlsClientReader {
    /// Gets the next message from the TlsStream
    /// Always returns Some variant, Option returned to mirror futures library
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