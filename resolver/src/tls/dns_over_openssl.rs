// Copyright 2015-2018 Benjamin Fry <benjaminfry@me.com>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![cfg(feature = "dns-over-openssl")]
#![allow(dead_code)]

use std::io;
use std::net::SocketAddr;

use futures::Future;
use tokio_core::reactor::Handle;

use trust_dns_openssl::{TlsClientStream, TlsClientStreamBuilder};
use trust_dns_proto::DnsStreamHandle;

use error::*;

pub(crate) fn new_tls_stream(
    socket_addr: SocketAddr,
    dns_name: String,
    loop_handle: &Handle,
) -> (
    Box<Future<Item = TlsClientStream, Error = io::Error>>,
    Box<DnsStreamHandle<Error = ResolveError>>,
) {
    let tls_builder = TlsClientStreamBuilder::new();
    tls_builder.build(socket_addr, dns_name, loop_handle)
}
