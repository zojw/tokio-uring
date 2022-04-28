use std::{env, net::SocketAddr, time::Duration};

use tokio_uring::net::TcpListener;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() <= 1 {
        panic!("no addr specified");
    }

    let socket_addr: SocketAddr = args[1].parse().unwrap();

    tokio_uring::start(async {
        let listener = TcpListener::bind(socket_addr).unwrap();

        println!("listen: {}", args[1]);

        loop {
            let (stream, socket_addr) = listener.accept().await.unwrap();
            tokio_uring::spawn(async move {
                loop {
                    let buf = vec![1u8; 128];

                    let fut = tokio::time::timeout(Duration::from_secs(10), stream.read(buf));

                    let res = fut.await;

                    if res.is_err() {
                        println!("wait read timeouted, and close connection???");
                        return;
                    }

                    let (result, buf) = res.unwrap();

                    let read = result.unwrap();
                    println!("read from {}: {:?}", socket_addr, &buf[..read]);
                }
            });
        }
    });
}
