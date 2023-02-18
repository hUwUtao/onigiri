#[cfg(not(target_arch = "wasm32"))]
extern crate winres;
fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("src/assets/icon/icon.ico"); // Replace this with the filename of your .ico file.
        match res.compile() {
            Ok(_) => println!("winres success"),
            Err(_) => println!("skipping winres"),
        };
    }
}
