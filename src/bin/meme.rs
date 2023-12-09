use std::{fs};
use std::ops::Deref;
use tiny_skia::{Pixmap, Transform};
use usvg::{Options, Tree, TreeParsing};
use qrcodegen::{QrCode, QrCodeEcc};
use resvg::Tree as ResvgTree;

const WIDTH: u32 = 810;
const HEIGHT: u32 = 810;

fn main() {
	let text: &'static str = "Hello, world!";   // User-supplied Unicode text
	let errcorlvl: QrCodeEcc = QrCodeEcc::High;  // Error correction level
	
	// Make and print the QR Code symbol
	let qr: QrCode = QrCode::encode_text(text, errcorlvl).unwrap();
// Create a new pixmap buffer to render to

    let qrcode = to_svg_string(&qr, 1);

    let mut pixmap = Pixmap::new(WIDTH, HEIGHT)
        .ok_or("Pixmap allocation error").unwrap();

    // Use default settings
    let options = Options::default();

    // Build our string into a svg tree
    let tree = Tree::from_str(&qrcode, &options).unwrap();
    let resvgtree = ResvgTree::from_usvg(&tree);

    // Render our tree to the pixmap buffer, using default fit and transformation settings

    resvgtree.render(
        Transform::from_scale(30., 30.),
        &mut pixmap.as_mut(),
    );

    // Encode our pixmap buffer into a webp image
    let encoded_buffer =
        webp::Encoder::new(pixmap.data(), webp::PixelLayout::Rgba, WIDTH, HEIGHT).encode_lossless();

    let result = encoded_buffer.deref();

    // Write the result
    fs::write("image.webp", result).unwrap();
}


fn to_svg_string(qr: &QrCode, border: i32) -> String {
	assert!(border >= 0, "Border must be non-negative");
	let mut result = String::new();
	result += "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";
	result += "<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n";
	let _dimension = qr.size().checked_add(border.checked_mul(2).unwrap()).unwrap();
	result += "<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" viewBox=\"0 0 29 29\" stroke=\"none\">\n";
	result += "\t<rect width=\"100%\" height=\"100%\" fill=\"#FFFFFF\"/>\n";
	result += "\t<path d=\"";
	for y in 0 .. qr.size() {
		for x in 0 .. qr.size() {
			if qr.get_module(x, y) {
				if x != 0 || y != 0 {
					result += " ";
				}
				result += &format!("M{},{}h1v1h-1z", x + border, y + border);
			}
		}
	}
	result += "\" fill=\"#000000\"/>\n";
	result += "\" fill-rule=\"evenodd\"/>\n";
	result += "</svg>\n";
	result
}
