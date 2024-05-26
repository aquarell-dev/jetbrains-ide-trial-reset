mod reset;

use crate::reset::reset;
use crate::reset::windows::WindowsResetter;


fn main() {
    let w = WindowsResetter {};
    reset(&w);
    println!("Hello, world!");
}
