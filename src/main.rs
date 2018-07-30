#[macro_use]
extern crate log;
extern crate bytes;
extern crate futures;
extern crate pretty_env_logger;
extern crate tokio;
extern crate tokio_io;

use futures::{Future, Stream};
use tokio::net::{TcpListener, TcpStream};
use tokio_io::_tokio_codec::{Decoder, Framed};

mod codec;
mod error;

use std::fmt;

fn client_fut(
    framed_sock: Framed<TcpStream, codec::LinesCodec>,
) -> impl Future<Item = (), Error = ()> + 'static + Send {
    framed_sock
        .for_each(|line| {
            info!("Recv msg: {:?}", line);
            Ok(())
        })
        .map_err(|err| {
            error!("Decode failed: {:?}", err);
        })
}

fn server_fut(listener: TcpListener) -> impl Future<Item = (), Error = ()> + 'static + Send {
    listener
        .incoming()
        .for_each(|sock| {
            let _ = sock.peer_addr()
                .map(|peer_addr| {
                    info!("Tcp connection [{:?}] connected to server", peer_addr);
                })
                .map_err(|err| error!("Fetch peer addr failed: {:?}", err));

            let framed_sock = codec::LinesCodec::new().framed(sock);

            tokio::spawn(client_fut(framed_sock));
            Ok(())
        })
        .map_err(|err| {
            error!("Accept connection failed: {:?}", err);
        })
}

fn run() -> Result<(), error::Error> {
    let addr = "127.0.0.1:1234".parse()?;
    info!("Listening on {:?}", addr);

    let listener = TcpListener::bind(&addr)?;
    tokio::run(server_fut(listener));
    Ok(())
}

fn print<T: fmt::Debug>(result: Result<T, error::Error>) {
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
