use anyhow::{Context, Result};
use async_std::io::{Read, Write};
use async_std::net::{TcpListener, TcpStream};
use async_tls::TlsAcceptor;
use async_trait::async_trait;

use crate::network::tls::{get_tls_connector, get_tls_listener};
use crate::settings::Shared;

/// Windowsspecific cleanup handling when getting a SIGINT/SIGTERM.
pub fn socket_cleanup(_settings: &Shared) {}

/// This is a helper struct for TCP connections.
/// TCP should always be used in conjunction with TLS.
/// That's why this helper exists, which encapsulates the logic of accepting a new
/// connection and initializing the TLS layer on top of it.
/// This way we can expose an `accept` function and implement the GenericListener trait.
pub struct TlsTcpListener {
    tcp_listener: TcpListener,
    tls_acceptor: TlsAcceptor,
}

/// A new trait, which can be used to represent Unix- and TcpListeners.
/// This is necessary to easily write generic functions where both types can be used.
#[async_trait]
pub trait GenericListener: Sync + Send {
    async fn accept<'a>(&'a self) -> Result<GenericStream>;
}

#[async_trait]
impl GenericListener for TlsTcpListener {
    async fn accept<'a>(&'a self) -> Result<GenericStream> {
        let (stream, _) = self.tcp_listener.accept().await?;
        Ok(Box::new(self.tls_acceptor.accept(stream).await?))
    }
}

/// A new trait, which can be used to represent Unix- and Tls encrypted TcpStreams.
/// This is necessary to write generic functions where both types can be used.
pub trait Stream: Read + Write + Unpin + Send {}
impl Stream for async_tls::server::TlsStream<TcpStream> {}
impl Stream for async_tls::client::TlsStream<TcpStream> {}

/// Two convenient types, so we don't have type write Box<dyn ...> all the time.
pub type Listener = Box<dyn GenericListener>;
pub type GenericStream = Box<dyn Stream>;

/// Get a new stream for the client.
/// This can either be a UnixStream or a Tls encrypted TCPStream, depending on the parameters.
pub async fn get_client_stream(settings: &Shared) -> Result<GenericStream> {
    // Connect to the daemon via TCP
    let address = format!("{}:{}", settings.host, settings.port);
    let tcp_stream = TcpStream::connect(&address).await.context(format!(
        "Failed to connect to the daemon on {}. Did you start it?",
        &address
    ))?;

    // Get the configured rustls TlsConnector
    let tls_connector = get_tls_connector(&settings)
        .await
        .context("Failed to initialize TLS Connector")?;

    // Initialize the TLS layer
    let stream = tls_connector
        .connect("pueue.local", tcp_stream)
        .await
        .context("Failed to initialize TLS stream")?;

    Ok(Box::new(stream))
}

/// Get a new tcp&tls listener for the daemon.
pub async fn get_listener(settings: &Shared) -> Result<Listener> {
    // This is the listener, which accepts low-level TCP connections
    let address = format!("{}:{}", settings.host, settings.port);
    let tcp_listener = TcpListener::bind(&address)
        .await
        .context(format!("Failed to listen on address: {}", address))?;

    // This is the TLS acceptor, which initializes the TLS layer
    let tls_acceptor = get_tls_listener(&settings)?;

    // Create a struct, which accepts connections and initializes a TLS layer in one go.
    let tls_listener = TlsTcpListener {
        tcp_listener,
        tls_acceptor,
    };

    Ok(Box::new(tls_listener))
}
