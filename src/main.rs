extern crate tokio;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;
use std::io::Error;
use std::fmt::Debug;


fn run() -> Result<(), Error> {
    // Bind the server's socket
    let addr = "127.0.0.1:1234".parse().unwrap();
    let tcp = TcpListener::bind(&addr)?;

    // Iterate incoming connections
    let server = tcp.incoming().for_each(|tcp| {
        // Split up the read and write halves
        let (reader, writer) = tcp.split();

        // Copy the data back to the client
        let conn = io::copy(reader, writer)
            // print what happened
            .map(|(n, _, _)| {
                println!("wrote {} bytes", n)
            })
            // Handle any errors
            .map_err(|err| {
                println!("IO error {:?}", err)
            });

        // Spawn the future as a concurrent task
        tokio::spawn(conn);

        Ok(())
    })
        .map_err(|err| {
            println!("server error {:?}", err);
        });

    // Start the runtime and spin up the server
    tokio::run(server);
    Ok(())
}

fn print<T: Debug>(result: Result<T, Error>) {
    match result {
        Ok(any)  => println!("{:?}", any),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn main() {
    print(run())
}
