use winit::event::VirtualKeyCode as VirtKey;

pub static STR_TO_VIRT_KEY: &[(&str, VirtKey)] = &[
    ("a", VirtKey::A),
    ("b", VirtKey::B),
    ("c", VirtKey::C),
    ("d", VirtKey::D),
    ("e", VirtKey::E),
    ("f", VirtKey::F),
    ("g", VirtKey::G),
    ("h", VirtKey::H),
    ("i", VirtKey::I),
    ("j", VirtKey::J),
    ("k", VirtKey::K),
    ("l", VirtKey::L),
    ("m", VirtKey::M),
    ("n", VirtKey::N),
    ("o", VirtKey::O),
    ("p", VirtKey::P),
    ("q", VirtKey::Q),
    ("r", VirtKey::R),
    ("s", VirtKey::S),
    ("t", VirtKey::T),
    ("u", VirtKey::U),
    ("v", VirtKey::V),
    ("w", VirtKey::W),
    ("x", VirtKey::X),
    ("y", VirtKey::Y),
    ("z", VirtKey::Z),
    ("0", VirtKey::Key0),
    ("1", VirtKey::Key1),
    ("2", VirtKey::Key2),
    ("3", VirtKey::Key3),
    ("4", VirtKey::Key4),
    ("5", VirtKey::Key5),
    ("6", VirtKey::Key6),
    ("7", VirtKey::Key7),
    ("8", VirtKey::Key8),
    ("9", VirtKey::Key9),
    ("F1", VirtKey::F1),
    ("F2", VirtKey::F2),
    ("F3", VirtKey::F3),
    ("F4", VirtKey::F4),
    ("F5", VirtKey::F5),
    ("F6", VirtKey::F6),
    ("F7", VirtKey::F7),
    ("F8", VirtKey::F8),
    ("F9", VirtKey::F9),
    ("F10", VirtKey::F10),
    ("F11", VirtKey::F11),
    ("F12", VirtKey::F12),
    ("Up", VirtKey::Up),
    ("Right", VirtKey::Right),
    ("Down", VirtKey::Down),
    ("Left", VirtKey::Left),
    ("`", VirtKey::Grave),
    ("@", VirtKey::At),
    ("*", VirtKey::Asterisk),
    ("-", VirtKey::Minus),
    ("=", VirtKey::Equals),
    ("+", VirtKey::Plus),
    ("[", VirtKey::LBracket),
    ("]", VirtKey::RBracket),
    (r"\", VirtKey::Backslash),
    (";", VirtKey::Semicolon),
    (":", VirtKey::Colon),
    ("'", VirtKey::Apostrophe),
    (",", VirtKey::Comma),
    (".", VirtKey::Period),
    ("/", VirtKey::Slash),
    ("Escape", VirtKey::Escape),
    ("Tab", VirtKey::Tab),
    ("Insert", VirtKey::Insert),
    ("Delete", VirtKey::Delete),
    ("Backspace", VirtKey::Back),
    ("Enter", VirtKey::Return),
    ("Home", VirtKey::Home),
    ("End", VirtKey::End),
    ("PageUp", VirtKey::PageUp),
    ("PageDown", VirtKey::PageDown),
    ("Space", VirtKey::Space),
];
