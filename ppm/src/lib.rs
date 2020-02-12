use std::path::Path;
use std::io::Write;
use std::fs::File;

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

    fn display(self) -> String {
        let space = " ";
        let mut build_str = String::from("");
        let red = self.r.to_string();
        let green = self.g.to_string();
        let bleu = self.b.to_string();
        build_str.push_str(&red);
        build_str.push_str(space);
        build_str.push_str(&green);
        build_str.push_str(space);
        build_str.push_str(&bleu);
  
        return build_str;
    }

    fn invert(&mut self){
        self.r = 255-self.r;
        self.g = 255-self.g;
        self.b = 255-self.b;
    }
}

impl Copy for Pixel { }

impl Clone for Pixel {
    fn clone(&self) -> Pixel {
        *self
    }
}

impl PartialEq for Pixel {
    fn eq(&self, other: &Self) -> bool {
        return self.r == self.r && self.g == self.g && self.b == self.b;
    }
}

//build an image
struct Image {
    points: Vec<Pixel>,
    heigth:u32,
    width:u32
}

impl Image{
    pub fn new(height: u32, width: u32) -> Image {
        let size = 3 * height * width;
        let vect = vec![0; size as usize];
        Image { height: height, width: width, points: vect }
    }
 
    fn buffer_size(&self) -> u32 {
        3 * self.height * self.width
    }
 
    fn get_offset(&self, x: u32, y: u32) -> Option<usize> {
        let offset = (y * self.width * 3) + (x * 3);
        if offset < self.buffer_size() {
            Some(offset as usize)
        } else {
            None
        }
    }
 
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<Pixel> {
        match self.get_offset(x, y) {
            Some(offset) => {
                let r = self.points[offset];
                let g = self.points[offset + 1];
                let b = self.points[offset + 2];
                Some(Pixel {r: r, g: g, b: b})
            },
            None => None
        }
    }
 
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Pixel) -> bool {
        match self.get_offset(x, y) {
            Some(offset) => {
                self.points[offset] = color.r;
                self.points[offset + 1] = color.g;
                self.points[offset + 2] = color.b;
                true
            },
            None => false
        }
    }
 
    pub fn write_file(&self, filename: &str) -> std::io::Result<()> {
        let path = Path::new(filename);
        let mut file = try!(File::create(&path));
        let header = format!("P6 {} {} 255\n", self.width, self.height);
        try!(file.write(header.as_bytes()));
        try!(file.write(&self.points));
        Ok(())
    }
}

//externals methodes
#[no_mangle]
pub extern fn toSeeIfPixelWorks() {
    let mut p = Pixel::new(12, 78, 200);
    println!("{}\n",p.display() );
    let p2 =  Pixel::clone(&p);
    println!("{}",p2.display());
}

#[no_mangle]
fn new_with_file(filename: &Path) -> Image{

}