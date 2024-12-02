use enigo::{self, Keyboard};

fn init_enigo() -> enigo::Enigo {
    enigo::Enigo::new(&enigo::Settings::default()).unwrap()
}

pub fn keyboard_basic_text(input: &str) {
    let mut enigo = init_enigo();
    let _ = enigo.text(input);
}

pub fn keyboard_delete() {
    let mut enigo = init_enigo();
    let _ = enigo.key(enigo::Key::Backspace, enigo::Direction::Click);
}
