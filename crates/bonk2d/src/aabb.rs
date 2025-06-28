//! Axis-Aligned Bounding Box (AABB) implementation.
//! See [this](https://en.wikipedia.org/wiki/Bounding_volume#Common_types) for details.

use anyhow::{anyhow, Context, Result};
use macroquad::prelude::*;

/// Axis-Aligned Bounding Box (AABB) structure
/// This structure represents an axis-aligned bounding box defined by its minimum and maximum corners.
///
/// # Example
/// ```rust
/// use macroquad::prelude::Vec2;
/// use bonk2d::AABB;
/// use anyhow::Result;
///
/// fn main() -> Result<()> {
///     let aabb = AABB::from_points(&[Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0)])?;
///     let another_aabb = AABB::from_center_size(Vec2::new(0.0, 0.0), Vec2::new(4.0, 6.0))?;
///     let intersects = aabb.intersects(&another_aabb);
///     Ok(())
/// }
/// ```
pub struct AABB {
    min: Vec2,
    max: Vec2,
}

impl AABB {
    /// Creates a new AABB from a list of points.
    /// Points list can not be empty.
    ///
    /// # Example
    /// ```rust
    /// use macroquad::prelude::Vec2;
    /// use anyhow::Result;
    /// use bonk2d::AABB;
    ///
    /// fn main() -> Result<()> {
    ///     let points = vec![
    ///        Vec2::new(1.0, 2.0),
    ///        Vec2::new(3.0, 4.0),
    ///        Vec2::new(-1.0, -2.0),
    ///        Vec2::new(-3.0, -4.0),
    ///     ];
    ///     let aabb = AABB::from_points(&points)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn from_points(points: &[Vec2]) -> Result<Self> {
        if points.len() == 0 {
            return Err(anyhow!("AABB::from_points requires at least one point"))
                .context("While creating AABB from points");
        }

        let mut min = Vec2::new(f32::INFINITY, f32::INFINITY);
        let mut max = Vec2::new(f32::NEG_INFINITY, f32::NEG_INFINITY);
        for &point in points {
            min.x = min.x.min(point.x);
            min.y = min.y.min(point.y);
            max.x = max.x.max(point.x);
            max.y = max.y.max(point.y);
        }
        Ok(AABB { min, max })
    }

    /// Creates a new AABB from a center point and size.
    /// The size must be non-negative and non-zero.
    ///
    /// # Example
    /// ```rust
    /// use macroquad::prelude::Vec2;
    /// use anyhow::Result;
    /// use bonk2d::AABB;
    ///
    /// fn main() -> Result<()> {
    ///    let center = Vec2::new(1.0, -2.0);
    ///    let size = Vec2::new(4.0, 6.0);
    ///    let aabb = AABB::from_center_size(center, size)?;
    ///    Ok(())
    /// }
    /// ```
    pub fn from_center_size(center: Vec2, size: Vec2) -> Result<Self> {
        if size.x < 0.0 || size.y < 0.0 {
            return Err(anyhow!("AABB::from_center_size requires non-negative size"))
                .context("While creating AABB from center and size");
        }
        if size.x == 0.0 && size.y == 0.0 {
            return Err(anyhow!("AABB::from_center_size requires non-zero size"))
                .context("While creating AABB from center and size");
        }

        let half_size = size * 0.5;
        Ok(AABB {
            min: center - half_size,
            max: center + half_size,
        })
    }

    /// Returns the height of the AABB as a f32 number.
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    /// Returns the width of the AABB as a f32 number.
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    /// Returns the center of the AABB as a Vec2.
    pub fn center(&self) -> Vec2 {
        (self.min + self.max) * 0.5
    }

    /// Translates the AABB by a given vector.
    pub fn translate(&self, translation: Vec2) -> Self {
        AABB {
            min: self.min + translation,
            max: self.max + translation,
        }
    }

    /// Checks if this AABB intersects with another AABB.
    ///
    /// # Example
    /// ```rust
    /// use macroquad::prelude::Vec2;
    /// use anyhow::Result;
    /// use bonk2d::AABB;
    ///
    /// fn main() -> Result<()> {
    ///    let aabb1 = AABB::from_points(&[Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0)])?;
    ///    let aabb2 = AABB::from_points(&[Vec2::new(1.0, 1.0), Vec2::new(3.0, 3.0)])?;
    ///    let intersects = aabb1.intersects(&aabb2);
    ///    Ok(())
    /// }
    /// ```
    pub fn intersects(&self, other: &AABB) -> bool {
        !(self.max.x < other.min.x
            || self.min.x > other.max.x
            || self.max.y < other.min.y
            || self.min.y > other.max.y)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aabb_from_points_ok() -> Result<()> {
        let points = vec![
            Vec2::new(1.0, 2.0),
            Vec2::new(3.0, 4.0),
            Vec2::new(-1.0, -2.0),
            Vec2::new(-3.0, -4.0),
            Vec2::new(0.0, 0.0),
        ];
        let aabb = AABB::from_points(&points);
        assert!(aabb.is_ok());
        let aabb = aabb?;
        assert_eq!(aabb.min, Vec2::new(-3.0, -4.0));
        assert_eq!(aabb.max, Vec2::new(3.0, 4.0));
        Ok(())
    }

    #[test]
    fn aabb_from_points_err_empty() {
        let points = vec![];
        let aabb = AABB::from_points(&points);
        assert!(aabb.is_err());
    }

    #[test]
    fn aabb_from_center_size_ok() {
        let center = Vec2::new(0.0, 0.0);
        let size = Vec2::new(4.0, 6.0);
        let aabb = AABB::from_center_size(center, size);
        assert!(aabb.is_ok());
        let aabb = aabb.unwrap();
        assert_eq!(aabb.min, Vec2::new(-2.0, -3.0));
        assert_eq!(aabb.max, Vec2::new(2.0, 3.0));
    }

    #[test]
    fn aabb_from_center_size_err_negative_size() {
        let center = Vec2::new(0.0, 0.0);
        let size = Vec2::new(-4.0, 6.0);
        let aabb = AABB::from_center_size(center, size);
        assert!(aabb.is_err());
    }

    #[test]
    fn aabb_from_center_size_err_zero_size() {
        let center = Vec2::new(0.0, 0.0);
        let size = Vec2::new(0.0, 0.0);
        let aabb = AABB::from_center_size(center, size);
        assert!(aabb.is_err());
    }

    #[test]
    fn aabb_width() {
        let aabb = AABB {
            min: Vec2::new(1.0, 2.0),
            max: Vec2::new(5.0, 6.0),
        };
        assert_eq!(aabb.width(), 4.0);

        let aabb = AABB {
            min: Vec2::new(-3.0, -4.0),
            max: Vec2::new(3.0, 4.0),
        };
        assert_eq!(aabb.width(), 6.0);

        let aabb = AABB {
            min: Vec2::new(0.0, 0.0),
            max: Vec2::new(0.0, 0.0),
        };
        assert_eq!(aabb.width(), 0.0);
    }

    #[test]
    fn aabb_height() {
        let aabb = AABB {
            min: Vec2::new(1.0, 2.0),
            max: Vec2::new(5.0, 6.0),
        };
        assert_eq!(aabb.height(), 4.0);

        let aabb = AABB {
            min: Vec2::new(-3.0, -4.0),
            max: Vec2::new(3.0, 4.0),
        };
        assert_eq!(aabb.height(), 8.0);

        let aabb = AABB {
            min: Vec2::new(0.0, 0.0),
            max: Vec2::new(0.0, 0.0),
        };
        assert_eq!(aabb.height(), 0.0);
    }

    #[test]
    fn aabb_center() {
        let aabb = AABB {
            min: Vec2::new(0.0, 0.0),
            max: Vec2::new(2.0, 2.0),
        };
        assert_eq!(aabb.center(), Vec2::new(1.0, 1.0));

        let aabb = AABB {
            min: Vec2::new(-1.0, -1.0),
            max: Vec2::new(1.0, 1.0),
        };
        assert_eq!(aabb.center(), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn aabb_translate() {
        let aabb = AABB {
            min: Vec2::new(0.0, 0.0),
            max: Vec2::new(2.0, 2.0),
        };

        let translated = aabb.translate(Vec2::new(1.0, 1.0));
        assert_eq!(translated.min, Vec2::new(1.0, 1.0));
        assert_eq!(translated.max, Vec2::new(3.0, 3.0));

        let translated = aabb.translate(Vec2::new(-1.0, -1.0));
        assert_eq!(translated.min, Vec2::new(-1.0, -1.0));
        assert_eq!(translated.max, Vec2::new(1.0, 1.0));
    }

    #[test]
    fn aabb_intersects() {
        let aabb = AABB {
            min: Vec2::new(0.0, 0.0),
            max: Vec2::new(2.0, 2.0),
        };
        let intersects = AABB {
            min: Vec2::new(1.0, 1.0),
            max: Vec2::new(3.0, 3.0),
        };
        assert!(aabb.intersects(&intersects));

        let not_intersects = AABB {
            min: Vec2::new(3.0, 3.0),
            max: Vec2::new(4.0, 4.0),
        };
        assert!(!aabb.intersects(&not_intersects));
    }
}
