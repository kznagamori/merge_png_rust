use image::{Pixel, RgbaImage};
use std::env;

fn main() -> Result<(), image::ImageError> {
    let args: Vec<String> = env::args().collect();

    // コマンドライン引数の確認
    if args.len() != 4 {
        eprintln!("Usage: {} <input1.png> <input2.png> <output.png>", args[0]);
        std::process::exit(1);
    }

    let input_path1 = &args[1];
    let input_path2 = &args[2];
    let output_path = &args[3];

    // 2つのPNGファイルを読み込む
    let img1 = image::open(input_path1)?.to_rgba8();
    let img2 = image::open(input_path2)?.to_rgba8();

    // 画像サイズをチェック
    let (width, height) = img1.dimensions();
    assert_eq!((width, height), img2.dimensions());

    // 新しい画像を作成
    let mut new_image = RgbaImage::new(width, height);

    // 2つの画像をピクセル単位で比較し、より明るいピクセルを新しい画像に使用
    for y in 0..height {
        for x in 0..width {
            let pixel1 = img1.get_pixel(x, y).to_rgba();
            let pixel2 = img2.get_pixel(x, y).to_rgba();

            // より明るいピクセルを選択
            let new_pixel = if pixel1[0] as u32 + pixel1[1] as u32 + pixel1[2] as u32
                > pixel2[0] as u32 + pixel2[1] as u32 + pixel2[2] as u32
            {
                pixel1
            } else {
                pixel2
            };

            new_image.put_pixel(x, y, new_pixel);
        }
    }

    // 新しい画像をPNGファイルとして保存
    new_image.save(output_path)?;

    Ok(())
}
