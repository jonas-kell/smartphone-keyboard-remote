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

pub fn keyboard_enter() {
    let mut enigo = init_enigo();
    let _ = enigo.key(enigo::Key::Return, enigo::Direction::Click);
}

fn translate(key: &str) -> Option<enigo::Key> {
    match key {
        // Control Keys
        "Super" => Some(enigo::Key::Meta),
        "Escape" => Some(enigo::Key::Escape),
        "Tab" => Some(enigo::Key::Tab),
        "CapsLock" => Some(enigo::Key::CapsLock),
        "ShiftLeft" => Some(enigo::Key::LShift),
        "ControlLeft" => Some(enigo::Key::LControl),
        "ShiftRight" => Some(enigo::Key::RShift),
        "AltLeft" => Some(enigo::Key::Alt),
        "AltRight" => Some(enigo::Key::Alt),
        "ControlRight" => Some(enigo::Key::RControl),
        "PageUp" => Some(enigo::Key::PageUp),
        "PageDown" => Some(enigo::Key::PageDown),
        "ArrowLeft" => Some(enigo::Key::LeftArrow),
        "ArrowDown" => Some(enigo::Key::DownArrow),
        "ArrowRight" => Some(enigo::Key::RightArrow),
        "ArrowUp" => Some(enigo::Key::UpArrow),
        "Backspace" => Some(enigo::Key::Backspace),
        "Enter" => Some(enigo::Key::Return),
        "Delete" => Some(enigo::Key::Delete),
        "Insert" => Some(enigo::Key::Insert),
        "End" => Some(enigo::Key::End),
        "Home" => Some(enigo::Key::Home),
        "NumLock" => Some(enigo::Key::Numlock),
        "NumpadEnter" => Some(enigo::Key::Return),

        // Extra Keys
        "Backquote" => Some(enigo::Key::Unicode('`')),
        "IntlBackslash" => Some(enigo::Key::Unicode('`')),
        "Comma" => Some(enigo::Key::Unicode('`')),
        "Period" => Some(enigo::Key::Unicode('`')),
        "Slash" => Some(enigo::Key::Unicode('`')),
        "Semicolon" => Some(enigo::Key::Unicode('`')),
        "Quote" => Some(enigo::Key::Unicode('`')),
        "Backslash" => Some(enigo::Key::Unicode('`')),
        "BracketLeft" => Some(enigo::Key::Unicode('`')),
        "BracketRight" => Some(enigo::Key::Unicode('`')),
        "Equal" => Some(enigo::Key::Unicode('`')),
        "Minus" => Some(enigo::Key::Unicode('`')),
        "NumpadDecimal" => Some(enigo::Key::Unicode('`')),
        "NumpadAdd" => Some(enigo::Key::Unicode('`')),
        "NumpadSubtract" => Some(enigo::Key::Unicode('`')),
        "NumpadMultiply" => Some(enigo::Key::Unicode('`')),
        "NumpadDivide" => Some(enigo::Key::Unicode('`')),

        // Normal Keys
        "KeyA" => Some(enigo::Key::Unicode('a')),
        "Digit1" => Some(enigo::Key::Unicode('1')),
        "Numpad1" => Some(enigo::Key::Unicode('1')),

        // Function keys
        "F1" => Some(enigo::Key::F1),
        "F2" => Some(enigo::Key::F2),
        "F3" => Some(enigo::Key::F3),
        "F4" => Some(enigo::Key::F4),
        "F5" => Some(enigo::Key::F5),
        "F6" => Some(enigo::Key::F6),
        "F7" => Some(enigo::Key::F7),
        "F8" => Some(enigo::Key::F8),
        "F9" => Some(enigo::Key::F9),
        "F10" => Some(enigo::Key::F10),
        "F11" => Some(enigo::Key::F11),
        "F12" => Some(enigo::Key::F12),
        "F13" => Some(enigo::Key::F13),
        "F14" => Some(enigo::Key::F14),
        "F15" => Some(enigo::Key::F15),
        "F16" => Some(enigo::Key::F16),
        "F17" => Some(enigo::Key::F17),
        "F18" => Some(enigo::Key::F18),
        "F19" => Some(enigo::Key::F19),
        "F20" => Some(enigo::Key::F20),
        "F21" => Some(enigo::Key::F21),
        "F22" => Some(enigo::Key::F22),
        "F23" => Some(enigo::Key::F23),
        "F24" => Some(enigo::Key::F24),
        "F25" => Some(enigo::Key::F25),
        "F26" => Some(enigo::Key::F26),
        "F27" => Some(enigo::Key::F27),
        "F28" => Some(enigo::Key::F28),
        "F29" => Some(enigo::Key::F29),
        "F30" => Some(enigo::Key::F30),
        "F31" => Some(enigo::Key::F31),
        "F32" => Some(enigo::Key::F32),
        "F33" => Some(enigo::Key::F33),
        "F34" => Some(enigo::Key::F34),
        "F35" => Some(enigo::Key::F35),

        _ => None, // fallback
    }
}

pub fn keyboard_various(key_code: &str, down: bool) {
    let mut enigo = init_enigo();

    match translate(key_code) {
        Some(translated_key) => {
            let _ = enigo.key(
                translated_key,
                match down {
                    true => enigo::Direction::Press,
                    false => enigo::Direction::Release,
                },
            );
            println!(
                "Executed translated key: {:?}, down: {}",
                translated_key, down
            )
        }
        None => {
            println!(
                "Recieved a key code that could not be translated: {}",
                key_code
            )
        }
    }
}
