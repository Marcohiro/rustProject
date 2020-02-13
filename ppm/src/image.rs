pub mod image{
    use std::io::Read;
    use std::fs::File;
    use std::path::Path;
    use std::io::Write;
    use std::sync::mpsc;
    use std::thread;
    use std::io::{BufReader};
    use std::io::BufRead;

    use crate pixel:pixel;

    //build an image
    struct Image {
        pixles: Vec<pixel:Pixel>,
        height:u32,
        width:u32
    }

    impl Image{
        pub fn new(height: u32, width: u32) -> Image {
            let size = 3 * height * width;
            let vect = vec![0; size as usize];
            Image { height: height, width: width, points: vect }
        }
 
        pub fn new_with_file(filename: &Path) -> Image{
            let file = File::open(filename);
        }
    
        pub fn save(&self, filename: &str) -> std::io::Result<()> {
            let path = Path::new(filename);
            let mut file = File::create(&path)?;
            let header = format!("P6 {} {} 255\n", self.width, self.height);
            file.write(header.as_bytes())?;
            file.write(&self.points)?;
            Ok(())
        }

        pub fn invert(&self){
            let mut pixels = self.pixels;
            for i in 0..pixels.len(){
                pixels[i].invert();
            }
        }

        pub fn grayscale(&self){
            let mut pixels = self.pixels;
            for i in 0..pixels.len(){
                pixels[i] = pixel::grayscale(pixels[i]);
            }
        }
    }
}