use clap::{arg, Command};
use image::{io::Reader as ImageReader, GenericImageView, Pixel};
use owo_colors::{OwoColorize, Rgb};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli().get_matches();

    match matches.get_one::<String>("path") {
        Some(path) => output_image(&path),
        None => Err("No path provided".into()),
    }
}

fn output_image(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (width, height) = termion::terminal_size().unwrap();

    let img = ImageReader::open(path)?
        .decode()?
        .thumbnail(width.into(), (height as u32 - 4) * 2);

    let w = img.width();
    let h = img.height();

    img.thumbnail_exact(w, h / 2)
        .pixels()
        .enumerate()
        .for_each(|(i, pixel)| {
            let c = pixel.2.channels();
            let color = Rgb(c[0], c[1], c[2]);
            print!("{}", "█".color(color));
            if (i + 1) % w as usize == 0 {
                println!();
            }
        });
    println!("");

    Ok(())
}

fn cli() -> Command {
    Command::new("climage")
        .version("0.1.0")
        .arg(arg!(path: [PATH]))
}
