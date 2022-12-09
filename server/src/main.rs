//! Server
//! Facilitates online multiplayer, including lobbies and spectating matches

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

use argh::FromArgs;
use rustls_pemfile::{certs, rsa_private_keys};
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader},
    net::ToSocketAddrs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc::UnboundedSender,
};
use tokio_rustls::{
    rustls::{self, Certificate, PrivateKey},
    server::TlsStream,
    TlsAcceptor,
};

#[cfg(feature = "cppintegration")]
mod bindings;
mod connection;
mod lobby;

type Client = TlsStream<TcpStream>;
type Lobbies = HashMap<String, UnboundedSender<Client>>;

/// Command line options
#[derive(FromArgs)]
struct CLIOptions {
    /// address to bind to
    #[argh(positional)]
    address: String,

    /// certificate file
    #[argh(option, short = 'c')]
    certificate: PathBuf,

    /// key file
    #[argh(option, short = 'k')]
    key: PathBuf,
}

fn load_certs(path: &Path) -> io::Result<Vec<Certificate>> {
    certs(&mut BufReader::new(File::open(path)?))
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid cert"))
        .map(|certs| certs.into_iter().map(Certificate).collect())
}

fn load_keys(path: &Path) -> io::Result<Vec<PrivateKey>> {
    rsa_private_keys(&mut BufReader::new(File::open(path)?))
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid key"))
        .map(|keys| keys.into_iter().map(PrivateKey).collect())
}

/// Main loop: listens for connection requests, and creates a task to handle each requests
/// Uses a multithreaded asynchronous runtime
#[tokio::main]
async fn main() -> std::io::Result<()> {
    #[cfg(feature = "cppintegration")]
    println!("C++ integration enabled.");
    #[cfg(not(feature = "cppintegration"))]
    println!("C++ integration disabled.");

    #[cfg(feature = "use-certificate")]
    let (address, acceptor) = {
        let cli_options: CLIOptions = argh::from_env();
        let address = cli_options
            .address
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| io::Error::from(io::ErrorKind::AddrNotAvailable))?;
        let certificates = load_certs(&cli_options.certificate)?;
        let mut keys = load_keys(&cli_options.key)?;
        println!(
            "Successfully loaded {} certificates and {} keys.",
            certificates.len(),
            keys.len()
        );

        let config = rustls::ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(certificates, keys.remove(0))
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
        (address, TlsAcceptor::from(Arc::new(config)))
    };

    #[cfg(not(feature = "use-certificate"))]
    let address = "127.0.0.1:8081";

    let listener = TcpListener::bind(&address).await?;
    println!("Listening on {}", address);

    // "Global" storage of the lobbies in existence
    let lobbies = Arc::new(Mutex::new(Lobbies::new()));

    loop {
        // Wait for new connection requests
        let (incoming, _) = listener.accept().await?;
        // Handle the request
        let lobbies = Arc::clone(&lobbies);
        #[cfg(feature = "use-certificate")]
        let acceptor_clone = acceptor.clone();
        tokio::spawn(async move {
            if let Err(e) = connection::handle_connection(acceptor_clone, incoming, lobbies).await {
                println!("Client failed to connect with {}", e);
            }
        });
    }
}
