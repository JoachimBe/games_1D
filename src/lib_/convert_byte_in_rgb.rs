use std::collections::VecDeque;

pub fn convert_byte_in_rgb(mut received_byte: VecDeque<u8>){
    let mut red: Option<u8> = None;
    let mut green: Option<u8>= None;
    let mut blue: Option<u8>= None;
    while !received_byte.is_empty(){
        while red.is_none() || green.is_none() ||  blue.is_none(){
            if let Some(byte) = received_byte.pop_front(){
                if red.is_none(){
                    red = Some(byte);
                } else if green.is_none(){
                    green = Some(byte);
                } else if blue.is_none(){
                    blue = Some(byte);
                }
            } else{
                if red.is_none(){
                    red = Some(0);
                } else if green.is_none(){
                    green = Some(0);
                } else if blue.is_none(){
                    blue = Some(0);
                }
                break;
            }
        }
        print!("\x1b[38;2;{};{};{}m■\x1b[0m", red.unwrap(), green.unwrap(), blue.unwrap());
        (red,green,blue)  = (None,None,None)
    }
}