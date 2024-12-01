use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

const LOCAL: &str = "127.0.0.1:6000"; //local host with port
const MSSG_SIZE: usize = 32; //buffer size for message

fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
}

fn main() {
    let server = TcpListener::bind(LOCAL).expect("Failed to bind");
    server
        .set_nonblocking(true)
        .expect("Failed to set non-blocking");

    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<String>();

    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client connected at {}", addr);

            let tx = tx.clone();
            clients.push(socket.try_clone().expect("Failed to clone client"));

            thread::spawn(move || loop {
                let mut buff_u8 = vec![0u8; MSSG_SIZE]; // Temporary buffer for u8
                match socket.read_exact(&mut buff_u8) {
                    Ok(_) => {
                        // Convert the buffer to usize
                        let buff_usize: Vec<usize> = buff_u8.iter().map(|&x| x as usize).collect();

                        let msg = buff_usize
                            .into_iter()
                            .take_while(|&x| x != 0)
                            .map(|x| x as u8)
                            .collect::<Vec<u8>>();

                        let msg = String::from_utf8(msg).expect("Invalid UTF-8 message");
                        println!("{}: {:?}", addr, msg);
                        tx.send(msg).expect("Failed to send message to rx");
                    }
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("Closing connection with: {}", addr);
                        break;
                    }
                }

                sleep();
            });
        }

        if let Ok(msg) = rx.try_recv() {
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    let mut buff = msg.clone().into_bytes();
                    buff.resize(MSSG_SIZE, 0);

                    client.write_all(&buff).map(|_| client).ok()
                })
                .collect::<Vec<_>>();
        }

        sleep();
    }
}
