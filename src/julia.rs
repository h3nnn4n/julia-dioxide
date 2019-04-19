use complex::Complex;
use std::f64;

pub fn get_julia_set(width: u32, height: u32, c: Complex) -> Vec<u8> {
    let mut data = Vec::new();

    let view_width = 1.5;
    let view_height = 1.5;

    let center_x = 0.0;
    let center_y = 0.0;

    let scale_x = (view_width * 2.0) / (width as f64);
    let scale_y = (view_height * 2.0) / (height as f64);

    for x in 0..width {
        for y in 0..height {
            let y = (y as f64) * scale_x - view_width + center_x;
            let x = (x as f64) * scale_y - view_height + center_y;

            let z = Complex {
                real: y,
                imaginary: x,
            };

            let iter_index = get_iter_index(z, c);

            data.push((iter_index / 4) as u8);
            data.push((iter_index / 2) as u8);
            data.push(iter_index as u8);
            data.push(255);
        }
    }

    data
}

fn get_iter_index(z: Complex, c: Complex) -> u32 {
    let mut iter_index: u32 = 0;
    let mut z = z;

    while iter_index < 1000 {
        if z.norm() > 4.0 {
            break;
        }

        z = z.square() + c;

        iter_index += 1;
    }

    iter_index
}
