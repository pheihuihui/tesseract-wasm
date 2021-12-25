use tesslib::ocr;

fn main() {
    let res = ocr("../pic.png", "eng");
    if let Ok(text) = res {
        println!("{}", text);
    }
}
