use enigo::*;
use std::thread;
use std::time::Duration;

fn main() {
    let mut enigo = Enigo::new();

    thread::sleep(Duration::from_secs(1));

    println!("pasting");
    // Simulate pressing a combination: Ctrl+V
    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('v'));
    enigo.key_up(Key::Control);
    println!("end");
}
