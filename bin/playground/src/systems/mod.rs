use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use crate::resources::Selection;
use bonk2d::{Circle, Collider, ColliderTrait, Transform};

pub fn reset_selection(mut selection: ResMut<Selection>) {
    selection.changed = false;
}

pub fn render_circles(query: Query<(&Transform, &Circle)>) {
    for (transform, circle) in query.iter() {
        let pos = *transform.pos();
        let res_pos = *transform.res_pos();
        let vel = *transform.vel();

        draw_circle(
            pos.x,
            pos.y,
            circle.radius,
            Color::from_rgba(255, 255, 255, 100),
        );
        draw_circle_lines(pos.x, pos.y, circle.radius - 1.0, 1.0, BLUE);

        if vel != glam::Vec2::ZERO {
            draw_line(pos.x, pos.y, pos.x + vel.x, pos.y + vel.y, 1.0, WHITE);
            draw_circle_lines(
                pos.x + vel.x,
                pos.y + vel.y,
                circle.radius - 1.0,
                1.0,
                GREEN,
            );
        }

        if res_pos != pos {
            draw_circle_lines(res_pos.x, res_pos.y, circle.radius, 1.0, ORANGE);
        }
    }
}

pub fn update_circles(
    mut query: Query<(Entity, &mut Transform, &Circle)>,
    mut selection: ResMut<Selection>,
) {
    for (_, mut transform, _) in query.iter_mut() {
        let pos = *transform.pos();
        transform.set_res_pos(pos);
    }

    let mouse_pos: Vec2 = mouse_position().into();

    if is_mouse_button_pressed(MouseButton::Right) {
        for (entity, transform, circle) in query.iter_mut() {
            if selection.changed {
                continue;
            }

            if mouse_pos.distance(*transform.pos()) < circle.radius {
                selection.changed = true;
                selection.entity = Some(entity);
            }
        }
    } else if is_key_pressed(KeyCode::F) {
        for (entity, mut transform, _) in query.iter_mut() {
            if let Some(selected_entity) = selection.entity
                && selected_entity == entity
            {
                transform.set_pos(mouse_pos);
            }
        }
    }
}

pub fn process_physics(
    mut param_set: ParamSet<(
        Query<(Entity, &Transform, &Circle)>,
        Query<(Entity, &mut Transform)>,
    )>,
) {
    let p1 = param_set.p0();
    let snapshots = p1
        .iter()
        .map(|(entity, transform, circle)| (entity, transform, circle))
        .collect::<Vec<_>>();

    let mut updates = Vec::new();

    for i in 0..snapshots.len() {
        let (entity_a, transform_a, circle_a) = snapshots[i];
        let delta_a = glam::Vec2::ZERO;

        for j in (i + 1)..snapshots.len() {
            let (entity_b, transform_b, circle_b) = snapshots[j];

            let other = Collider::Circle(circle_b.clone());
            if circle_a.collides(transform_a, &other, transform_b).unwrap() {
                println!(
                    "Collision detected between {:?} and {:?}",
                    entity_a, entity_b
                );
            }
        }

        if let Ok((_, mut transform_a)) = param_set.p1().get_mut(entity_a) {
            let res_a = *transform_a.res_pos();
            transform_a.set_res_pos(res_a + delta_a);
        }
    }
}
