use macroquad::prelude::*;
use std::sync::{Arc, Mutex};

use super::Terrain;
use crate::game::GameData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BulletType {
    Simple,
    Shrapnel,
}

impl std::fmt::Display for BulletType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BulletType::Simple => write!(f, "Simple"),
            BulletType::Shrapnel => write!(f, "Shrapnel"),
        }
    }
}

impl BulletType {
    #[inline]
    pub fn variants() -> Vec<BulletType> {
        vec![BulletType::Simple, BulletType::Shrapnel]
    }

    #[inline]
    pub fn lifetime(&self) -> f32 {
        match self {
            BulletType::Simple => 5.0,
            BulletType::Shrapnel => 5.0,
        }
    }

    #[inline]
    pub fn speed(&self) -> f32 {
        match self {
            BulletType::Simple => 200.0,
            BulletType::Shrapnel => 125.0,
        }
    }

    #[inline]
    pub fn radius(&self) -> Option<f32> {
        match self {
            BulletType::Simple => None,
            BulletType::Shrapnel => Some(3.0),
        }
    }

    #[inline]
    pub fn cooldown(&self) -> f32 {
        match self {
            BulletType::Simple => 0.05,
            BulletType::Shrapnel => 0.2,
        }
    }

    #[inline]
    pub fn damage(&self) -> f32 {
        match self {
            BulletType::Simple => 2.0,
            BulletType::Shrapnel => 20.0,
        }
    }
}

pub struct Bullet {
    data: Arc<Mutex<GameData>>,
    pub position: Vec2,
    pub velocity: Vec2,
    pub lifetime: f32,
    pub ty: BulletType,
    pub dead: bool,
}

impl Bullet {
    pub fn new(data: Arc<Mutex<GameData>>, position: Vec2, direction: f32, ty: BulletType) -> Self {
        Self {
            data,
            position,
            velocity: Vec2::new(direction.cos(), direction.sin()) * ty.speed(),
            lifetime: ty.lifetime(),
            ty,
            dead: false,
        }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn kill(&mut self) {
        self.velocity = Vec2::ZERO;
        self.dead = true;
    }

    pub fn is_alive(&self) -> bool {
        !self.dead
    }

    pub fn update(&mut self, terrain: &mut Terrain, gravity: f32) {
        let dt = get_frame_time();

        if self.lifetime < dt {
            self.kill();
        } else {
            self.lifetime -= dt;
        }

        self.velocity += Vec2::new(0.0, gravity * dt);
        self.position += self.velocity * dt;
        self.collide_with_terrain(terrain);
    }

    pub fn collide_with_terrain(&mut self, terrain: &mut Terrain) {
        let nearby_radius = self
            .data
            .lock()
            .unwrap()
            .config
            .physics
            .collisions
            .nearby_nodes_radius_bullet;
        let nearby_nodes = terrain.bvh.get_nearby_nodes(self.position, nearby_radius);

        for (_node, bounds) in &nearby_nodes {
            match self.ty.radius() {
                Some(r) => {
                    if bounds.intersects_circle(self.position, r) {
                        terrain.destruct(self.position.x as u32, self.position.y as u32, r as u32);
                        self.kill();
                    }
                }
                None => {
                    if bounds.contains_point(self.position()) {
                        self.kill();
                    }
                }
            };
        }
    }

    pub fn draw(&self) {
        match self.ty {
            BulletType::Simple => {
                draw_circle(self.position.x, self.position.y, 0.75, WHITE);
            }
            BulletType::Shrapnel => {
                draw_circle(self.position.x, self.position.y, 3.0, WHITE);
            }
        }
    }
}
