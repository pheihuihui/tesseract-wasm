#[link(name = "leptonica")]
extern "C" {
    pub fn getLeptonicaVersion() -> *mut ::std::os::raw::c_char;
}

fn main() {
    unsafe {
        let ver = getLeptonicaVersion();
        println!("{}", *ver);
    }
}
