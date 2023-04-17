use std::net::{TcpStream};
use std::io::prelude::*;
use std::io::{Read, Write, Error};
use std::str::{from_utf8, FromStr};

use aes::Aes256;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};
use generic_array::typenum::U32;
use std::convert::TryInto;


fn main() {
    // let stdin = std::io::stdin();
    // let stdin = stdin.lock();
    // for line in stdin.lines() {
    //     println!("{:?}", &line.unwrap());
    // }

    let key = GenericArray::from([0u8; 32]);
    let mut block = GenericArray::from([42u8; 32]);

    // Initialize cipher
    let cipher = Aes256::new(&key);



    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            let mut keydata = GenericArray::from([0u8; 32]);
            let mut key = GenericArray::from([0u8; 32]);
            let mut cipher = Aes256::new(&keydata);

            match stream.read(&mut keydata) {
                Ok(_) => {
                    key = keydata;
                    cipher = Aes256::new(&keydata);
                    println!("receive a key from server : {:?}", key);
                }
                Err(_) => {
                    println!("errr");
                }
            }

            // open stdin
            let stdin = std::io::stdin();
            let stdin = stdin.lock();
            for line in stdin.lines() {
                /*
                let msg = &line.unwrap();

                stream.write(msg.as_bytes()).unwrap();
                println!("Sent {}, awaiting reply...", msg);

                let mut data = vec![0 as u8; msg.len()]; // using dynamic length buffer
                */
// /*
                let msg = &line.unwrap();
                let x = msg.as_bytes();
                println!("msg bytes :: {:?}", x);
                println!("msg bytes :: {:?}", &x[..]);

                let mut encrypted:GenericArray<u8, U32> =  GenericArray::from(&x[..x.len()]);
                // println!("{:?}", encrypted);


                // let mut encrypted: _ = msg.into_bytes()
                //     .chunks_mut(16)
                //     .map(|_| GenericArray::from_slice)
                //     .collect(); // <-- This line fails to compile

                // let mut encrypted = GenericArray::from([42u8; 16]);
                //
                // println!("encrypted :: {:?}", encrypted);
                //
                // cipher.encrypt_block(&mut encrypted);


                // println!("msg_block :: {:?}", msg_block);

                // let mut enc_msg = cipher.encrypt_block(&mut msg_block);

                // println!("enc_msg :: {:?}", &enc_msg);

                // stream.write(enc_msg).unwrap();
                // println!("Sent {}, Ciphered Text : {:?},  awaiting reply...", msg, enc_msg);

                stream.write(msg.as_bytes()).unwrap();
                let mut data = vec![0 as u8; msg.len()]; // using dynamic length buffer
// */
                match stream.read_exact(&mut data) {
                    Ok(_) => {
                        let text = from_utf8(&data).unwrap();
                        println!("From server text : {}", text);


                        if text == msg {
                            println!("Reply is Ok!");
                        } else {
                            // let text = from_utf8(&data).unwrap();
                            println!("Unexpected reply : {}", text);
                        }

                        // let text = from_utf8(&data).unwrap();
                        // println!("{}", text);
                    },
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
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
