use std::{thread, time};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write, Error};
use std::str::{from_utf8};
use std::clone;

use aes::Aes256;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};

fn handle_client(mut clients: Vec<TcpStream>) {
    let mut data = [0 as u8; 65536];

    // 루프 돌면서 모든 클라이언트에 데이터를 전송
    loop {
        for stream in clients.iter() {
            let mut _stream = stream.try_clone().unwrap();

            match _stream.read(&mut data) {
                Ok(size) => {
                    println!("From client data :: {}", String::from_utf8_lossy(&data[0..size]));

                    for mut __stream in clients.iter() {
                        __stream.write(&data[0..size]);
                        // __stream.write_all(&data);
                        __stream.flush();
                    }
                }
                Err(e) => {}
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server listening on port 3333");

    // 키 생성
    let key = GenericArray::from([0u8; 32]);
    let mut clients:Vec<TcpStream> = Vec::new();

    for mut stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // 비동기 설정(스트림 읽기에서 블로킹 방지)
                stream.set_nonblocking(true);

                // 클라이언트에서 입력하는 닉네임 데이터 수신
                let mut nickname_data = [0 as u8; 1024];
                match stream.read(&mut nickname_data) {
                    Ok(size) => {
                        println!("Nickname {:?} entered.", from_utf8(&nickname_data[0..size]).unwrap());
                    },
                    Err(e) => {}
                }

                // 접속한 클라이언트에 키 내려줌
                stream.write(&key).unwrap(); // key send
                stream.flush();

                let cloned_stream = stream.try_clone().unwrap();
                clients.push(cloned_stream);

                // 클라이언트가 모두 접속(2개) 하면 쓰레드 실행
                if clients.len() == 2 {
                    // 클라이언트 TCP stream을 복제해서 스레드에서 실행할 수 있도록 전달
                    let mut cloned_clients = Vec::new();

                    for _stream in clients.iter() {
                        cloned_clients.push(_stream.try_clone().unwrap());
                    }

                    thread::spawn(move || {
                        handle_client(cloned_clients);
                    });
                }

                drop(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    drop(listener);
}
