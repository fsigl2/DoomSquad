use minifb::Key;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::path::PathBuf;

pub const KEYBINDS_PATH: &str = "keybinds.json";

#[derive(Copy, Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    MoveForward,
    MoveLeft,
    MoveBack,
    MoveRight,
    Sprint,
    Crouch,
    Jump,
    RocketJump,
    Interact,
    CrosshairUp,
    CrosshairDown,
    CrosshairLeft,
    CrosshairRight,
    FineAiming,
    FireBullet,
    FireArrow,
    Godmode,
    Lock,
    Escape,
}

pub fn generate_default_keybindings() -> HashMap<Action, Key> {
    let mut action_map = HashMap::new();
    action_map.insert(Action::MoveForward, Key::W);
    action_map.insert(Action::MoveLeft, Key::A);
    action_map.insert(Action::MoveBack, Key::S);
    action_map.insert(Action::MoveRight, Key::D);
    action_map.insert(Action::Sprint, Key::LeftShift);
    action_map.insert(Action::Jump, Key::Space);
    action_map.insert(Action::Crouch, Key::C);
    action_map.insert(Action::RocketJump, Key::R);
    action_map.insert(Action::Interact, Key::F);
    action_map.insert(Action::CrosshairUp, Key::Up);
    action_map.insert(Action::CrosshairDown, Key::Down);
    action_map.insert(Action::CrosshairLeft, Key::Left);
    action_map.insert(Action::CrosshairRight, Key::Right);
    action_map.insert(Action::FineAiming, Key::E);
    action_map.insert(Action::FireBullet, Key::RightCtrl);
    action_map.insert(Action::FireArrow, Key::RightShift);
    action_map.insert(Action::Godmode, Key::G);
    action_map.insert(Action::Lock, Key::L);
    action_map.insert(Action::Escape, Key::Escape);
    action_map
}

//helper function that takes binding out of hashmap and formats it so it can be used by minifb
pub fn key_for(action: &Action, bindings: &HashMap<Action, Key>) -> Key {
    bindings
        .get(action)
        .copied()
        .expect("Action should have a binding")
}

//function that checks if keybinding file is there, and then either returns keybindings from file or
//generates default keybinds
pub fn load_keybinds(filepath: PathBuf) -> HashMap<Action, Key> {
    if filepath.exists() {
        let data =
            fs::read_to_string(filepath).expect("Should be able to read file, since it exists");
        let str_map: HashMap<Action, String> = serde_json::from_str(&data).unwrap();
        let mut result: HashMap<Action, Key> = generate_default_keybindings();
        for (action, key_str) in str_map {
            if let Some(key) = string_to_key(&key_str) {
                result.insert(action, key);
            }
        }
        result
    } else {
        generate_default_keybindings()
    }
}

fn string_to_key(data: &str) -> Option<Key> {
    ALL_KEYS
        .iter()
        .find(|&&x| key_to_string(x) == data)
        .copied()
}

//convert key to string since minifb keys cannot be printed to files
fn key_to_string(key: Key) -> String {
    format!("{:?}", key)
}

//save hashmap to file
pub fn save_keybinds(new_map: &HashMap<Action, Key>, filepath: PathBuf) {
    let mut str_map: HashMap<Action, String> = HashMap::new();
    for (action, key) in new_map {
        str_map.insert(*action, key_to_string(*key));
    }
    let output = serde_json::to_string_pretty(&str_map).expect("this should always work");
    if let Err(e) = fs::write(filepath, output) {
        eprintln!("Could not work with file: {}", e);
    }
}

pub const ALL_ACTIONS: &[Action] = &[
    Action::MoveForward,
    Action::MoveLeft,
    Action::MoveBack,
    Action::MoveRight,
    Action::Sprint,
    Action::Crouch,
    Action::Jump,
    Action::RocketJump,
    Action::Interact,
    Action::CrosshairUp,
    Action::CrosshairDown,
    Action::CrosshairLeft,
    Action::CrosshairRight,
    Action::FineAiming,
    Action::FireBullet,
    Action::FireArrow,
    Action::Godmode,
    Action::Lock,
    Action::Escape,
];

const ALL_KEYS: &[Key] = &[
    Key::Key0,
    Key::Key1,
    Key::Key2,
    Key::Key3,
    Key::Key4,
    Key::Key5,
    Key::Key6,
    Key::Key7,
    Key::Key8,
    Key::Key9,
    Key::A,
    Key::B,
    Key::C,
    Key::D,
    Key::E,
    Key::F,
    Key::G,
    Key::H,
    Key::I,
    Key::J,
    Key::K,
    Key::L,
    Key::M,
    Key::N,
    Key::O,
    Key::P,
    Key::Q,
    Key::R,
    Key::S,
    Key::T,
    Key::U,
    Key::V,
    Key::W,
    Key::X,
    Key::Y,
    Key::Z,
    Key::F1,
    Key::F2,
    Key::F3,
    Key::F4,
    Key::F5,
    Key::F6,
    Key::F7,
    Key::F8,
    Key::F9,
    Key::F10,
    Key::F11,
    Key::F12,
    Key::F13,
    Key::F14,
    Key::F15,
    Key::Down,
    Key::Left,
    Key::Right,
    Key::Up,
    Key::Apostrophe,
    Key::Backquote,
    Key::Backslash,
    Key::Comma,
    Key::Equal,
    Key::LeftBracket,
    Key::Minus,
    Key::Period,
    Key::RightBracket,
    Key::Semicolon,
    Key::Slash,
    Key::Backspace,
    Key::Delete,
    Key::End,
    Key::Enter,
    Key::Escape,
    Key::Home,
    Key::Insert,
    Key::Menu,
    Key::PageDown,
    Key::PageUp,
    Key::Pause,
    Key::Space,
    Key::Tab,
    Key::NumLock,
    Key::CapsLock,
    Key::ScrollLock,
    Key::LeftShift,
    Key::RightShift,
    Key::LeftCtrl,
    Key::RightCtrl,
    Key::NumPad0,
    Key::NumPad1,
    Key::NumPad2,
    Key::NumPad3,
    Key::NumPad4,
    Key::NumPad5,
    Key::NumPad6,
    Key::NumPad7,
    Key::NumPad8,
    Key::NumPad9,
    Key::NumPadDot,
    Key::NumPadSlash,
    Key::NumPadAsterisk,
    Key::NumPadMinus,
    Key::NumPadPlus,
    Key::NumPadEnter,
    Key::LeftAlt,
    Key::RightAlt,
    Key::LeftSuper,
    Key::RightSuper,
];
