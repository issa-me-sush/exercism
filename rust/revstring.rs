use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut graphemes = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();
    graphemes.reverse();
    graphemes.join("")
}

// cargo.toml
// [dependencies]
// unicode-segmentation = "1.8.0"
// [features]
// grapheme = []

// [package]
// edition = "2021"
// name = "reverse_string"
// version = "1.2.0"
