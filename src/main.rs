
mod lib_;


use crossterm::cursor::position;
use lib_::random_color::random_color;
use lib_::byte_rgb::ByteRGB;
use lib_::udp_sockets::{create_socket,create_listen_udp_port_3615, create_sender_udp_port};
use lib_::games_functions::{create_rgb, calculate_buffer, create_buffer_to_send,erase_line};

use lib_::player::Player;

use std::collections::VecDeque;
use std::{thread, time};

fn main() {
    let time_in_millis =  time::Duration::from_millis(200);
    let red: ByteRGB = ByteRGB::new(255,0,0);
    let green: ByteRGB= ByteRGB::new(0,255,0);
    let blue: ByteRGB = ByteRGB::new(0,0,255);
    let black: ByteRGB = ByteRGB::new(0,0,0);

    let mut player_1= Player::new(blue.clone(), green.clone(), blue.clone());

    let socket = create_socket();

    loop{        
        erase_line();
        let buffer_send = create_buffer_to_send(red.clone(), player_1.clone(), black.clone(),calculate_buffer(player_1.get_player_position()));
        thread::spawn(||{
            create_sender_udp_port(buffer_send);
        });
        let received_byte = VecDeque::from(create_listen_udp_port_3615(&socket));
        create_rgb(received_byte);
        thread::sleep(time::Duration::from_millis(18));
        player_1.calculate_player_position();
        
    }
}