use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;

use aes::Aes128;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};


fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 65536];
    while match stream.read(&mut data) {
        Ok(size) => {
            // let text = from_utf8(&data[0..size]).unwrap();
            println!("From client data / size :: {:?}, {}", &data[0..size], size);

            stream.write(&data[0..size]);
            // stream.write(&data[0..size]).unwrap();
            true
        },
        Err(e) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server listening on port 3333");

    // Key generate
    let key = GenericArray::from([0u8; 16]);

    // Initialize cipher
    let cipher = Aes128::new(&key);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("new Connection: {}", stream.peer_addr().unwrap());
                stream.write(&key).unwrap(); // key send
                stream.flush();
                println!("key sent, {:?}", &key);

                // server key send to client
                thread::spawn(move || {
                    handle_client(stream);

                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    drop(listener);
}
