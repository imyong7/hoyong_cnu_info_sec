use std::net::{TcpStream};
use std::io::prelude::*;
use std::io::{Read, Write, Error};
use std::str::{from_utf8, FromStr};

use aes::{Aes256, Block};
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};
use std::thread;


// 서버에서 데이터를 받아서 복호화 하는 함수 - 스레드 생성해서 실행
fn handle_receive(mut stream: TcpStream, cipher: Aes256, nickname: String) {
    let mut data = vec![0 as u8; 1024]; // 수신할 데이터 블록 길이 설정

    while match stream.read(&mut data) {
        Ok(size) => {
            let mut _sliced_block = data.as_slice(); // 복호화된 배열을 형변환 수행
            if let Some(index) = _sliced_block.iter().position(|&i| i == 0) {
                _sliced_block = &_sliced_block[0..index];
            }

            let msg_bytes = _sliced_block;
            let msg_len = msg_bytes.len();
            let msg_split_blocks = ((msg_len / 16) + 1);

            // 복호화 할 블록 어레이 선언
            let mut blocks = vec![];

            // 길이 :: 16으로 버퍼로 잘라서 복호화 수행
            for i in 0..msg_split_blocks {
                let mut split_unit = 16;

                if msg_len - (i * 16) > 16 {
                    split_unit = 16;
                } else {
                    split_unit = msg_len - (i * 16);
                }

                let mut buffer = [0u8; 16]; // 복호화 할 버퍼 생성
                let split_msg = &msg_bytes[(i*16)..(i*16 + split_unit)];
                buffer[..split_unit].copy_from_slice(split_msg); // 버퍼에 문자 길이만큼 할당

                let mut block = GenericArray::from( buffer ); // 버퍼를 복호화용 블록 어레이로 변환
                blocks.push(block);
            }

            // 블록 복호화
            cipher.decrypt_blocks(&mut blocks);

            // 블록의 형타입 변환
            let mut _block = [0 as u8; 1024]; // 블록 버퍼 zero-fill
            let mut index = 0;
            for block in blocks.iter() { // 배열에 블록 바이트 값 할당
                for byte in block.iter() {
                    _block[index] = *byte;
                    index = index + 1;
                }
            }

            // zero-fill 된 어레이 trimming
            let mut _dec_sliced_block = _block.as_slice(); // 복호화된 배열을 형변환 수행
            if let Some(index) = _dec_sliced_block.iter().position(|&i| i == 0) {
                _dec_sliced_block = &_dec_sliced_block[0..index];
            }

            // 바이트 배열을 합쳐서 문자열로 변환
            let dec_text = from_utf8(_dec_sliced_block).unwrap(); // 복호화된 배열을 문자열로 변환
            println!("{}", dec_text);

            true
        },
        Err(e) => {
            println!("Failed to receive data: {}", e);
            false
        }
    } {}
}

fn main() {
    println!("Enter nickname...");
    let mut nickname = String::new(); // 닉네임 설정

    let stdin = std::io::stdin();
    stdin.lock().read_line(&mut nickname).unwrap();
    nickname = nickname.replace('\n', "").replace('\r', "");
    println!("Nickname entered ==> {}", nickname);

    let _nickname = nickname.clone();

    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            stream.write_all(nickname.as_bytes()).unwrap(); // 서버로 닉네임 들어왔다고 전송
            println!("Successfully connected to server in port 3333");

            // 키 데이터, 키, AES256 선언
            let mut keydata = GenericArray::from([0u8; 32]);
            let mut key = GenericArray::from([0u8; 32]);
            let mut cipher = Aes256::new(&keydata);
            let mut _cipher = Aes256::new(&keydata);

            match stream.read(&mut keydata) {
                Ok(_) => {
                    // 서버로부터 키 정보를 받아서 변수 할당
                    key = keydata;
                    cipher = Aes256::new(&keydata);
                    _cipher = Aes256::new(&keydata);
                    // println!("receive a key from server : {:?}", key);
                }
                Err(_) => {
                    println!("error");
                }
            }

            let receive_stream = stream.try_clone().unwrap();

            thread::spawn(move || {
                handle_receive(receive_stream, _cipher, _nickname);
            });

            let stdin = std::io::stdin();
            let stdin = stdin.lock();
            for line in stdin.lines() { // 키보드 입력, 엔터 값 구분으로 값 불러옴

                // WORKING CODE!!!!! => multiple block
                let mut msg = format!("[{}] {}", nickname, &line.unwrap()); // 입력값을 문자열 변수로 할당
                let msg_bytes = msg.as_bytes(); // 문자열을 바이트(배열)로 선언
                let msg_len = msg_bytes.len();
                let msg_split_blocks = ((msg_len / 16) + 1);

                // 암호화 할 블록 어레이 선언
                let mut blocks = vec![];

                // 길이 :: 16으로 버퍼로 잘라서 블록 암호화 수행
                for i in 0..msg_split_blocks {
                    let mut split_unit = 16;

                    if msg_len - (i * 16) > 16 {
                        split_unit = 16;
                    } else {
                        split_unit = msg_len - (i * 16);
                    }

                    let mut buffer = [0u8; 16]; // 암호화 할 버퍼 생성
                    let split_msg = &msg_bytes[(i*16)..(i*16 + split_unit)];
                    buffer[..split_unit].copy_from_slice(split_msg); // 버퍼에 문자 길이만큼 할당

                    let mut block = GenericArray::from( buffer ); // 버퍼를 암호화용 블록 어레이로 변환
                    blocks.push(block);
                }

                cipher.encrypt_blocks(&mut blocks);

                // 블록의 형타입 변환 후 서버로 전송
                let mut _block = [0 as u8; 1024]; // 블록 버퍼 zero-fill
                let mut index = 0;
                for block in blocks.iter() { // 배열에 블록 바이트 값 할당
                    for byte in block.iter() {
                        _block[index] = *byte;
                        index = index + 1;
                    }
                }

                // zero-fill 된 어레이 trimming
                let mut _sliced_block = _block.as_slice(); // 복호화된 배열을 형변환 수행
                if let Some(index) = _sliced_block.iter().position(|&i| i == 0) {
                    _sliced_block = &_sliced_block[0..index];
                }

                // 암호화된 블록 전송
                stream.write_all(&_sliced_block).unwrap(); // 서버로 전송
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }

    println!("Terminated.");
}
