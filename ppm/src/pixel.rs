pub mod pixel {
//build a pixel

    pub struct Pixel {
        r:u8,
        g:u8,
        b:u8
    }

    impl Pixel{
        
        pub fn new(r:u8, g:u8, b:u8) -> Pixel{
            Pixel {
                r:r,
                g:g,
                b:b,
            }    
        }

        pub fn getRed(self) -> u8{
            return self.r;
        }

        pub fn getGreen(self) -> u8{
            return self.g;
        }

        pub fn getBlue(self) -> u8{
            return self.b;
        }

        pub fn display(self) -> String {
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

        pub fn invert(&mut self){
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

    pub fn grayscale(pixel: Pixel) -> Pixel{
        let value = ((pixel.getRed() as u32 + pixel.getGreen() as u32 + pixel.getBlue() as u32) / 3) as u8;
        return Pixel::new(value, value, value);
    }

    impl PartialEq for Pixel {
        fn eq(&self, other: &Self) -> bool {
            return self.r == self.r && self.g == self.g && self.b == self.b;
        }
    }
}

#[cfg(test)]
mod pixel_tests {
    
    pub use super::pixel;

    #[test]
    fn new_pixel(){
        let p = pixel::Pixel::new(255, 255, 255);
        assert_eq!(p.getRed(), 255);
        assert_eq!(p.getGreen(), 255);
        assert_eq!(p.getBlue(), 255);
    }

    #[test]
    fn pixel_display(){
        let p = pixel::Pixel::new(255, 255, 255);
        let x = p.display();
        assert_eq!("255 255 255", x);
    }

    #[test]
    fn pixel_invert(){
        let mut p = pixel::Pixel::new(200, 100, 50);
        p.invert();
        assert_eq!(p.getRed(), 55);
        assert_eq!(p.getGreen(), 155);
        assert_eq!(p.getBlue(), 205);
    }

    #[test]
    fn pixel_partial_eq(){
        let mut p1 = pixel::Pixel::new(255, 255, 255);
        let p2 = pixel::Pixel::new(255, 255, 255);
        assert_eq!(p1 == p2, true);
        assert_eq!(p1 != p2, false);
    }

    #[test]
    fn pixel_clone(){
        let p = pixel::Pixel::new(10, 50, 100);
        let mut pc = p.clone();
        assert_eq!(p == pc, true);
    }

    #[test]
    fn pixel_grayscale(){
        let p = pixel::Pixel::new(200, 100, 30);
        let gray = pixel::grayscale(p);
        assert_eq!(gray.getGreen(), gray.getGreen());
        assert_eq!(gray.getRed(), gray.getBlue());
        assert_eq!(gray.getGreen(), gray.getBlue());
        assert_eq!(gray.getRed(), 110);
    }

}