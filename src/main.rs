extern crate image;
extern crate num_complex;

use num_complex::Complex;
use image:: ImageBuffer;

fn main() {
    let imgx = 800;
    let imgy = 800;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    let mut img_buf = ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        *pixel = image::Rgb([255, 255, 255]);
    }

    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = Complex::new(-0.5, 0.6);
            let mut z = Complex::new(cx, cy);

            let mut i = 255;
            while i > 0 && z.norm() <= 2.0 {
                z = z * z + c;
                i -= 1;
            }

            let pixel = img_buf.get_pixel_mut(x, y);
            let image::Rgb { data } = *pixel;
            if i < 255 {
                *pixel = image::Rgb([data[0] - i, data[1] - i,data[2] - i]);
            }
            else {
                *pixel = image::Rgb([data[0], i, data[1]]);
            }
        }
    }

    img_buf.save("julia_set.png").unwrap();
}