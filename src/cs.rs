#![allow(non_snake_case)]

use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, sync::{Arc, Mutex}, thread};

pub fn server() {
    let x = TcpListener::bind("0.0.0.0:8080").unwrap();
    let mutx = Arc::new(Mutex::new([0; 1024]));
    println!("Listening on http://127.0.0.1:8080");

    for stream in x.incoming() {
        match stream {
            Ok(mut strm) => {
                let x = mutx.clone();
                thread::spawn(move || {
                    let mut y = *x.lock().unwrap();
                    let sz = strm.read(&mut y).unwrap();
                    println!("Received: {:?}", String::from_utf8_lossy(&y[..sz]));

                    let response = b"HTTP/1.1 200 OK\r\n\r\nHello, World!";
                    strm.write_all(response).unwrap();
                });
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}


pub fn client() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();

    let request = b"GET / HTTP/1.1\r\nHost: 127.0.0.1:8080\r\nConnection: keep-alive\r\nThe phone rings...
but nobody came\r\n\r\n";
    stream.write_all(request).unwrap();
    
    let mut response = [0; 1024];
    let size = stream.read(&mut response).unwrap();
    println!("Response: {:?}", String::from_utf8_lossy(&response[..size]));
}
