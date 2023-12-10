use image::{imageops, io::Reader, DynamicImage, ImageOutputFormat};
use qrcode_generator::QrCodeEcc;
use std::io::Cursor;

pub fn make_qrcode(link: &str, logo: Option<Vec<u8>>) -> Vec<u8> {
    let qr_bytes = qrcode_generator::to_png_to_vec(link.as_bytes(), QrCodeEcc::High, 1024).unwrap();

    let mut qr = get_dynamic_image(qr_bytes).to_rgba8();

    if let Some(logo) = logo {
        let logo_bytes = get_dynamic_image(logo.to_owned());
        let (width, height) = get_dynamic_image_size(logo);

        let width = 381;
        let height = 381;

        imageops::overlay(&mut qr, &logo_bytes, width.into(), height.into());
    }

    let mut meme: Vec<u8> = Vec::new();

    qr.write_to(&mut Cursor::new(&mut meme), ImageOutputFormat::Png)
        .unwrap();

    meme
}

fn get_dynamic_image(bytes: Vec<u8>) -> DynamicImage {
    let png = Reader::new(Cursor::new(bytes))
        .with_guessed_format()
        .unwrap();

    png.decode().unwrap()
}

fn get_dynamic_image_size(bytes: Vec<u8>) -> (u32, u32) {
    let png = Reader::new(Cursor::new(bytes))
        .with_guessed_format()
        .unwrap();

    png.into_dimensions().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    use image::ImageFormat;

    #[test]
    fn test_qr_conversion() {
        let qr_bytes =
            qrcode_generator::to_png_to_vec("https://google.com".as_bytes(), QrCodeEcc::High, 1024)
                .unwrap();

        let png = Reader::new(Cursor::new(qr_bytes.clone()))
            .with_guessed_format()
            .unwrap();
        assert_eq!(png.format(), Some(ImageFormat::Png));

        let qr = get_dynamic_image(qr_bytes);

        let meme: Vec<u8> = Vec::new();

        qr.write_to(&mut Cursor::new(meme.clone()), ImageOutputFormat::Png)
            .unwrap();

        let png = Reader::new(Cursor::new(meme))
            .with_guessed_format()
            .unwrap();

        assert_eq!(png.format(), Some(ImageFormat::Png));
    }
}
