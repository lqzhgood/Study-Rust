use clipboard_win::{formats, get_clipboard, set_clipboard};

fn main() {
    let text = "my sample ><";

    set_clipboard(formats::Unicode, text).expect("To set clipboard");
}
