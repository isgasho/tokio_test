#[macro_use]
extern crate log;
extern crate futures;
extern crate pretty_env_logger;
extern crate tokio;

use futures::future::{done, ok};
use futures::{Future, Stream};
use tokio::io::AsyncRead;
use tokio::net::{TcpListener, TcpStream};

use std::error;
use std::fmt;
use std::io;

fn client_fut(socket: TcpStream) -> impl Future<Item = (), Error = ()> + 'static + Send {
    futures::lazy(move || match socket.peer_addr() {
        Ok(peer) => {
            info!("Tcp connection [{:?}] connected to server", peer);
            Ok((socket, peer))
        }
        Err(err) => {
            error!("Fetch peer address failed: {:?}", err);
            Err(())
        }
    }).and_then(move |(socket, peer)| ok((socket.split(), peer)))
        .and_then(move |((ref mut r, ref mut w), peer)| {
            let loop_fut = done(io::copy(r, w))
                .and_then(move |num| {
                    info!("Wrote {:?} bytes", num);
                    ok(())
                })
                .map_err(|err| {
                    error!("Write failed: {:?}", err);
                });

            tokio::spawn(loop_fut);
            ok(())
        })
}

fn server_fut(listener: TcpListener) -> impl Future<Item = (), Error = ()> + 'static + Send {
    listener
        .incoming()
        .for_each(|socket| {
            tokio::spawn(client_fut(socket));
            Ok(())
        })
        .map_err(|err| {
            error!("Accept connection failed: {:?}", err);
        })
}

fn run() -> Result<(), io::Error> {
    let addr = "127.0.0.1:1234".parse().unwrap();
    info!("Listening on {:?}", addr);

    let listener = TcpListener::bind(&addr)?;
    let server_fut = server_fut(listener);

    tokio::run(server_fut);
    Ok(())
}

fn print<T: fmt::Debug, E: error::Error>(result: Result<T, E>) {
    match result {
        Ok(any) => info!("Result: {:?}", any),
        Err(err) => error!("Error: {:?}", err),
    }
}

fn init() {
    pretty_env_logger::init();
}

fn main() {
    init();
    print(run());
}
