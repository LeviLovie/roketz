use macroquad::prelude::*;
use std::sync::{Arc, Mutex};

use super::Terrain;
use crate::game::GameData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BulletType {
    Simple,
    Shrapnel,
}

impl ToString for BulletType {
    fn to_string(&self) -> String {
        match self {
            BulletType::Simple => "Simple",
            BulletType::Shrapnel => "Shrapnel",
        }
        .to_string()
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
            BulletType::Simple => 1.0,
            BulletType::Shrapnel => 2.0,
        }
    }

    #[inline]
    pub fn speed(&self) -> f32 {
        match self {
            BulletType::Simple => 200.0,
            BulletType::Shrapnel => 200.0,
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
            BulletType::Simple => 0.2,
            BulletType::Shrapnel => 0.5,
        }
    }

    #[inline]
    pub fn damage(&self) -> f32 {
        match self {
            BulletType::Simple => 5.0,
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
    pub is_player_2: bool,
}

impl Bullet {
    pub fn new(
        data: Arc<Mutex<GameData>>,
        position: Vec2,
        direction: f32,
        ty: BulletType,
        is_player_2: bool,
    ) -> Self {
        Self {
            data,
            position,
            velocity: Vec2::new(direction.cos(), direction.sin()) * ty.speed(),
            lifetime: ty.lifetime(),
            ty,
            dead: false,
            is_player_2,
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

    pub fn update(&mut self, terrain: &mut Terrain) {
        let dt = get_frame_time();

        if self.lifetime < dt {
            self.kill();
        } else {
            self.lifetime -= dt;
        }

        match self.ty {
            _ => {}
        }

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
        let color = if self.is_player_2 {
            Color::from_rgba(245, 122, 56, 255)
        } else {
            Color::from_rgba(66, 245, 230, 255)
        };

        match self.ty {
            BulletType::Simple => {
                draw_circle(self.position.x, self.position.y, 0.75, color);
            }
            BulletType::Shrapnel => {
                draw_circle(self.position.x, self.position.y, 3.0, color);
            }
        }
    }
}
