use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use rapier2d::prelude::*;

use crate::{
    cs::{Bullet, BulletType, RigidCollider, Transform},
    r::{PhysicsWorld, Sound, DT},
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
            rotation_speed: 400.0,
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
    mut commands: Commands,
    mut query: Query<(&mut Player, &mut Transform, &RigidCollider)>,
    physics: ResMut<PhysicsWorld>,
    dt: Res<DT>,
    sound: Res<Sound>,
) {
    let mut physics: Mut<PhysicsWorld> = physics.into();
    for (mut player, transform, collider) in query.iter_mut() {
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
            let bullet_pos =
                transform.pos + vec2(transform.angle.cos(), transform.angle.sin()) * 5.0;
            let bullet_vel =
                vec2(transform.angle.cos(), transform.angle.sin()) * player.bullet_type.speed();
            commands.spawn((
                Bullet::new(player.bullet_type, transform.angle),
                RigidCollider::dynamic(
                    &mut physics,
                    ColliderBuilder::ball(player.bullet_type.radius()).build(),
                    vector![bullet_pos.x, bullet_pos.y],
                    vector![bullet_vel.x, bullet_vel.y],
                    0.0,
                ),
                Transform::from_pos(
                    transform.pos + vec2(transform.angle.cos(), transform.angle.sin()) * 5.0,
                ),
            ));
        }

        let PhysicsWorld { bodies, .. } = &mut *physics;
        if let Some(rb) = bodies.get_mut(collider.body) {
            let mut linvel = *rb.linvel();
            let forward = vector![transform.angle.cos(), transform.angle.sin()];
            if (player.is_player_1 && is_key_down(KeyCode::W))
                || (!player.is_player_1 && is_key_down(KeyCode::I))
            {
                linvel += forward * player.thrust * dt.0;
                sound
                    .borrow()
                    .play_looping("event:/gameplay/thrust")
                    .unwrap_or_else(|e| error!("Failed to play thrust sound: {}", e));
            } else {
                sound
                    .borrow()
                    .stop_looping("event:/gameplay/thrust")
                    .unwrap_or_else(|e| error!("Failed to stop thrust sound: {}", e));
            }
            rb.set_linvel(linvel, true);

            let angvel;
            if (player.is_player_1 && is_key_down(KeyCode::A))
                || (!player.is_player_1 && is_key_down(KeyCode::J))
            {
                angvel = -player.rotation_speed * dt.0;
            } else if (player.is_player_1 && is_key_down(KeyCode::D))
                || (!player.is_player_1 && is_key_down(KeyCode::L))
            {
                angvel = player.rotation_speed * dt.0;
            } else {
                angvel = 0.0;
            }
            rb.set_angvel(angvel, true);
        }
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

pub fn ui_players(mut query: Query<&mut Player>) {
    for player in query.iter_mut() {
        const WIDTH: f32 = 200.0;
        const HEIGHT: f32 = 15.0;
        const MARGIN: f32 = 4.0;
        let x = if player.is_player_1 {
            MARGIN
        } else {
            screen_width() - WIDTH - MARGIN
        };
        let y = screen_height() - HEIGHT - MARGIN;

        {
            let health_percentage = player.health / 100.0;
            let health_bar_width = WIDTH * health_percentage;
            let health_bar_color = if health_percentage > 0.5 {
                Color::from_rgba(0, 255, 0, 100)
            } else if health_percentage > 0.2 {
                Color::from_rgba(255, 255, 0, 100)
            } else {
                Color::from_rgba(255, 0, 0, 100)
            };
            draw_rectangle(x, y, WIDTH, HEIGHT, Color::from_rgba(0, 0, 0, 100));
            draw_rectangle(x, y, health_bar_width, HEIGHT, health_bar_color);
        };

        {
            let text = format!("{}", player.bullet_type);
            let text_width = measure_text(text.as_str(), None, 20, 1.0).width;
            let x = if player.is_player_1 {
                MARGIN
            } else {
                screen_width() - text_width - MARGIN
            };

            draw_text(&text, x, y + HEIGHT - MARGIN * 4.0 - 2.0, 20.0, WHITE);
        };
    }
}
