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

    pub fn push_circle_out(&self, center: &mut Vec2, radius: f32) -> bool {
        let closest = vec2(
            center.x.clamp(self.min.x, self.max.x),
            center.y.clamp(self.min.y, self.max.y),
        );

        let delta = *center - closest;
        let dist_sq = delta.length_squared();

        if dist_sq < radius * radius {
            let dist = dist_sq.sqrt().max(0.0001);
            let push = delta / dist * (radius - dist);
            *center += push;
            return true;
        }

        false
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
        let left = center.x - radius;
        let right = center.x + radius;
        let top = center.y + radius;
        let bottom = center.y - radius;
        left >= self.min.x && right <= self.max.x && top <= self.max.y && bottom >= self.min.y
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn center() {
        let aabb = AABB {
            min: vec2(0.0, 0.0),
            max: vec2(2.0, 2.0),
        };
        assert_eq!(aabb.center(), vec2(1.0, 1.0));
    }

    #[test]
    fn intersects_bounds() {
        let bounds = AABB {
            min: vec2(0.0, 0.0),
            max: vec2(2.0, 2.0),
        };
        let intersects = AABB {
            min: vec2(1.0, 1.0),
            max: vec2(3.0, 3.0),
        };
        assert!(bounds.intersects_bounds(&intersects));
        let not_intersects = AABB {
            min: vec2(3.0, 3.0),
            max: vec2(4.0, 4.0),
        };
        assert!(!bounds.intersects_bounds(&not_intersects));
    }

    #[test]
    fn intersects_circle() {
        let aabb = AABB {
            min: vec2(0.0, 0.0),
            max: vec2(2.0, 2.0),
        };
        assert!(aabb.intersects_circle(vec2(1.0, 1.0), 1.0));
        assert!(!aabb.intersects_circle(vec2(3.0, 3.0), 1.0));
    }

    #[test]
    fn contains() {
        let aabb = AABB {
            min: vec2(0.0, 0.0),
            max: vec2(2.0, 2.0),
        };
        let contained = AABB {
            min: vec2(0.5, 0.5),
            max: vec2(1.5, 1.5),
        };
        assert!(aabb.contains(&contained));
        let not_contained = AABB {
            min: vec2(-1.0, -1.0),
            max: vec2(3.0, 3.0),
        };
        assert!(!aabb.contains(&not_contained));
    }

    #[test]
    fn contains_point() {
        let aabb = AABB {
            min: vec2(0.0, 0.0),
            max: vec2(2.0, 2.0),
        };
        assert!(aabb.contains_point(vec2(1.0, 1.0)));
        assert!(!aabb.contains_point(vec2(3.0, 3.0)));
    }

    #[test]
    fn contains_circle() {
        let aabb = AABB {
            min: vec2(0.0, 0.0),
            max: vec2(2.0, 2.0),
        };
        assert!(aabb.contains_circle(vec2(1.0, 1.0), 0.3));
        assert!(!aabb.contains_circle(vec2(3.0, 3.0), 0.5));
    }

    #[test]
    fn subdivide() {
        let aabb = AABB {
            min: vec2(0.0, 0.0),
            max: vec2(4.0, 4.0),
        };
        let sub_aabbs = aabb.subdivide();
        assert_eq!(sub_aabbs.len(), 4);
        assert_eq!(sub_aabbs[0].min, vec2(0.0, 0.0));
        assert_eq!(sub_aabbs[0].max, vec2(2.0, 2.0));
        assert_eq!(sub_aabbs[1].min, vec2(2.0, 0.0));
        assert_eq!(sub_aabbs[1].max, vec2(4.0, 2.0));
        assert_eq!(sub_aabbs[2].min, vec2(0.0, 2.0));
        assert_eq!(sub_aabbs[2].max, vec2(2.0, 4.0));
        assert_eq!(sub_aabbs[3].min, vec2(2.0, 2.0));
        assert_eq!(sub_aabbs[3].max, vec2(4.0, 4.0));
    }
}
