use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use crate::{cs::Transform, r::DT};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BulletType {
    Simple,
    Grenade,
}

impl BulletType {
    pub fn speed(&self) -> f32 {
        match self {
            BulletType::Simple => 500.0,
            BulletType::Grenade => 300.0,
        }
    }

    pub fn damage(&self) -> f32 {
        match self {
            BulletType::Simple => 10.0,
            BulletType::Grenade => 20.0,
        }
    }

    pub fn radius(&self) -> f32 {
        match self {
            BulletType::Simple => 1.0,
            BulletType::Grenade => 3.0,
        }
    }

    pub fn lifetime(&self) -> f32 {
        match self {
            BulletType::Simple => 3.0,
            BulletType::Grenade => 5.0,
        }
    }

    pub fn cooldown(&self) -> f32 {
        match self {
            BulletType::Simple => 0.1,
            BulletType::Grenade => 0.5,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            BulletType::Simple => BulletType::Grenade,
            BulletType::Grenade => BulletType::Simple,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            BulletType::Simple => BulletType::Grenade,
            BulletType::Grenade => BulletType::Simple,
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
    mut bullets: Query<(Entity, &mut Bullet, &mut Transform)>,
    dt: Res<DT>,
) {
    for (entity, mut bullet, mut transform) in bullets.iter_mut() {
        if bullet.lifetime <= dt.0 {
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
