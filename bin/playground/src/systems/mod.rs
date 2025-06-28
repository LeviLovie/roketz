use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use crate::resources::RenderConfig;
use bonk2d::{Circle, Transform};

pub fn render_circles(query: Query<(&Transform, &Circle)>, config: Res<RenderConfig>) {
    for (transform, circle) in query.iter() {
        let pos = *transform.pos();
        let radius = circle.radius;
        if config.filled_in {
            draw_circle(pos.x, pos.y, radius, WHITE);
        } else {
            draw_circle_lines(pos.x, pos.y, radius - 1.0, 1.0, WHITE);
        }
    }
}
