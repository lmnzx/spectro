use hound;
use image::{ImageBuffer, Rgb};
use rustfft::algorithm::Radix4;
use rustfft::num_complex::{Complex, ComplexFloat};
use rustfft::{Fft, FftDirection};

fn main() {
    let path: String = std::env::args().skip(1).collect();

    if path.is_empty() {
        println!("Usage: cargo run -- <path>");
        return;
    }
    println!("path -> {}", path);

    let mut reader = hound::WavReader::open(path).expect("not a valid WAV file");

    let samples = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32)
        .collect::<Vec<f32>>();

    let frame_size = 1024;
    let overlap = frame_size / 2;
    let fft: Radix4<f32> = Radix4::new(frame_size, FftDirection::Forward);

    let img_width = frame_size / 2;
    let img_height = samples.len() / frame_size;
    let mut img = ImageBuffer::<Rgb<u8>, Vec<_>>::new(img_width as u32, img_height as u32);

    for (i, frame) in samples.windows(frame_size).step_by(overlap).enumerate() {
        if i >= img_height {
            break;
        }

        let mut frame: Vec<Complex<f32>> = frame
            .iter()
            .enumerate()
            .map(|(j, &s)| {
                let window = 0.54
                    - 0.46
                        * (2.0 * std::f32::consts::PI * j as f32 / (frame_size as f32 - 1.0)).cos();
                Complex::new(s * window, 0.0)
            })
            .collect();

        fft.process(&mut frame);

        for (j, value) in frame.iter().enumerate().take(img_width) {
            if j >= img_width {
                break;
            }
            let magnitude = 255 - (value.norm().log(10.0) * 255.0) as u8;
            img.put_pixel(j as u32, i as u32, Rgb([magnitude, magnitude, magnitude]));
        }
    }

    img.save("output.png").unwrap();
}
