use enigo::*;

pub fn paste_test() {
    let mut enigo = Enigo::new();

    println!("enigo start");
    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('v'));
    enigo.key_up(Key::Control);
    println!("enigo end");
}
