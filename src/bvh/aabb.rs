use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub min: Vec2,
    pub max: Vec2,
}

impl AABB {
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn center(&self) -> Vec2 {
        (self.min + self.max) * 0.5
    }

    pub fn intersects_bounds(&self, other: &AABB) -> bool {
        !(self.max.x < other.min.x
            || self.min.x > other.max.x
            || self.max.y < other.min.y
            || self.min.y > other.max.y)
    }

    pub fn intersects_circle(&self, center: Vec2, radius: f32) -> bool {
        let closest_x = center.x.clamp(self.min.x, self.max.x);
        let closest_y = center.y.clamp(self.min.y, self.max.y);
        let distance_x = center.x - closest_x;
        let distance_y = center.y - closest_y;
        (distance_x * distance_x + distance_y * distance_y) <= (radius * radius)
    }

    pub fn contains(&self, other: &AABB) -> bool {
        self.min.x <= other.min.x
            && self.max.x >= other.max.x
            && self.min.y <= other.min.y
            && self.max.y >= other.max.y
    }

    pub fn contains_point(&self, point: Vec2) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }

    pub fn contains_circle(&self, center: Vec2, radius: f32) -> bool {
        let corners = [
            self.min,
            vec2(self.min.x, self.max.y),
            vec2(self.max.x, self.min.y),
            self.max,
        ];
        let radius_sq = radius * radius;
        corners
            .iter()
            .all(|&corner| corner.distance_squared(center) <= radius_sq)
    }

    pub fn subdivide(&self) -> [AABB; 4] {
        let c = self.center();
        [
            AABB {
                min: self.min,
                max: c,
            },
            AABB {
                min: vec2(c.x, self.min.y),
                max: vec2(self.max.x, c.y),
            },
            AABB {
                min: vec2(self.min.x, c.y),
                max: vec2(c.x, self.max.y),
            },
            AABB {
                min: c,
                max: self.max,
            },
        ]
    }
}
