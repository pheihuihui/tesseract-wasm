use std::env;
use tesslib::ocr;

fn main() {
    if let Some(path) = env::args().nth(1) {
        let res = ocr(&path, "eng");
        if let Ok(text) = res {
            println!("{}", text);
        }
    }
}
