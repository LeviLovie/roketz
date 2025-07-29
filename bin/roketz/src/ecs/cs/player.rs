use anyhow::{Context, Result};
use bevy_ecs::prelude::*;
use helpers::error::HandleError;
use macroquad::prelude::*;
use rapier2d::prelude::*;
use rdss::Loader;
use std::sync::{Arc, Mutex};
use tracing::error;

use crate::{
    ecs::{
        cs::{Bullet, BulletType, RigidCollider, Transform},
        r::{Collisions, PhysicsWorld, Sound, ThrustSound, DT},
    },
    sprites::{kinds, SpriteKind, Sprites},
};

#[derive(Component)]
pub struct Player {
    pub texture_idle: Texture2D,
    pub sprite_idle: kinds::Simple,
    pub texture_moving: Texture2D,
    pub sprite_moving: kinds::Simple,
    pub color: Color,
    pub thrust: f32,
    pub rotation_speed: f32,
    pub is_player_1: bool,
    pub bullet_type: BulletType,
    pub is_dead: bool,
    pub health: f32,
    pub respawn_time: f32,
    pub bullet_cooldown: f32,
    pub is_moving: bool,
}

impl Player {
    pub fn new(
        sprites: Arc<Mutex<Sprites>>,
        assets: Arc<Mutex<Loader>>,
        color: Color,
        is_player_1: bool,
    ) -> Result<Self> {
        fn load_texture(
            sprites: Arc<Mutex<Sprites>>,
            assets: Arc<Mutex<Loader>>,
            name: &str,
        ) -> Result<(kinds::Simple, Texture2D)> {
            let rocket_sprite = match sprites
                .lock()
                .handle("Failed to lock sprites mutex")
                .find(name)
                .context(format!("Failed to find sprite {}", name))?
            {
                SpriteKind::Simple(simple) => simple,
            };
            let rocket_path = Sprites::to_absolute_path(rocket_sprite.path.clone());
            let rocket_file = assets
                .lock()
                .handle("Failed to lock assets mutex")
                .read_raw(&rocket_path)
                .context(format!("Failed to read {}", rocket_path))?;
            let rocket_image = Image::from_file_with_format(&rocket_file, None)
                .context("Failed to load an image")?;
            let mut texture = Texture2D::from_image(&rocket_image);
            texture.set_filter(FilterMode::Nearest);
            Ok((rocket_sprite, texture))
        }

        let (sprite_idle, texture_idle) =
            load_texture(sprites.clone(), assets.clone(), "rocket_idle")
                .context("Failed to load rocket_idle texture")?;
        let (sprite_moving, texture_moving) =
            load_texture(sprites.clone(), assets.clone(), "rocket_moving")
                .context("Failder to load rocket_moving texture")?;

        Ok(Self {
            texture_idle,
            sprite_idle,
            texture_moving,
            sprite_moving,
            color,
            thrust: 150.0,
            rotation_speed: 400.0,
            bullet_type: BulletType::Simple,
            is_player_1,
            is_dead: false,
            health: 100.0,
            respawn_time: 0.0,
            bullet_cooldown: 0.0,
            is_moving: false,
        })
    }

    pub fn respawn(&mut self) {
        self.is_dead = false;
        self.health = 100.0;
        self.respawn_time = 0.0;
    }

    pub fn damage(&mut self, amount: f32) {
        if self.is_dead {
            return;
        }

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
    mut thrust_sound: ResMut<ThrustSound>,
    sound: Res<Sound>,
) {
    let mut physics: Mut<PhysicsWorld> = physics.into();
    for (mut player, transform, collider) in query.iter_mut() {
        if player.is_dead {
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
                transform.pos + vec2(transform.angle.cos(), transform.angle.sin()) * 7.5;
            let bullet_vel =
                vec2(transform.angle.cos(), transform.angle.sin()) * player.bullet_type.speed();
            commands.spawn((
                Bullet::new(player.bullet_type, transform.angle),
                RigidCollider::dynamic(
                    &mut physics,
                    ColliderBuilder::ball(player.bullet_type.radius()),
                    vector![bullet_pos.x, bullet_pos.y],
                    vector![bullet_vel.x, bullet_vel.y],
                    0.0,
                ),
                Transform::from_pos(
                    transform.pos + vec2(transform.angle.cos(), transform.angle.sin()) * 5.0,
                ),
            ));
            if let Some(fire_sound) = player.bullet_type.sound_fire() {
                if let Err(e) = sound.borrow().play(fire_sound) {
                    error!("Failed to play bullet fire sound: {}", e);
                }
            }
        }

        let PhysicsWorld { bodies, .. } = &mut *physics;
        if let Some(rb) = bodies.get_mut(collider.body) {
            let mut linvel = *rb.linvel();
            let forward = vector![transform.angle.cos(), transform.angle.sin()];
            if (player.is_player_1 && is_key_down(KeyCode::W))
                || (!player.is_player_1 && is_key_down(KeyCode::I))
            {
                linvel += forward * player.thrust * dt.0;
                thrust_sound.set(player.is_player_1, true);
                player.is_moving = true;
            } else {
                thrust_sound.set(player.is_player_1, false);
                player.is_moving = false;
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

pub fn handle_player_bullet_collisions(
    mut commands: Commands,
    mut players: Query<(&mut Player, &RigidCollider), Without<Bullet>>,
    mut bullets: Query<(Entity, &Bullet, &mut RigidCollider), Without<Player>>,
    collisions: Res<Collisions>,
    physics: ResMut<PhysicsWorld>,
) {
    let mut physics: Mut<PhysicsWorld> = physics.into();
    for event in collisions.0.iter() {
        if let CollisionEvent::Started(h1, h2, _flags) = event {
            let players = players
                .iter_mut()
                .find(|(_, col)| col.collider == *h1 || col.collider == *h2);
            let bullets = bullets
                .iter_mut()
                .find(|(_, _, col)| col.collider == *h1 || col.collider == *h2);

            if let (Some((mut player, _)), Some((bullet_entity, bullet, mut bullet_collider))) =
                (players, bullets)
            {
                player.damage(bullet.ty.damage());

                // It is necessary to use `try_remove` instead of `remove` because a bullet
                // might be colliding multiple times and therefore will be despawned
                // twice causing a warning to be emmited.
                commands.entity(bullet_entity).try_despawn();
                bullet_collider.despawn(&mut physics);
            }
        }
    }
}

pub fn draw_players(query: Query<(&Player, &Transform)>) {
    for (p, t) in query.iter() {
        if !p.is_dead {
            let (texture, origin) = if p.is_moving {
                (&p.texture_moving, &p.sprite_moving.origin)
            } else {
                (&p.texture_idle, &p.sprite_idle.origin)
            };

            let scale = 2.0;
            draw_texture_ex(
                texture,
                t.pos.x - origin.x as f32 / scale,
                t.pos.y - origin.y as f32 / scale,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(texture.size() / scale),
                    // 0º in rapier2d is 3 hours, but 0º on the texture is 12 hours.
                    rotation: t.angle + std::f32::consts::PI / 2.0,
                    pivot: Some(vec2(t.pos.x, t.pos.y)),
                    ..Default::default()
                },
            );
        }
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
