
mod lib_;

use std::io::Write;
use crossterm::cursor::position;
use lib_::random_color::random_color;
use lib_::byte_rgb::ByteRGB;
use lib_::udp_sockets::{create_listen_udp_port_3615, create_sender_udp_port};
use lib_::convert_byte_in_rgb::convert_byte_in_rgb;

use std::collections::VecDeque;
use std::thread;

fn main() {
    let red: ByteRGB = ByteRGB::new(255,0,0);
    let blue: ByteRGB = ByteRGB::new(0,0,255);
    let black: ByteRGB = ByteRGB::new(0,0,0);
    //red.convert_byte_in_rgb();
    let buffer_send = create_buffer_to_send(red, blue, black,calculate_buffer(16));
    thread::spawn(||{
        create_sender_udp_port(buffer_send);
    });
    let received_byte = VecDeque::from(create_listen_udp_port_3615());
    create_rgb(received_byte);
    //convert_byte_in_rgb(received_byte);
}
pub struct Player{
    colors: (ByteRGB,ByteRGB,ByteRGB),
    position: u32,
}
impl Player{
    pub fn new(color1: ByteRGB, color2: ByteRGB, color3: ByteRGB)-> Player{
        Self{
            colors:(color1,color2,color3),
            position: 50,
        }
    }

    pub fn set_player_position(&mut self,new_position: u32){
        self.position= new_position;
    }

    pub fn get_player_position(&self)-> u32{
        self.position
    }
}

fn create_rgb(mut received_byte: VecDeque<u8>){
    while !received_byte.is_empty(){
        let color: ByteRGB = ByteRGB::new(received_byte.pop_front().unwrap(), received_byte.pop_front().unwrap(),received_byte.pop_front().unwrap());
        color.convert_byte_in_rgb();
    }
}

fn calculate_buffer(player_position: u8)-> (u8,u8,u8,u8,u8){
    let left_tube:u8 =15;
    let empty_space_left:u8;
    let player:u8= 3;
    let empty_space_right: u8;
    let right_tube:u8= 15;

    empty_space_left = (player_position -1) -left_tube;
    empty_space_right= (((100 - right_tube) - player)-empty_space_left) -left_tube;
    (left_tube,empty_space_left,player,empty_space_right,right_tube)


}

fn create_buffer_to_send(tube: ByteRGB, player: ByteRGB, empty: ByteRGB, calcultated_bufffer: (u8,u8,u8,u8,u8))-> Vec<u8>{
    let (left_tube,left_empty_space, player_pixel, right_empty_space,right_tube) = calcultated_bufffer;
    //creation du tampon à envoyer
    let mut tampon: Vec<u8> = Vec::new();
    // fois tubes
    for index in 0..left_tube{
        for value in tube.get_rgb_values(){
            tampon.push(value);
        }
    }
    // fois empty
    for index in 0..left_empty_space{
        for value in empty.get_rgb_values(){
            tampon.push(value);
        }
    }
    // fois player
    for index in 0..player_pixel{
        for value in player.get_rgb_values(){
            tampon.push(value);
        }
    }
    // fois empty
    for index in 0..right_empty_space{
        for value in empty.get_rgb_values(){
            tampon.push(value);
        }
    }
    // fois tubes
    for index in 0..right_tube{
        for value in tube.get_rgb_values(){
            tampon.push(value);
        }
    }
    tampon

}

fn erase_line(){
    //clear the console and hide cursor
    print!("\x1bc\x1b[?25l"); 

    //print is line-buffered by default, use flush method to ensure the output is emitted immediately
    std::io::stdout().flush().expect("error while flush"); 
}