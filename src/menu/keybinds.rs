use minifb::Key;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Clone, Hash, PartialEq, Eq)]
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

//helper function thingy
pub fn key_for(action: &Action, bindings: &HashMap<Action, Key>) -> Key {
    bindings
        .get(action)
        .copied()
        .expect("Action should have a binding")
}

//TODO:

//function that checks if keybinding file is there, and then either returns keybindings from file or
//generates default keybinds
pub fn load_keybinds(filepath: PathBuf) -> HashMap<Action, Key> {
    generate_default_keybindings()
}

//get hashmap from file
//
//print hashmap to file
