use device_query::{DeviceQuery, DeviceState, Keycode};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, poll};

use super::debug::wich_key_is_pressed;
use super::random_color::random_color;
use super::byte_rgb::ByteRGB;
use super::udp_sockets::{create_socket,create_listen_udp_port_3615, create_sender_udp_port};
use super::games_functions::{create_rgb, calculate_buffer, create_buffer_to_send,erase_line, convert_byte_rgb_in_u8};
use super::player::Player;

use std::collections::VecDeque;
use std::time::Duration;
use std::{thread, time};



pub fn flappy_bird(){


    let socket = create_socket();
    loop {
        let device_state= DeviceState::new();
        let mut was_pressed = false;

        let red: ByteRGB = ByteRGB::new(255,0,0);
        let green: ByteRGB= ByteRGB::new(0,255,0);
        let blue: ByteRGB = ByteRGB::new(0,0,255);
        let black: ByteRGB = ByteRGB::new(0,0,0);

        let mut player_1= Player::new(blue.clone(), green.clone(), blue.clone());

        loop{
            erase_line();
            let mut player_mouvement:i16 = 0;

            let keys = device_state.get_keys();
            let is_pressed = keys.contains(&Keycode::Space);

            if event::poll(Duration::from_millis(10)).unwrap(){
                if let Event::Key(key_event_kind) = event::read().unwrap(){
                    //if key_event_kind.is_press() == true{
                        if is_pressed && !was_pressed{
                            player_mouvement = 10;
                            //player_1.set_player_position(player_1.get_player_position() +3 );
                            was_pressed = true;
                        } else if !is_pressed && was_pressed {
                            was_pressed= false;
                            }
                            
                        //} 
                }else{
                    player_mouvement = -1
                }
            }
            player_1.set_player_position(player_1.get_player_position() +player_mouvement );
            if player_1.get_player_position() <= 15 || player_1.get_player_position()>= 83 {
                break
            }
            let buffer_send = create_buffer_to_send(red.clone(), player_1.clone(), black.clone(),calculate_buffer(player_1.get_player_position()));
            
            thread::spawn(||{
                create_sender_udp_port(buffer_send);
            });
            let received_byte = VecDeque::from(create_listen_udp_port_3615(&socket));
            create_rgb(received_byte);
            thread::sleep(time::Duration::from_millis(60));
            
            if keys.contains(&Keycode::Escape){ break;}

        }

        //create all red socket
        let red_array=[red;100];
        let all_red = VecDeque::from(convert_byte_rgb_in_u8(red_array.to_vec()));
        create_rgb(all_red);
        thread::sleep(Duration::from_millis(1000))
    }
}




pub fn auto_pong(){

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
        player_1.calculate_player_position_auto_pong();
       
    }
}

