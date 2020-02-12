//use std::ops::Add;

//build a pixel

struct Pixel {
    r:u8,
    g:u8,
    b:u8
}

impl Pixel{
    fn new(r:u8, g:u8, b:u8) -> Pixel{
        Pixel {
            r:r,
            g:g,
            b:b,
        } 
    }
    // Copy
    // Clone
    fn display(self) -> String {
        let space = " ";
        let mut build_str = String::from("");
        let  red = self.r.to_string();
        let  green = self.g.to_string();
        let bleu = self.b.to_string();
        build_str.push_str(&red);
        build_str.push_str(space);
        build_str.push_str(&green);
        build_str.push_str(space);
        build_str.push_str(&bleu);
  
        return build_str;
    }
}

#[no_mangle]
pub extern fn process() {
    let p = Pixel::new(12, 78, 200);
    println!("{}",p.display() );
}
