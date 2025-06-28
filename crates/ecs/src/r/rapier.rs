use bevy_ecs::prelude::*;
use rapier2d::prelude::*;

use crate::r::DT;

#[derive(Resource)]
pub struct PhysicsWorld {
    pub pipeline: PhysicsPipeline,
    pub gravity: Vector<f32>,
    pub integration_params: IntegrationParameters,
    pub island_manager: IslandManager,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,
    pub bodies: RigidBodySet,
    pub colliders: ColliderSet,
    pub impulse_joints: ImpulseJointSet,
    pub multibody_joints: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub physics_hooks: (),
    pub event_handler: (),
    pub query_pipeline: Option<QueryPipeline>,
}

pub fn init_physics(world: &mut World) {
    world.insert_resource(PhysicsWorld {
        pipeline: PhysicsPipeline::new(),
        gravity: vector![0.0, 30.0],
        integration_params: IntegrationParameters::default(),
        island_manager: IslandManager::new(),
        broad_phase: DefaultBroadPhase::new(),
        narrow_phase: NarrowPhase::new(),
        bodies: RigidBodySet::new(),
        colliders: ColliderSet::new(),
        impulse_joints: ImpulseJointSet::new(),
        multibody_joints: MultibodyJointSet::new(),
        ccd_solver: CCDSolver::new(),
        physics_hooks: (),
        event_handler: (),
        query_pipeline: Some(QueryPipeline::new()),
    });
}

pub fn step_physics(mut world: ResMut<PhysicsWorld>, dt: Res<DT>) {
    world.integration_params.dt = dt.0;

    let PhysicsWorld {
        pipeline,
        gravity,
        integration_params,
        island_manager,
        broad_phase,
        narrow_phase,
        bodies,
        colliders,
        impulse_joints,
        multibody_joints,
        ccd_solver,
        physics_hooks,
        event_handler,
        query_pipeline,
    } = &mut *world;

    let gravity = &*gravity;
    let physics_hooks = &*physics_hooks;
    let event_handler = &*event_handler;
    let query_pipeline = query_pipeline.as_mut();

    pipeline.step(
        gravity,
        integration_params,
        island_manager,
        broad_phase,
        narrow_phase,
        bodies,
        colliders,
        impulse_joints,
        multibody_joints,
        ccd_solver,
        query_pipeline,
        physics_hooks,
        event_handler,
    );
}
