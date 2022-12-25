use image::{self, GenericImageView, ImageBuffer, Pixel};

#[allow(dead_code)]
fn saturate(file_name:&str){
let img = image::open(file_name).expect("failed to find images");
let (w, h) = img.dimensions();
let mut output_buf = ImageBuffer::new(w, h);
for (x, y, pixel) in img.pixels() {
    output_buf.put_pixel(x, y, pixel.map(|p| p.saturating_sub(65)));
}
output_buf.save(format!("new_{}",file_name)).expect("failed to save file");
}

#[allow(dead_code)]
fn get_str_ascii(average_rgb_value: u8) -> &'static str {
    // mapping with 8 ascii characters : so dividing 0-255 into 8 intervals
    let index = average_rgb_value/32;
    let ascii = [" ", ".", ",", "-", "~", "+", "=", "@"];
    return ascii[index as usize];
    // index is u32 but array needs usize
}
#[allow(dead_code)]
fn get_image(dir: &str, scale: u32) {
    let img = image::open(dir).expect("Unable to open file in dir");
    let (w, h) = img.dimensions();
    println!("Current dimensions are {:?}", (w,h));
    for y in 0..h {
        for x in 0..w {
            if y % (scale * 2) == 0 || x % scale == 0 {
                let pix = img.get_pixel(x, y);
                let mut average_rgb_value = (pix[0] / 3) + (pix[1] / 3) + (pix[2] / 3);
                if pix[3] == 0 {
                    // alpha==0 i.e transparent image
                    average_rgb_value = 0;
                }
                print!("{}", get_str_ascii(average_rgb_value));
            }
        }
        if y % (scale * 2) == 0 {
            // adding space
            println!(" ")
        }
    }
}
#[allow(dead_code)]
fn scale_image_down(file_name:&str){
    let img = image::open(file_name).unwrap();
    let (w, h) = img.dimensions();
    let new_image = image::imageops::resize(&img, w/3, h/3,image::imageops::FilterType::Nearest);
    new_image.save(format!("new_{}",file_name)).expect("unable to save scaled image");
}
#[allow(dead_code)]
fn main() {
    let file_name = "new_pug.png";
    scale_image_down(&file_name);
    // get_image(file_name, 4);
}
