use utils::color::Color;
#[warn(non_snake_case)]
fn main() {
    // Image
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in 0..IMAGE_HEIGHT {
        eprintln!("Scanlines remaining: {}", IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            let pixel_color = Color::new(
                (i as f64) / ((IMAGE_WIDTH - 1) as f64),
                (j as f64) / ((IMAGE_HEIGHT - 1) as f64),
                0.0
            );
            pixel_color.write_color();
        }
    }
    eprintln!("\rDone.                               \n");
}
