#![allow(clippy::unwrap_used)]

use criterion::{Criterion, criterion_group, criterion_main};
use macroquad::prelude::*;

use bvh::{AABB, BVH};

pub fn bench_aabb(c: &mut Criterion) {
    let mut group = c.benchmark_group("aabb");

    group.bench_function("intersects_bounds", |b| {
        let bounds1 = AABB {
            min: vec2(0.0, 0.0),
            max: vec2(10.0, 10.0),
        };
        let bounds2 = AABB {
            min: vec2(5.0, 5.0),
            max: vec2(15.0, 15.0),
        };
        b.iter(|| {
            bounds1.intersects_bounds(&bounds2);
        });
    });

    group.bench_function("intersects_circle", |b| {
        let bounds = AABB {
            min: vec2(0.0, 0.0),
            max: vec2(10.0, 10.0),
        };
        let center = vec2(5.0, 5.0);
        let radius = 3.0;
        b.iter(|| {
            bounds.intersects_circle(center, radius);
        });
    });

    group.bench_function("contains", |b| {
        let bounds1 = AABB {
            min: vec2(0.0, 0.0),
            max: vec2(10.0, 10.0),
        };
        let bounds2 = AABB {
            min: vec2(2.0, 2.0),
            max: vec2(8.0, 8.0),
        };
        b.iter(|| {
            bounds1.contains(&bounds2);
        });
    });

    group.bench_function("contains_point", |b| {
        let bounds = AABB {
            min: vec2(0.0, 0.0),
            max: vec2(10.0, 10.0),
        };
        let point = vec2(5.0, 5.0);
        b.iter(|| {
            bounds.contains_point(point);
        });
    });

    group.bench_function("contains_circle", |b| {
        let bounds = AABB {
            min: vec2(0.0, 0.0),
            max: vec2(10.0, 10.0),
        };
        let center = vec2(5.0, 5.0);
        let radius = 5.0;
        b.iter(|| {
            bounds.contains_circle(center, radius);
        });
    });

    group.bench_function("subdivide", |b| {
        let bounds = AABB {
            min: vec2(0.0, 0.0),
            max: vec2(10.0, 10.0),
        };
        b.iter(|| {
            bounds.subdivide();
        });
    });

    group.finish();
}

pub fn bench_bvh(c: &mut Criterion) {
    let mut group = c.benchmark_group("bvh");

    group.bench_function("cut_circle_shallow", |b| {
        b.iter(|| {
            let mut bvh = BVH::new(100, 100, 5);
            bvh.cut_circle(vec2(25.0, 40.0), 20.0).unwrap();
        });
    });

    group.bench_function("cut_circle_deep", |b| {
        b.iter(|| {
            let mut bvh = BVH::new(100, 100, 12);
            bvh.cut_circle(vec2(25.0, 40.0), 20.0).unwrap();
        });
    });

    group.bench_function("cut_point", |b| {
        b.iter(|| {
            let mut bvh = BVH::new(100, 100, 10);
            bvh.cut_point(vec2(25.0, 40.0));
        });
    });

    group.finish();
}

criterion_group!(benches, bench_bvh, bench_aabb);
criterion_main!(benches);
