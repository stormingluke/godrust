use godot::classes::{ISprite2D, InputEvent, InputEventKey, InputEventMouse, Sprite2D};
use godot::global::Key;
use godot::obj::Base;
use godot::prelude::*;
use std::collections::HashMap;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Player {
    #[base]
    base: Base<Sprite2D>,
    keys: HashMap<String, bool>,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            keys: HashMap::new(),
        }
    }
    fn physics_process(&mut self, delta: f64) {
        let radians = (3.14 * delta) as f32;
        self.base_mut().rotate(radians);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Ok(mouse) = event.clone().try_cast::<InputEventMouse>() {
            // self.base_mut().set_position(mouse.get_position());
            if mouse.is_pressed() {
                godot_print!("Mouse clicked at {:?}", mouse.get_position());
                self.base_mut().set_position(mouse.get_position());
            }
        } else if let Ok(key) = event.clone().try_cast::<InputEventKey>() {
            godot_print!("Key event: {:?}", key.get_keycode());
            let current_pos = self.base().get_position();
            match key.get_keycode() {
                Key::H => {
                    self.keys.insert("h".to_string(), true);
                }
                Key::LEFT => {
                    self.base_mut()
                        .set_position(current_pos + Vector2::new(-15.0, 0.0));
                }
                Key::RIGHT => {
                    self.base_mut()
                        .set_position(current_pos + Vector2::new(15.0, 0.0));
                }
                Key::SPACE => {
                    godot_print!("Space pressed!");
                }
                Key::UP => {
                    self.base_mut()
                        .set_position(current_pos + Vector2::new(0.0, -15.0));
                }
                Key::DOWN => {
                    self.base_mut()
                        .set_position(current_pos + Vector2::new(0.0, 15.0));
                }
                Key { .. } => {}
            }
        }
    }
}
