use clipboard_win::{set_clipboard_string};

fn main() {
    let text = "Hello, world!";
    set_clipboard_string(text).expect("Success");

    println!("Hello, world!");
}
