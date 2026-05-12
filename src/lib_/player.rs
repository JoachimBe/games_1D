use super::byte_rgb::ByteRGB;

#[derive(Clone)]
pub struct Player{
    colors: [ByteRGB;3],
    position: i16,
    previous_position: i16,
}
impl Player{
    pub fn new(color1: ByteRGB, color2: ByteRGB, color3: ByteRGB)-> Player{
        Self{
            colors:[color1,color2,color3],
            position: 50,
            previous_position: 51,
        }
    }

    pub fn set_player_position(&mut self,new_position: i16){
        self.position= new_position;
    }

    pub fn get_player_position(&self)-> i16{
        self.position
    }

    pub fn set_player_previous_position(&mut self, player_previous_position: i16){
        self.previous_position= player_previous_position;
    }
    pub fn get_player_previous_position(&self) ->i16{
        self.previous_position
    }
    pub fn get_player_colors(&self)-> [ByteRGB; 3]{
        self.colors.clone()
    }

    pub fn calculate_player_position_auto_pong(&mut self){
    if self.position<self.previous_position && self.position >16{
        self.previous_position = self.position;
        self.position = self.position -1;
    } else if self.position<self.previous_position && self.position ==16{
        self.previous_position = self.position;
        self.position = self.position +1;
    } else if self.position > self.previous_position && self.position < 83{
        self.previous_position = self.position;
        self.position = self.position +1;
    } else if self.position > self.previous_position && self.position == 83{
        self.previous_position = self.position;
        self.position = self.position -1;
    }

    /*pub fn calculate_player_position_flappybird(&mut self, is_pressed: bool){

    }*/
}
}