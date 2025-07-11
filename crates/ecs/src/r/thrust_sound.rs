use bevy_ecs::prelude::*;
use tracing::error;

#[cfg(feature = "fmod")]
use crate::r::Sound;

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

pub fn update_thrust_sound(
    #[cfg(feature = "fmod")] thrust_sound: Res<ThrustSound>,
    #[cfg(feature = "fmod")] sound: ResMut<Sound>,
) {
    #[cfg(feature = "fmod")]
    {
        if !thrust_sound.is_playing_1 && !thrust_sound.is_playing_2 {
            sound
                .borrow()
                .stop_looping(sound::bindings::EVENT_GAMEPLAY_THRUST)
                .unwrap_or_else(|e| error!("Failed to stop thrust sound: {}", e));
        } else {
            sound
                .borrow()
                .play_looping(sound::bindings::EVENT_GAMEPLAY_THRUST)
                .unwrap_or_else(|e| error!("Failed to play thrust sound: {}", e));
        }
    }
    #[cfg(not(feature = "fmod"))]
    {
        error!("Sound engine is not enabled. Compile with the 'fmod' feature.");
    }
}
