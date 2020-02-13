pub mod pixel {
//build a pixel

    pub struct Pixel {
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

        pub fn invert(&mut self){
            self.r = 255-self.r;
            self.g = 255-self.g;
            self.b = 255-self.b;
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

        pub fn grayscale(&mut self){
            let value = (self.r + self.g + self.b) / 3;
            self.r = value;
            self.g = value;
            self.b = value;
        }
    }
}