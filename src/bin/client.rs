use std::net::{TcpStream};
use std::io::prelude::*;
use std::io::{Read, Write};
use std::str::from_utf8;

use aes::Aes256;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};


fn main() {
    // let stdin = std::io::stdin();
    // let stdin = stdin.lock();
    // for line in stdin.lines() {
    //     println!("{:?}", &line.unwrap());
    // }

    let key = GenericArray::from([0u8; 16]);
    let mut block = GenericArray::from([42u8; 16]);

    // Initialize cipher
    let cipher = Aes256::new(&key);



    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            // open stdin
            let stdin = std::io::stdin();
            let stdin = stdin.lock();
            for line in stdin.lines() {
                let msg = &line.unwrap();

                stream.write(msg.as_bytes()).unwrap();
                println!("Sent {}, awaiting reply...", msg);

                let mut data = vec![0 as u8; msg.len()]; // using dynamic length buffer

                match stream.read_exact(&mut data) {
                    Ok(_) => {
                        let text = from_utf8(&data).unwrap();

                        if text == msg {
                            println!("Reply is Ok!");
                        } else {
                            let text = from_utf8(&data).unwrap();
                            println!("Unexpected reply : {}", text);
                        }

                        // let text = from_utf8(&data).unwrap();
                        // println!("{}", text);
                    },
                    Err(e) => {
                        println!("FAiled to receive data: {}", e);
                    }
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }

    println!("Terminated.");
}
