pub struct ByteRGB{
    red: u8,
    green: u8,
    blue: u8,
}
impl ByteRGB{
    pub fn new(red: u8, green: u8, blue: u8)-> Self{
        Self{
            red,
            green,
            blue,
        }
    }

    pub fn convert_byte_in_rgb(&self){
        print!("\x1b[38;2;{};{};{}m■\x1b[0m", self.red, self.green, self.blue);
    }

    pub fn get_rgb_values(&self)-> [u8;3]{
        let rgb_values:[u8;3] = [self.red, self.green, self.blue];
        rgb_values
    }
}