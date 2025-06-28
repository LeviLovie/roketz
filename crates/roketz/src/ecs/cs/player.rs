use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use super::{Bullet, BulletType, Physics, Transform};
use crate::ecs::{
    cs::Terrain,
    res::{DT, Gravity},
};

#[derive(Component)]
pub struct Player {
    pub color: Color,
    pub thrust: f32,
    pub rotation_speed: f32,
    pub is_player_1: bool,
    pub bullet_type: BulletType,
    pub is_dead: bool,
    pub health: f32,
    pub respawn_time: f32,
    pub bullet_cooldown: f32,
}

impl Player {
    pub fn new(color: Color, is_player_1: bool) -> Self {
        Self {
            color,
            thrust: 150.0,
            rotation_speed: 5.0,
            bullet_type: BulletType::Simple,
            is_player_1,
            is_dead: false,
            health: 100.0,
            respawn_time: 0.0,
            bullet_cooldown: 0.0,
        }
    }

    pub fn is_alive(&self) -> bool {
        !self.is_dead
    }

    pub fn respawn(&mut self) {
        self.is_dead = false;
        self.health = 100.0;
        self.respawn_time = 0.0;
    }

    pub fn damage(&mut self, amount: f32) {
        self.health -= amount;
        if self.health <= 0.0 {
            self.is_dead = true;
            self.respawn_time = 5.0;
        }
    }
}

pub fn update_players(
    mut query: Query<(&mut Player, &mut Transform, &mut Physics)>,
    mut commands: Commands,
    dt: Res<DT>,
    gravity: Res<Gravity>,
) {
    for (mut player, mut transform, mut physics) in query.iter_mut() {
        if !player.is_alive() {
            if player.respawn_time < dt.0 {
                player.respawn()
            } else {
                player.respawn_time -= dt.0;
            }
            continue;
        }

        if player.bullet_cooldown < dt.0 {
            player.bullet_cooldown = 0.0;
        } else {
            player.bullet_cooldown -= dt.0;
        }

        if player.is_player_1 && is_key_pressed(KeyCode::Q)
            || !player.is_player_1 && is_key_pressed(KeyCode::U)
        {
            player.bullet_type = player.bullet_type.prev();
        } else if player.is_player_1 && is_key_pressed(KeyCode::E)
            || !player.is_player_1 && is_key_pressed(KeyCode::O)
        {
            player.bullet_type = player.bullet_type.next();
        }

        if (player.is_player_1 && is_key_down(KeyCode::Space)
            || !player.is_player_1 && is_key_down(KeyCode::Semicolon))
            && player.bullet_cooldown <= 0.0
        {
            player.bullet_cooldown = player.bullet_type.cooldown();
            commands.spawn((
                Bullet::new(player.bullet_type, transform.angle),
                Transform::from_pos(
                    transform.pos + vec2(transform.angle.cos(), transform.angle.sin()) * 5.0,
                ),
            ));
        }

        if player.is_player_1 && is_key_down(KeyCode::A)
            || !player.is_player_1 && is_key_down(KeyCode::J)
        {
            transform.angle -= player.rotation_speed * dt.0;
        } else if player.is_player_1 && is_key_down(KeyCode::D)
            || !player.is_player_1 && is_key_down(KeyCode::L)
        {
            transform.angle += player.rotation_speed * dt.0;
        }

        let mut force = Vec2::ZERO;
        if player.is_player_1 && is_key_down(KeyCode::W)
            || !player.is_player_1 && is_key_down(KeyCode::I)
        {
            force += vec2(transform.angle.cos(), transform.angle.sin()) * player.thrust;
        }
        force.y += gravity.0 * physics.mass;

        physics.acc = force / physics.mass;
    }
}

pub fn draw_players(query: Query<(&Player, &Transform)>) {
    for (p, t) in query.iter() {
        draw_circle(t.pos.x, t.pos.y, 3.0, p.color);
        draw_line(
            t.pos.x,
            t.pos.y,
            t.pos.x + t.angle.cos() * 5.0,
            t.pos.y + t.angle.sin() * 5.0,
            2.0,
            p.color,
        );
    }
}

pub fn check_player_terrain_collisions(
    players: Query<(&mut Transform, &Player)>,
    terrain: Query<&Terrain>,
) {
    if let Ok(terrain) = terrain.single() {
        for (mut transform, _) in players {
            let mut total_push = vec2(0.0, 0.0);
            let nearby = terrain.bvh.get_nearby_nodes(transform.pos, 20.0);

            for (_, bounds) in nearby {
                let mut pos = transform.pos;
                if bounds.push_circle_out(&mut pos, 3.0) {
                    let push = pos - transform.pos;
                    total_push += push;
                }
            }

            transform.pos += total_push;
        }
    }
}

pub fn check_player_bullet_collisions(
    mut commands: Commands,
    players: Query<(&mut Player, &Transform)>,
    bullets: Query<(Entity, &Bullet, &Transform)>,
) {
    for (mut player, player_transform) in players {
        for (bullet_entity, bullet, bullet_transform) in bullets.iter() {
            if player_transform.pos.distance(bullet_transform.pos) < bullet.ty.radius() {
                commands.entity(bullet_entity).despawn();
                player.damage(bullet.ty.damage());
            }
        }
    }
}
