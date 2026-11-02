//TODO:

use minifb::Key;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
pub enum action {
    move_forward,
    move_left,
    move_back,
    move_right,
    sprint,
    crouch,
    jump,
    rocket_jump,
    interact,
    crosshair_up,
    crosshair_down,
    crosshair_left,
    crosshair_right,
    fine_aiming,
    fire_bullet,
    fire_arrow,
    godmode,
    lock,
    escape,
}

pub fn generate_default_keybindings() -> HashMap {
    let mut action_map = HashMap::new();
    action_map.insert(action::move_forward, Key::W);
    action_map.insert(action::move_left, Key::A);
    action_map.insert(action::move_back, Key::S);
    action_map.insert(action::move_right, Key::D);
    action_map.insert(action::sprint, Key::LeftShift);
    action_map.insert(action::jump, Key::Space);
    action_map.insert(action::crouch, Key::C);
    action_map.insert(action::rocket_jump, Key::R);
    action_map.insert(action::interact, Key::F);
    action_map.insert(action::crosshair_up, Key::Up);
    action_map.insert(action::crosshair_down, Key::Down);
    action_map.insert(action::crosshair_left, Key::Left);
    action_map.insert(action::crosshair_right, Key::Right);
    action_map.insert(action::fine_aiming, Key::E);
    action_map.insert(action::fire_bullet, Key::RightCtrl);
    action_map.insert(action::fire_arrow, Key::RightShift);
    action_map.insert(action::godmode, Key::G);
    action_map.insert(action::lock, Key::L);
    action_map.insert(action::escape, Key::Escape);
    action_map
}
