use bevy_ecs::prelude::*;

#[cfg(feature = "fmod")]
use crate::ecs::r::Sound;

#[derive(Resource)]
pub struct ThrustSound {
    pub is_playing_1: bool,
    pub is_playing_2: bool,
}

impl ThrustSound {
    pub fn set(&mut self, is_player_1: bool, new_value: bool) {
        if is_player_1 {
            self.is_playing_1 = new_value;
        } else {
            self.is_playing_2 = new_value;
        }
    }
}

pub fn init_thrust_sound(mut commands: Commands) {
    commands.insert_resource(ThrustSound {
        is_playing_1: false,
        is_playing_2: false,
    });
}

#[cfg(feature = "fmod")]
pub fn update_thrust_sound(thrust_sound: Res<ThrustSound>, sound: ResMut<Sound>) {
    if !thrust_sound.is_playing_1 && !thrust_sound.is_playing_2 {
        sound
            .borrow()
            .stop_looping(sound::bindings::EVENT_GAMEPLAY_THRUST)
            .unwrap_or_else(|e| tracing::error!("Failed to stop thrust sound: {}", e));
    } else {
        sound
            .borrow()
            .play_looping(sound::bindings::EVENT_GAMEPLAY_THRUST)
            .unwrap_or_else(|e| tracing::error!("Failed to play thrust sound: {}", e));
    }
}

#[cfg(not(feature = "fmod"))]
pub fn update_thrust_sound() {}
