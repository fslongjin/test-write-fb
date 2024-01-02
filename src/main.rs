use std::{fs::File, io::{Read, Seek, Write}, thread::sleep};

use image::GenericImageView;
use jpeg_encoder::Encoder;

const WIDTH: usize = 1440;
const HEIGHT: usize = 900;

static DRAGONOS_LOGO: &[u8] = include_bytes!("dragonos_logo.jpg");

fn main() {
    println!("Hello, world DragonOS-musl");
    // 打开framebuffer
    let mut fb = File::open("/dev/fb0").expect("Unable to open framebuffer");

    // 读取framebuffer的信息
    screen_shot("screen_shot1.jpg", &mut fb);
    print_rect(&mut fb, WIDTH-100, 0, 30, 30);
    print_rect(&mut fb, WIDTH-100, HEIGHT-100, 30, 30);
    print_rect(&mut fb, 0, HEIGHT-100, 30, 30);
    print_rect(&mut fb, 0, 0, 30, 30);

    print_dragonos_logo(&mut fb);
    sleep(std::time::Duration::from_secs(10));
    // 再次截图

    screen_shot("screen_shot2.jpg", &mut fb);
    // 打开输出文件


}

/// 在屏幕上打印一个矩形
fn print_rect(fb: &mut File, xpos: usize, ypos: usize, width: usize, height: usize) {
    for y in ypos..ypos+height {
        for x in xpos..xpos+width {
            let offset = (y * WIDTH + x) * 4;
            fb.seek(std::io::SeekFrom::Start(offset as u64)).expect("Unable to seek framebuffer");
            fb.write_all(&[255, 255, 255, 255]).expect("Unable to write framebuffer"); // white color
        }
    }
}

// 在屏幕正中央打印DragonOS的logo（jpg格式）
fn print_dragonos_logo(fb: &mut File) {

    // 读取图片
    let img = image::load_from_memory(DRAGONOS_LOGO).expect("Unable to load image");

    // 限制图片的大小
    let img = img.resize(800, 800, image::imageops::FilterType::Gaussian);


    let (img_width, img_height) = img.dimensions();

    // 计算图片应该放置的位置，使其位于屏幕中央
    let xpos = (WIDTH as u32 - img_width) / 2;
    let ypos = (HEIGHT as u32  - img_height) / 2;

    

    // 将图片的每个像素写入到 framebuffer
    for y in 0..img_height {
        for x in 0..img_width {
            let pixel = img.get_pixel(x, y);
            let offset = ((y + ypos) * (WIDTH as u32 ) + (x + xpos)) * 4;
            fb.seek(std::io::SeekFrom::Start(offset as u64)).expect("Unable to seek framebuffer");
            fb.write_all(&[pixel[2], pixel[1], pixel[0], 255]).expect("Unable to write framebuffer"); // RGBA format
        }
    }
    
}

fn screen_shot(filename: &str, fb: &mut File) {
    let mut data = Vec::new();
    data.resize(WIDTH * HEIGHT * 4, 0u8);
    
    // seek
    fb.seek(std::io::SeekFrom::Start(0))
        .expect("Unable to seek framebuffer");

    // 读取framebuffer的信息
    fb.read_exact(data.as_mut_slice())
        .expect("Unable to read framebuffer");
    // 输出为jpg文件
    let encoder = Encoder::new_file(filename, 100).expect("Unable to create encoder");

    // 写入jpg文件
    encoder
        .encode(
            &data,
            WIDTH as u16,
            HEIGHT as u16,
            jpeg_encoder::ColorType::Bgra,
        )
        .expect("Unable to encode image");
}
