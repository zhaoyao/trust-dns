// Copyright 2015-2016 Benjamin Fry <benjaminfry@me.com>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::io;
use std::net::SocketAddr;

use futures::Future;
use rustls::{Certificate, ClientConfig, ClientSession};
use tokio_core::net::TcpStream as TokioTcpStream;
use tokio_core::reactor::Handle;
use tokio_rustls::TlsStream as TokioTlsStream;

use trust_dns_proto::error::FromProtoError;
use trust_dns_proto::tcp::TcpClientStream;
use trust_dns_proto::xfer::{BufDnsStreamHandle, DnsStreamHandle};

use TlsStreamBuilder;

pub type TlsClientStream = TcpClientStream<TokioTlsStream<TokioTcpStream, ClientSession>>;

#[derive(Clone)]
pub struct TlsClientStreamBuilder(TlsStreamBuilder);

impl TlsClientStreamBuilder {
    /// Returns a new Builder for the TlsClientSteam
    pub fn new() -> Self {
        TlsClientStreamBuilder(TlsStreamBuilder::new())
    }

    /// Constructs a new TlsClientStreamBuilder with the associated ClientConfig
    pub fn with_client_config(client_config: ClientConfig) -> Self {
        TlsClientStreamBuilder(TlsStreamBuilder::with_client_config(client_config))
    }

    /// Add a custom trusted peer certificate or certificate auhtority.
    ///
    /// If this is the 'client' then the 'server' must have it associated as it's `identity`, or have had the `identity` signed by this certificate.
    pub fn add_ca(&mut self, ca: Certificate) {
        self.0.add_ca(ca);
    }

    /// Client side identity for client auth in TLS (aka mutual TLS auth)
    #[cfg(feature = "mtls")]
    pub fn identity(&mut self, pkcs12: Pkcs12) {
        self.0.identity(pkcs12);
    }

    /// Creates a new TlsStream to the specified name_server
    ///
    /// # Arguments
    ///
    /// * `name_server` - IP and Port for the remote DNS resolver
    /// * `dns_name` - The DNS name, Subject Public Key Info (SPKI) name, as associated to a certificate
    /// * `loop_handle` - The reactor Core handle
    pub fn build<E>(
        self,
        name_server: SocketAddr,
        dns_name: String,
        loop_handle: &Handle,
    ) -> (
        Box<Future<Item = TlsClientStream, Error = io::Error>>,
        Box<DnsStreamHandle<Error = E>>,
    )
    where
        E: FromProtoError + 'static,
    {
        let (stream_future, sender) = self.0.build(name_server, dns_name, loop_handle);

        let new_future: Box<Future<Item = TlsClientStream, Error = io::Error>> =
            Box::new(stream_future.map(move |tls_stream| TcpClientStream::from_stream(tls_stream)));

        let sender = Box::new(BufDnsStreamHandle::new(name_server, sender));

        (new_future, sender)
    }
}
