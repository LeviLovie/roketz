use nalgebra::Vector2;
use std::collections::HashMap;
use winit::{
    event::{ElementState, Modifiers, MouseButton, MouseScrollDelta, WindowEvent},
    keyboard::Key,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyState {
    Open,
    Pressed,
    Down,
    Released,
}

pub struct Inputs {
    keys: HashMap<Key, KeyState>,
    modifiers: Modifiers,
    mouse_buttons: HashMap<MouseButton, KeyState>,
    mouse_pos: Vector2<f32>,
    mouse_delta: Vector2<f32>,
    mouse_wheel: Vector2<f32>,
}

impl Default for Inputs {
    fn default() -> Self {
        Self {
            keys: HashMap::new(),
            modifiers: Modifiers::default(),
            mouse_buttons: HashMap::new(),
            mouse_pos: Vector2::zeros(),
            mouse_delta: Vector2::zeros(),
            mouse_wheel: Vector2::zeros(),
        }
    }
}

#[allow(dead_code)]
impl Inputs {
    pub fn pre_update(&mut self) {
        for state in self.keys.values_mut() {
            *state = match state {
                KeyState::Pressed => KeyState::Down,
                KeyState::Released => KeyState::Open,
                _ => state.clone(),
            };
        }
        for state in self.mouse_buttons.values_mut() {
            *state = match state {
                KeyState::Pressed => KeyState::Down,
                KeyState::Released => KeyState::Open,
                _ => state.clone(),
            };
        }
    }

    pub fn update(&mut self, window_event: &WindowEvent) {
        match window_event {
            WindowEvent::CursorMoved { position, .. } => {
                let new_pos = Vector2::new(position.x as f32, position.y as f32);
                self.mouse_delta = new_pos - self.mouse_pos;
                self.mouse_pos = new_pos;
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.mouse_wheel = match delta {
                    MouseScrollDelta::LineDelta(x, y) => Vector2::new(*x, *y) * 20.0,
                    MouseScrollDelta::PixelDelta(pos) => Vector2::new(pos.x as f32, pos.y as f32),
                };
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let button_state = match state {
                    ElementState::Pressed => KeyState::Pressed,
                    ElementState::Released => KeyState::Released,
                };
                self.mouse_buttons.insert(*button, button_state);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let key_state = match event.state {
                    ElementState::Pressed => KeyState::Pressed,
                    ElementState::Released => KeyState::Released,
                };
                self.keys.insert(event.logical_key.clone(), key_state);
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                self.modifiers = *modifiers;
            }
            _ => {}
        }
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        matches!(
            self.keys.get(&key),
            Some(KeyState::Down) | Some(KeyState::Pressed)
        )
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        matches!(self.keys.get(&key), Some(KeyState::Pressed))
    }

    pub fn is_key_released(&self, key: Key) -> bool {
        matches!(self.keys.get(&key), Some(KeyState::Released))
    }

    pub fn is_mouse_button_down(&self, button: MouseButton) -> bool {
        matches!(
            self.mouse_buttons.get(&button),
            Some(KeyState::Down) | Some(KeyState::Pressed)
        )
    }

    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        matches!(self.mouse_buttons.get(&button), Some(KeyState::Pressed))
    }

    pub fn is_mouse_button_released(&self, button: MouseButton) -> bool {
        matches!(self.mouse_buttons.get(&button), Some(KeyState::Released))
    }

    pub fn mouse_position(&self) -> Vector2<f32> {
        self.mouse_pos
    }

    pub fn mouse_delta(&self) -> Vector2<f32> {
        self.mouse_delta
    }

    pub fn mouse_wheel(&self) -> Vector2<f32> {
        self.mouse_wheel
    }
}
