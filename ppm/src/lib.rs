
pub mod pixel;
pub mod image;

pub use crate::pixel::pixel as pixel_mod;
pub use crate::image::image as image_mod;

//externals methodes
#[no_mangle]
pub extern fn toSeeIfPixelWorks() {
    let mut p = pixel_mod::Pixel::new(12, 78, 200);
    println!("Pixel: {}\n",p.display() );
    let p2 =  pixel_mod::Pixel::clone(&p);
    println!("Pixel clone: {}\n",p2.display());
    p.invert();
    println!("Pixel inverser: {}\n",p.display());
}

// #[no_mangle]
// fn new_with_file(filename: &Path) -> Image{

// }
