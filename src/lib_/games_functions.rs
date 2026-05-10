use std::collections::VecDeque;
use std::io::Write;

use super::byte_rgb::ByteRGB;
use super::player::Player;


pub fn create_rgb(mut received_byte: VecDeque<u8>){
    while !received_byte.is_empty(){
        let color: ByteRGB = ByteRGB::new(received_byte.pop_front().unwrap(), received_byte.pop_front().unwrap(),received_byte.pop_front().unwrap());
        color.convert_byte_in_rgb();
    }
}

pub fn calculate_buffer(player_position: u8)-> (u8,u8,u8,u8){
    let left_tube:u8 =15;
    let empty_space_left:u8;
    let player:u8= 3;
    let empty_space_right: u8;
    let right_tube:u8= 15;

    empty_space_left = (player_position -1) -left_tube;
    empty_space_right= (((100 - right_tube) - player)-empty_space_left) -left_tube;
    (left_tube,empty_space_left,empty_space_right,right_tube)


}
pub fn create_buffer_to_send(tube: ByteRGB, player: Player, empty: ByteRGB, calcultated_bufffer: (u8,u8,u8,u8))-> Vec<u8>{
    let (left_tube,left_empty_space, right_empty_space,right_tube) = calcultated_bufffer;
    //creation du tampon à envoyer
    let mut tampon: Vec<u8> = Vec::new();
    // fois tubes
    for _index in 0..left_tube{
        for value in tube.get_rgb_values(){
            tampon.push(value);
        }
    }
    // fois empty
    for _index in 0..left_empty_space{
        for value in empty.get_rgb_values(){
            tampon.push(value);
        }
    }
    // fois player
    for color in player.get_player_colors(){
        for value in color.get_rgb_values(){
            tampon.push(value);
        }
    }
    // fois empty
    for _index in 0..right_empty_space{
        for value in empty.get_rgb_values(){
            tampon.push(value);
        }
    }
    // fois tubes
    for _index in 0..right_tube{
        for value in tube.get_rgb_values(){
            tampon.push(value);
        }
    }
    tampon

}
pub fn erase_line(){
    //clear the console and hide cursor
    print!("\x1bc\x1b[?25l"); 

    //print is line-buffered by default, use flush method to ensure the output is emitted immediately
    std::io::stdout().flush().expect("error while flush"); 
}