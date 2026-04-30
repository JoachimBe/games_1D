
use std::collections::VecDeque;
use std::thread;

use super::generates_bytes_of_random_numbers;
use super::udp_sockets::{create_sender_udp_port,create_listen_udp_port_3615};
use super::convert_byte_in_rgb::convert_byte_in_rgb;


pub fn random_color(){
    thread::spawn(||{
        create_sender_udp_port(generates_bytes_of_random_numbers::gen_array());
    });

    let received_byte = VecDeque::from(create_listen_udp_port_3615());
    convert_byte_in_rgb(received_byte);
}