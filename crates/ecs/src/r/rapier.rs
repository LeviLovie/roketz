use bevy_ecs::prelude::*;
use rapier2d::prelude::*;

#[derive(Resource)]
pub struct PhysicsWorld {
    pub pipeline: PhysicsPipeline,
    pub gravity: Vector<f32>,
    pub integration_params: IntegrationParameters,
    pub island_manager: IslandManager,
    pub narrow_phase: NarrowPhase,
    pub bodies: RigidBodySet,
    pub colliders: ColliderSet,
    pub ccd_solver: CCDSolver,
    pub physics_hooks: (),
    pub event_handler: (),
}

pub fn init_physics(world: &mut World) {
    world.insert_resource(PhysicsWorld {
        pipeline: PhysicsPipeline::new(),
        gravity: vector![0.0, -9.81],
        integration_params: IntegrationParameters::default(),
        island_manager: IslandManager::new(),
        narrow_phase: NarrowPhase::new(),
        bodies: RigidBodySet::new(),
        colliders: ColliderSet::new(),
        ccd_solver: CCDSolver::new(),
        physics_hooks: (),
        event_handler: (),
    });
}
