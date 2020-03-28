use clipboard_win::{set_clipboard_string};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "snippet/"]
struct Asset;

fn main() {
    let louise = Asset::get("Louise.txt").unwrap();
    let text = std::str::from_utf8(louise.as_ref()).unwrap();
    set_clipboard_string(text).expect("Success");

    println!("Hello, world!");
}
