
use std::collections::VecDeque;
use std::{thread, time};
use std::sync::{mpsc};
use rand::RngExt;
use crossterm::event::{self, Event};

use super::udp_sockets::{create_socket,create_sender_udp_port,create_listen_udp_port_3615};
use super::convert_byte_in_rgb::convert_byte_in_rgb;


pub fn random_color(){
    thread::spawn(||{
        create_sender_udp_port(gen_array());
    });

    let received_byte = VecDeque::from(create_listen_udp_port_3615(&create_socket()));
    convert_byte_in_rgb(received_byte);
}

pub fn generate_random_number_in_thread(time_in_millis: time::Duration) -> u8{
    //variables qui permettent d'envoyer des données entre les threads
    let (tx, rx) = mpsc::channel::<u8>();

        thread::spawn(move || {
            let mut random_number = rand::rng();
            let random_number: u8 =  random_number.random_range(0..255);
            tx.send(random_number).unwrap();
        });
        
        thread::sleep(time_in_millis);
        let received: u8 = rx.recv().unwrap();
        received
}

pub fn gen_array()-> Vec<u8>  {
    
    let time_in_millis = time::Duration::from_millis(50); 
        let mut vec_of_bytes: Vec<u8> = vec![];
            println!("numbers are generated please press a key to show the range");

        loop{
            vec_of_bytes.push(generate_random_number_in_thread(time_in_millis));
            
            //event::poll permet d'attendre un evenement sans bloquer le thread
            // ici on attend qu'un évenment key soit pressé 
            if event::poll(time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(key_event_kind) = event::read().unwrap() {    // si l'event est lu correctement on effectue le block{ }
                    if key_event_kind.is_press()==true{
                        break;
                    }else{
                        continue;
                    }                    
                }
            }
        };
        vec_of_bytes
}