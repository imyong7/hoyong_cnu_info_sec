use std::net::{TcpStream};
use std::io::prelude::*;
use std::io::{Read, Write, Error};
use std::str::{from_utf8, FromStr};

use aes::{Aes128, Block};
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};
use generic_array::typenum::U32;
use std::convert::TryInto;
use std::any::type_name;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            let mut keydata = GenericArray::from([0u8; 16]);
            let mut key = GenericArray::from([0u8; 16]);
            let mut cipher = Aes128::new(&keydata);

            match stream.read(&mut keydata) {
                Ok(_) => {
                    key = keydata;
                    cipher = Aes128::new(&keydata);
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
                let msg = &line.unwrap();
                let x = msg.as_bytes();

                println!("msg bytes :: {:?}, {}", x, x.len());
                println!("msg bytes :: {:?}, {}", &x[..], x.len());

                let mut buffer = [0u8; 16];
                let pos = x.len();
                buffer[..pos].copy_from_slice(x);

                let mut block = GenericArray::from( buffer );

                let block_copy = block.clone();

                println!("before encrypt block :: {:?}, {}", &block, block.len());
                cipher.encrypt_block(&mut block);
                println!("after encrypt block :: {:?}, {}", &block, block.len());

                stream.write(&block).unwrap();
                let mut data = vec![0 as u8; block.len()]; // using dynamic length buffer

                match stream.read_exact(&mut data) {
                    Ok(_) => {
                        let enc_text = &data;
                        println!("--- data from server ---");
                        println!("enc_text array :: {:?}, {}", enc_text, enc_text.len());
                        let mut enc_buffer = [0u8; 16];
                        let enc_pos = data.len();
                        enc_buffer[..enc_pos].copy_from_slice(&data);

                        let mut enc_block = GenericArray::from( enc_buffer );

                        cipher.decrypt_block(&mut enc_block);
                        println!("after decrypt block :: {:?}, {}", &enc_block, enc_block.len());

                        let mut dec_sliced = enc_block.as_slice();
                        println!("dec_sliced before retaining :: {:?}", dec_sliced);

                        if let Some(index) = dec_sliced.iter().position(|&i| i == 0) {
                            // dec_sliced.remove(index);
                            println!("{:?}", &dec_sliced[0..index]);
                            // dec_buffer[..index].copy_from_slice(&dec_sliced[0..index]);
                            dec_sliced = &dec_sliced[0..index];
                        }
                        println!("dec_buffer after filtering :: {:?}", dec_sliced);

                        let dec_text = from_utf8(dec_sliced).unwrap();

                        println!("dec_buffer to string ==> {}", dec_text);


                        if msg == dec_text {
                            println!("Reply is Ok!");
                        } else {
                            println!("Unexpected reply : {:?}", dec_text);
                        }
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
