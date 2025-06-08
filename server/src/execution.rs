use enigo::{self, Keyboard};
use once_cell::sync::Lazy;
use std::sync::Mutex;

static ENIGO: Lazy<Mutex<enigo::Enigo>> =
    Lazy::new(|| Mutex::new(enigo::Enigo::new(&enigo::Settings::default()).unwrap()));

fn init_enigo() -> std::sync::MutexGuard<'static, enigo::Enigo> {
    ENIGO.lock().unwrap()
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

enum KeyWrapper {
    KeyEnum(enigo::Key),
    Raw(u16),
    None,
}

fn translate(key: &str) -> KeyWrapper {
    match key {
        // Control Keys
        "Super" => KeyWrapper::KeyEnum(enigo::Key::Meta),
        "Space" => KeyWrapper::KeyEnum(enigo::Key::Space),
        "Escape" => KeyWrapper::KeyEnum(enigo::Key::Escape),
        "Tab" => KeyWrapper::KeyEnum(enigo::Key::Tab),
        "CapsLock" => KeyWrapper::KeyEnum(enigo::Key::CapsLock),
        "ShiftLeft" => KeyWrapper::KeyEnum(enigo::Key::LShift),
        "ControlLeft" => KeyWrapper::KeyEnum(enigo::Key::LControl),
        "ShiftRight" => KeyWrapper::KeyEnum(enigo::Key::RShift),
        "AltLeft" => KeyWrapper::KeyEnum(enigo::Key::Alt),
        "ControlRight" => KeyWrapper::KeyEnum(enigo::Key::RControl),
        "PageUp" => KeyWrapper::KeyEnum(enigo::Key::PageUp),
        "PageDown" => KeyWrapper::KeyEnum(enigo::Key::PageDown),
        "ArrowLeft" => KeyWrapper::KeyEnum(enigo::Key::LeftArrow),
        "ArrowDown" => KeyWrapper::KeyEnum(enigo::Key::DownArrow),
        "ArrowRight" => KeyWrapper::KeyEnum(enigo::Key::RightArrow),
        "ArrowUp" => KeyWrapper::KeyEnum(enigo::Key::UpArrow),
        "Backspace" => KeyWrapper::KeyEnum(enigo::Key::Backspace),
        "Enter" => KeyWrapper::KeyEnum(enigo::Key::Return),
        "Delete" => KeyWrapper::KeyEnum(enigo::Key::Delete),
        "Insert" => KeyWrapper::KeyEnum(enigo::Key::Insert),
        "End" => KeyWrapper::KeyEnum(enigo::Key::End),
        "Home" => KeyWrapper::KeyEnum(enigo::Key::Home),
        "NumLock" => KeyWrapper::KeyEnum(enigo::Key::Numlock),
        "NumpadEnter" => KeyWrapper::KeyEnum(enigo::Key::Return),
        "AltRight" => KeyWrapper::Raw(92), // special case, extra filtered below, because AltGr is not supported by enigos keys directly

        // Extra Keys
        "Backquote" => KeyWrapper::Raw(49),
        "IntlBackslash" => KeyWrapper::Raw(94),
        "Comma" => KeyWrapper::Raw(59),
        "Period" => KeyWrapper::Raw(60),
        "Slash" => KeyWrapper::Raw(61),
        "Semicolon" => KeyWrapper::Raw(47),
        "Minus" => KeyWrapper::Raw(20),
        "Quote" => KeyWrapper::Raw(48),
        "Backslash" => KeyWrapper::Raw(51),
        "BracketLeft" => KeyWrapper::Raw(34),
        "BracketRight" => KeyWrapper::Raw(35),
        "Equal" => KeyWrapper::Raw(21),

        // Normal Keys
        // -> get codes         sudo xve
        "Digit1" => KeyWrapper::Raw(10),
        "Digit2" => KeyWrapper::Raw(11),
        "Digit3" => KeyWrapper::Raw(12),
        "Digit4" => KeyWrapper::Raw(13),
        "Digit5" => KeyWrapper::Raw(14),
        "Digit6" => KeyWrapper::Raw(15),
        "Digit7" => KeyWrapper::Raw(16),
        "Digit8" => KeyWrapper::Raw(17),
        "Digit9" => KeyWrapper::Raw(18),
        "Digit0" => KeyWrapper::Raw(19),
        "KeyQ" => KeyWrapper::Raw(24),
        "KeyW" => KeyWrapper::Raw(25),
        "KeyE" => KeyWrapper::Raw(26),
        "KeyR" => KeyWrapper::Raw(27),
        "KeyT" => KeyWrapper::Raw(28),
        "KeyY" => KeyWrapper::Raw(29),
        "KeyU" => KeyWrapper::Raw(30),
        "KeyI" => KeyWrapper::Raw(31),
        "KeyO" => KeyWrapper::Raw(32),
        "KeyP" => KeyWrapper::Raw(33),
        "KeyA" => KeyWrapper::Raw(38),
        "KeyS" => KeyWrapper::Raw(39),
        "KeyD" => KeyWrapper::Raw(40),
        "KeyF" => KeyWrapper::Raw(41),
        "KeyG" => KeyWrapper::Raw(42),
        "KeyH" => KeyWrapper::Raw(43),
        "KeyJ" => KeyWrapper::Raw(44),
        "KeyK" => KeyWrapper::Raw(45),
        "KeyL" => KeyWrapper::Raw(46),
        "KeyZ" => KeyWrapper::Raw(52),
        "KeyX" => KeyWrapper::Raw(53),
        "KeyC" => KeyWrapper::Raw(54),
        "KeyV" => KeyWrapper::Raw(55),
        "KeyB" => KeyWrapper::Raw(56),
        "KeyN" => KeyWrapper::Raw(57),
        "KeyM" => KeyWrapper::Raw(58),

        // Numpad Keys
        "Numpad0" => KeyWrapper::Raw(90),
        "Numpad1" => KeyWrapper::Raw(87),
        "Numpad2" => KeyWrapper::Raw(88),
        "Numpad3" => KeyWrapper::Raw(89),
        "Numpad4" => KeyWrapper::Raw(83),
        "Numpad5" => KeyWrapper::Raw(84),
        "Numpad6" => KeyWrapper::Raw(85),
        "Numpad7" => KeyWrapper::Raw(79),
        "Numpad8" => KeyWrapper::Raw(80),
        "Numpad9" => KeyWrapper::Raw(81),
        "NumpadMultiply" => KeyWrapper::Raw(63),
        "NumpadAdd" => KeyWrapper::Raw(86),
        "NumpadSubtract" => KeyWrapper::Raw(82),
        "NumpadDecimal" => KeyWrapper::Raw(91),
        "NumpadDivide" => KeyWrapper::Raw(106),

        // TODO this now sends the keyCODES for most of the keys. yet it is missing, sending the correct keySYMs
        // That would however require making assumptions about the current keyboard layout (I think)

        // Function keys
        "F1" => KeyWrapper::KeyEnum(enigo::Key::F1),
        "F2" => KeyWrapper::KeyEnum(enigo::Key::F2),
        "F3" => KeyWrapper::KeyEnum(enigo::Key::F3),
        "F4" => KeyWrapper::KeyEnum(enigo::Key::F4),
        "F5" => KeyWrapper::KeyEnum(enigo::Key::F5),
        "F6" => KeyWrapper::KeyEnum(enigo::Key::F6),
        "F7" => KeyWrapper::KeyEnum(enigo::Key::F7),
        "F8" => KeyWrapper::KeyEnum(enigo::Key::F8),
        "F9" => KeyWrapper::KeyEnum(enigo::Key::F9),
        "F10" => KeyWrapper::KeyEnum(enigo::Key::F10),
        "F11" => KeyWrapper::KeyEnum(enigo::Key::F11),
        "F12" => KeyWrapper::KeyEnum(enigo::Key::F12),
        "F13" => KeyWrapper::KeyEnum(enigo::Key::F13),
        "F14" => KeyWrapper::KeyEnum(enigo::Key::F14),
        "F15" => KeyWrapper::KeyEnum(enigo::Key::F15),
        "F16" => KeyWrapper::KeyEnum(enigo::Key::F16),
        "F17" => KeyWrapper::KeyEnum(enigo::Key::F17),
        "F18" => KeyWrapper::KeyEnum(enigo::Key::F18),
        "F19" => KeyWrapper::KeyEnum(enigo::Key::F19),
        "F20" => KeyWrapper::KeyEnum(enigo::Key::F20),
        "F21" => KeyWrapper::KeyEnum(enigo::Key::F21),
        "F22" => KeyWrapper::KeyEnum(enigo::Key::F22),
        "F23" => KeyWrapper::KeyEnum(enigo::Key::F23),
        "F24" => KeyWrapper::KeyEnum(enigo::Key::F24),
        "F25" => KeyWrapper::KeyEnum(enigo::Key::F25),
        "F26" => KeyWrapper::KeyEnum(enigo::Key::F26),
        "F27" => KeyWrapper::KeyEnum(enigo::Key::F27),
        "F28" => KeyWrapper::KeyEnum(enigo::Key::F28),
        "F29" => KeyWrapper::KeyEnum(enigo::Key::F29),
        "F30" => KeyWrapper::KeyEnum(enigo::Key::F30),
        "F31" => KeyWrapper::KeyEnum(enigo::Key::F31),
        "F32" => KeyWrapper::KeyEnum(enigo::Key::F32),
        "F33" => KeyWrapper::KeyEnum(enigo::Key::F33),
        "F34" => KeyWrapper::KeyEnum(enigo::Key::F34),
        "F35" => KeyWrapper::KeyEnum(enigo::Key::F35),

        _ => KeyWrapper::None, // fallback
    }
}

pub fn keyboard_various(key_information: &str, down: bool) {
    let mut enigo = init_enigo();

    // The format is <JSKeyCode>--<raw/key>--<JSKey>
    let parts: Vec<_> = key_information.split("--").collect();
    let key_code = parts[0];
    let raw = parts[1] == "raw";
    let key_unicode = parts[2].chars().nth(0);

    // println!("Key Type: {}, {}, {}", key_code, raw, parts[2]);

    match translate(key_code) {
        KeyWrapper::KeyEnum(translated_key) => {
            let _ = enigo.key(
                translated_key,
                match down {
                    true => enigo::Direction::Press,
                    false => enigo::Direction::Release,
                },
            );
        }
        KeyWrapper::Raw(translated_raw_code) => {
            if raw || key_code == "AltRight" {
                let _ = enigo.raw(
                    translated_raw_code,
                    match down {
                        true => enigo::Direction::Press,
                        false => enigo::Direction::Release,
                    },
                );
            } else {
                match key_unicode {
                    Some(key_unicode) => {
                        let _ = enigo.key(
                            enigo::Key::Unicode(key_unicode),
                            match down {
                                true => enigo::Direction::Press,
                                false => enigo::Direction::Release,
                            },
                        );
                    }
                    None => {
                        println!("Non-Raw mode, but did not get a valid char")
                    }
                }
            }
        }
        KeyWrapper::None => {
            println!(
                "Recieved a key code that could not be translated: {}",
                key_code
            )
        }
    }
}
