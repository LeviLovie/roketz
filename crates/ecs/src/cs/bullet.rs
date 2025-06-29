use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use crate::{
    cs::{RigidCollider, Terrain, Transform},
    r::{PhysicsWorld, DT},
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
            BulletType::Grenade => 3.0,
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
            BulletType::Simple => 0.1,
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
