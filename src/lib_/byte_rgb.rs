pub struct ByteRGB{
    pub red: u8,
    pub green: u8,
    pub blue: u8,
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
}