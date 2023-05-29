use std::net::{TcpStream};
use std::io::prelude::*;
use std::io::{Read, Write, Error};
use std::str::{from_utf8, FromStr};

use rsa::{RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt};
use aes::{Aes256, Block};
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};
use std::{thread, time};
use rand::Rng;
use rsa::traits::PublicKeyParts;
use std::hash::Hash;
use rsa::pkcs8::{EncodePublicKey, DecodePublicKey};


// 서버에서 데이터를 받아서 복호화 하는 함수 - 스레드 생성해서 실행
/*
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
*/

fn handle_receive(mut stream: TcpStream, mut cipher: Aes256, nickname: String) {
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
    /*
    // RSA 키 생성 시퀀스 시작
    let mut rng = rand::thread_rng();
    let bits = 2048; // RSA 비트 수 2048

    // 개인키 생성
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");

    // 공개키 생성
    let public_key = RsaPublicKey::from(&private_key);
    println!("public_key bytes :: {:?}, {:?}", public_key.to_public_key_der().unwrap().as_bytes(), public_key.to_public_key_der().unwrap().as_bytes().len());
    println!("public_key data :: {:?}",  RsaPublicKey::from_public_key_der(public_key.to_public_key_der().unwrap().as_bytes()) );

    // 상대 클라이언트측에서 복호화된 AES KEY
    // let mut rsa_dec_aes_key;

    // 대칭키(AES256) 생성
    let mut keydata = GenericArray::from([0u8; 32]);
    let mut key = GenericArray::from([0u8; 32]);
    let mut cipher = Aes256::new(&key);
    let mut _cipher = Aes256::new(&key);

    // let mut recv_cipher;
    // let mut _recv_cipher;

    // 공개키로 AES 키 암호화
    let enc_aes_key = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, &key).expect("failed to encrypt");
    println!("enc_aes_key :: {:?}, {:?}", enc_aes_key, enc_aes_key.len());
    */

    // WORKING CODE !!!!!
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

            // loop {
            let mut client_data = vec![0 as u8; 1];
            match stream.read(&mut client_data) {
                Ok(_) => {
                    if client_data[0] == 2 {
                        println!("{:?} clients connected!\n", client_data);
                        // break;
                    }
                }
                Err(_) => {
                    // println!("error");
                }
            }
            // }

            println!("Key distribution in progress...");
            // RSA 키 생성 시퀀스 시작
            let mut rng = rand::thread_rng();
            let bits = 2048; // RSA 비트 수 2048

            // 개인키 생성
            let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");

            // 공개키 생성
            let public_key = RsaPublicKey::from(&private_key);
            let mut recv_public_key = RsaPublicKey::from(&private_key);
            let mut public_key_bytes = public_key.to_public_key_der().unwrap();

            // 공개키 전송
            stream.write_all(&public_key_bytes.as_bytes());
            stream.flush();

            // 수신한 상대방의 공개키 수신
            let mut recv_rsa_pub_key_data = [0 as u8; 294];
            // loop {
            match stream.read(&mut recv_rsa_pub_key_data) {
                Ok(_) => {
                    if public_key_bytes.as_bytes() != recv_rsa_pub_key_data {
                        println!("recv_public_key :: {:?}, {:?}\n", recv_rsa_pub_key_data, recv_rsa_pub_key_data.len());

                        // 수신받은 공개키로 교체
                        recv_public_key = RsaPublicKey::from_public_key_der(&*recv_rsa_pub_key_data.to_vec()).unwrap();
                        println!("public_key :: {:?}, {:?}\n", &public_key_bytes.as_bytes(), &public_key_bytes.as_bytes().len());
                        // break;
                    }
                }
                Err(_) => {
                    println!("error");
                }
            }
            // }
            // thread::sleep(time::Duration::from_millis(3000));

            // 상대 클라이언트측에서 복호화된 AES KEY
            // let mut rsa_dec_aes_key= GenericArray::from([0u8; 32]);
            let mut rsa_dec_aes_key = GenericArray::from([0u8; 32]);

            // 대칭키(AES256) 생성
            let key_rng_bytes = rand::thread_rng().gen::<[u8; 32]>(); // 랜덤 난수 생성

            // let mut keydata = GenericArray::from([0u8; 32]);
            let mut key = GenericArray::from(key_rng_bytes);
            let mut cipher = Aes256::new(&key);
            let mut _cipher = Aes256::new(&key);

            // 초기는 dummy로 생성
            let mut recv_cipher:Aes256 = Aes256::new(&key);

            // 상대 공개키로 AES 키 암호화
            let rsa_enc_aes_key = recv_public_key.encrypt(&mut rng, Pkcs1v15Encrypt, &key).expect("failed to encrypt");
            println!("rsa_enc_aes_key :: {:?}\n", rsa_enc_aes_key);

            // 공개키로 암호화한 비밀키 전송
            stream.write_all(&rsa_enc_aes_key);
            stream.flush();
            // thread::sleep(time::Duration::from_millis(3000));

            // 교환된 상대방의 대칭키 수신
            let mut recv_key_data = vec![0 as u8; 256];
            // loop {
            match stream.read(&mut recv_key_data) {
                Ok(_) => {
                    if rsa_enc_aes_key != recv_key_data {
                        println!("recv_key_data :: {:?}, {:?}\n", recv_key_data, recv_key_data.len());

                        // 개인키로 복호화
                        let dec_key = private_key.decrypt(Pkcs1v15Encrypt, &recv_key_data).expect("failed to decrypt");

                        // 키 데이터를 암호화에 맞는 형으로 변환
                        let mut index = 0;
                        for block in dec_key.iter() { // 배열에 블록 바이트 값 할당
                            rsa_dec_aes_key[index] = *block;
                            index = index + 1;
                        }

                        println!("dec_key :: {:?}, {:?}\n", dec_key, dec_key.len());
                        println!("rsa_dec_aes_key :: {:?}, {:?}\n", rsa_dec_aes_key, rsa_dec_aes_key.len());

                        // 서버에서 수신한 키로 수신용 복호화 AES 클래스 선언
                        recv_cipher = Aes256::new(&rsa_dec_aes_key);
                        // break;
                    }
                }
                Err(_) => {
                    println!("error");
                }
            }
            // }
            println!("Key distribution completed...");

            //

            let receive_stream = stream.try_clone().unwrap();

            thread::spawn(move || {
                // handle_receive(receive_stream, _cipher, _nickname);
                handle_receive(receive_stream, recv_cipher, _nickname);
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
