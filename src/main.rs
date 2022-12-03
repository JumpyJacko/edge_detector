use image::{GenericImageView, ImageBuffer, RgbImage, Rgb};
use clap::{Arg, Command};
use std::time::Instant;

fn main() {
    println!("=== \u{001b}[32mprogram started\u{001b}[0m ===");
    let matches = Command::new("edge-detector")
        .version("1.0")
        .author("Jackson Ly (JumpyJacko)")
        .about("Detects edges on an image using convolution")
        .arg(Arg::new("image")
            .short('i')
            .long("open-image")
            .help("Path to image from working directory"))
        .arg(Arg::new("output")
            .short('o')
            .long("save-image")
            .default_value("./processed.png")
            .help("Path to output image from working directory, (include file ending)"))
        .get_matches();

    let img_path: &str = matches.get_one::<String>("image").unwrap();
    let o_img_path: &str = matches.get_one::<String>("output").unwrap();

    let timer = Instant::now();

    let img = image::open(img_path).unwrap().grayscale();

    let width: u32 = img.dimensions().0;
    let height: u32 = img.dimensions().1;

    let mut edges: RgbImage = ImageBuffer::new(width, height);

    for p in img.pixels() {
        // (x, y, Rgba[u8, u8, u8])

        // TODO: Things i need to implement

        // a, b, c
        // d, e, f
        // g, h, i
        let a: i32 = if p.0 > 1 && p.1 > 1                  {img.get_pixel(p.0 - 1, p.1 - 1).0[0]} else {0} as i32;
        let b: i32 = if p.1 > 1                             {img.get_pixel(p.0    , p.1 - 1).0[0]} else {0} as i32;
        let c: i32 = if p.0 < width - 1 && p.1 > 1          {img.get_pixel(p.0 + 1, p.1 - 1).0[0]} else {0} as i32;
        let d: i32 = if p.0 > 1                             {img.get_pixel(p.0 - 1, p.1    ).0[0]} else {0} as i32;
        let e: i32 =                                         img.get_pixel(p.0    , p.1    ).0[0] as i32;
        let f: i32 = if p.0 < width - 1                     {img.get_pixel(p.0 + 1, p.1    ).0[0]} else {0} as i32;
        let g: i32 = if p.0 > 1 && p.1 < height - 1         {img.get_pixel(p.0 - 1, p.1 + 1).0[0]} else {0} as i32;
        let h: i32 = if p.1 < height - 1                    {img.get_pixel(p.0    , p.1 + 1).0[0]} else {0} as i32;
        let i: i32 = if p.0 < width - 1 && p.1 < height - 1 {img.get_pixel(p.0 + 1, p.1 + 1).0[0]} else {0} as i32;

        // println!("coordinates: ({}, {}); a:{} b:{} c:{} d:{} e:{} f:{} g:{} h:{} i:{}", p.0, p.1, a, b, c, d, e, f, g, h, i);

        let v_convolve: i32 =
        (  (a * 32) + (b * 0) + (c * -32)
         + (d * 64) + (e * 0) + (f * -64)
         + (g * 32) + (h * 0) + (i * -32)) / 256;
        // println!("{}", v_convolve);

        let h_convolve: i32 =
        (  (a *  32) + (b *  64) + (c *  32)
         + (d *   0) + (e *   0) + (f *   0)
         + (g * -32) + (h * -64) + (i * -32)) / 256;

        let value: u8 = (v_convolve + h_convolve / 2) as u8;

        edges.put_pixel(p.0 , p.1, Rgb([value, value, value]));
    }

    edges.save(o_img_path).unwrap();

    let duration_us = timer.elapsed().as_micros();
    let duration_ms = timer.elapsed().as_millis();
    println!("processing duration: \u{001b}[34m{}\u{001b}[0m μs or \u{001b}[34m{}\u{001b}[0m ms", duration_us, duration_ms);
    println!("outputted as \u{001b}[34m{}\u{001b}[0m", o_img_path);
    println!("=== \u{001b}[31mprogram ended\u{001b}[0m ===");
}
