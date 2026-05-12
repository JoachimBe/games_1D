use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::time::Duration;


//function who use it must return -> Result<(), Box<dyn std::error::Error>>
pub fn wich_key_is_pressed(){
    if event::poll(Duration::from_millis(100)).unwrap(){
        if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
            // 'code' contient ici la variante de KeyCode (ex: Char('a'), Enter, etc.)
            println!("Le code de la touche est : {:?}", code);
        }
    }
}