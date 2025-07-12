use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use rapier2d::prelude::*;
use tracing::error;

use crate::{
    cs::{RigidCollider, Terrain, TerrainCollider, Transform},
    r::{Collisions, DT, PhysicsWorld, Sound},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BulletType {
    Simple,
    Grenade,
    Dynamite,
}

impl BulletType {
    pub fn speed(&self) -> f32 {
        match self {
            BulletType::Simple => 500.0,
            BulletType::Grenade => 300.0,
            BulletType::Dynamite => 100.0,
        }
    }

    pub fn damage(&self) -> f32 {
        match self {
            BulletType::Simple => 10.0,
            BulletType::Grenade => 20.0,
            BulletType::Dynamite => 50.0,
        }
    }

    pub fn explosive(&self) -> bool {
        match self {
            BulletType::Simple => false,
            BulletType::Grenade => true,
            BulletType::Dynamite => true,
        }
    }

    pub fn radius(&self) -> f32 {
        match self {
            BulletType::Simple => 1.0,
            BulletType::Grenade => 3.0,
            BulletType::Dynamite => 3.0,
        }
    }

    pub fn explosion_radius(&self) -> f32 {
        match self {
            BulletType::Simple => 0.0,
            BulletType::Grenade => 5.0,
            BulletType::Dynamite => 15.0,
        }
    }

    pub fn lifetime(&self) -> f32 {
        match self {
            BulletType::Simple => 3.0,
            BulletType::Grenade => 4.0,
            BulletType::Dynamite => 2.0,
        }
    }

    pub fn cooldown(&self) -> f32 {
        match self {
            BulletType::Simple => 0.25,
            BulletType::Grenade => 0.5,
            BulletType::Dynamite => 4.0,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            BulletType::Simple => BulletType::Dynamite,
            BulletType::Grenade => BulletType::Simple,
            BulletType::Dynamite => BulletType::Grenade,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            BulletType::Simple => BulletType::Grenade,
            BulletType::Grenade => BulletType::Dynamite,
            BulletType::Dynamite => BulletType::Simple,
        }
    }

    pub fn sound_fire(&self) -> Option<&'static str> {
        match self {
            BulletType::Simple => Some("event:/bullets/fire_simple"),
            _ => None,
        }
    }

    pub fn sound_hit(&self) -> Option<&'static str> {
        match self {
            BulletType::Simple => Some("event:/bullets/hit_simple"),
            BulletType::Grenade => Some("event:/bullets/hit_grenade"),
            BulletType::Dynamite => Some("event:/bullets/hit_grenade"),
        }
    }
}

impl std::fmt::Display for BulletType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BulletType::Simple => write!(f, "Simple"),
            BulletType::Grenade => write!(f, "Grenade"),
            BulletType::Dynamite => write!(f, "Dynamite"),
        }
    }
}

#[derive(Component)]
pub struct Bullet {
    pub ty: BulletType,
    pub vel: Vec2,
    pub lifetime: f32,
}

impl Bullet {
    pub fn new(ty: BulletType, dir: f32) -> Self {
        Self {
            ty,
            vel: Vec2::new(dir.cos(), dir.sin()) * ty.speed(),
            lifetime: ty.lifetime(),
        }
    }
}

pub fn update_bullets(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Bullet, &mut Transform, &mut RigidCollider)>,
    mut terrain: Query<&mut Terrain>,
    dt: Res<DT>,
    world: ResMut<PhysicsWorld>,
) {
    let mut world: Mut<PhysicsWorld> = world.into();
    for (entity, mut bullet, mut transform, mut collider) in bullets.iter_mut() {
        if bullet.lifetime <= dt.0 {
            if bullet.ty.explosive()
                && let Some(mut terrain) = terrain.iter_mut().next()
                && let Err(e) = terrain.destruct(
                    transform.pos.x as u32,
                    transform.pos.y as u32,
                    bullet.ty.explosion_radius() as u32,
                )
            {
                error!("Failed to destruct terrain: {}", e);
            }
            collider.despawn(&mut world);
            commands.entity(entity).despawn();
            continue;
        } else {
            bullet.lifetime -= dt.0;
        }

        transform.pos += bullet.vel * dt.0;
    }
}

pub fn draw_bullets(query: Query<(&Bullet, &Transform)>) {
    for (bullet, transform) in query.iter() {
        let radius = bullet.ty.radius();
        draw_circle(transform.pos.x, transform.pos.y, radius, WHITE);
    }
}

pub fn handle_bullet_terrain_collisions(
    mut commands: Commands,
    mut bullets: Query<(Entity, &Bullet, &Transform, &mut RigidCollider)>,
    terrain_colliders: Query<&RigidCollider, (With<TerrainCollider>, Without<Bullet>)>,
    mut terrain: Query<&mut Terrain>,
    physics: ResMut<PhysicsWorld>,
    collisions: Res<Collisions>,
    sound: Res<Sound>,
) {
    let mut physics: Mut<PhysicsWorld> = physics.into();

    for event in collisions.0.iter() {
        if let CollisionEvent::Started(h1, h2, _flags) = event {
            let bullet = bullets
                .iter_mut()
                .find(|(_, _, _, col)| col.collider == *h1 || col.collider == *h2);
            let terrain_hit = terrain_colliders
                .iter()
                .find(|col| col.collider == *h1 || col.collider == *h2);

            if let (Some((entity, bullet, transform, mut collider)), Some(_)) =
                (bullet, terrain_hit)
            {
                if let Some(hit_sound) = bullet.ty.sound_hit() {
                    if let Err(e) = sound.borrow().play(hit_sound) {
                        error!("Failed to play bullet hit sound: {}", e);
                    }
                }

                if bullet.ty.explosive() {
                    if let Ok(mut terrain) = terrain.single_mut() {
                        if let Err(e) = terrain.destruct(
                            transform.pos.x as u32,
                            transform.pos.y as u32,
                            bullet.ty.explosion_radius() as u32,
                        ) {
                            error!("Failed to destruct terrain: {}", e);
                        }
                    }
                }

                // It is necessary to use `try_remove` instead of `remove` because a bullet
                // might be colliding with multiple colliders and therefore will be despawned
                // twice causing a warning to be emmited.
                commands.entity(entity).try_despawn();
                collider.despawn(&mut physics);
            }
        }
    }
}
