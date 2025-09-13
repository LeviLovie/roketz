use bevy_ecs::{
    resource::Resource,
    system::{Commands, Res, ResMut},
};
use crossbeam::channel::{unbounded, Receiver};
use rapier2d::prelude::*;

use crate::ecs::DT;

#[derive(Resource)]
pub struct RapierWorld {
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
    pub query_pipeline: Option<QueryPipeline>,
    pub event_handler: ChannelEventCollector,
    pub _collision_events: Receiver<CollisionEvent>,
    pub _contact_force_events: Receiver<ContactForceEvent>,
}

pub fn init_rapier(mut commands: Commands) {
    let (collision_send, collision_recv) = unbounded();
    let (contact_send, contact_recv) = unbounded();

    commands.insert_resource(RapierWorld {
        pipeline: PhysicsPipeline::new(),
        gravity: vector![0.0, -0.0],
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
        query_pipeline: Some(QueryPipeline::new()),
        event_handler: ChannelEventCollector::new(collision_send, contact_send),
        _collision_events: collision_recv,
        _contact_force_events: contact_recv,
    });
}

pub fn step_rapier(mut world: ResMut<RapierWorld>, dt: Res<DT>) {
    world.integration_params.dt = dt.0;

    let RapierWorld {
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
        query_pipeline,
        event_handler,
        _collision_events: _,
        _contact_force_events: _,
    } = &mut *world;

    let gravity = &*gravity;
    let physics_hooks = &*physics_hooks;
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
