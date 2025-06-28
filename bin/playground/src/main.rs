mod resources;
mod systems;

use anyhow::{Context, Result};
use bevy_ecs::prelude::*;
use egui::{SidePanel, TopBottomPanel, menu};
use macroquad::prelude::*;
use tracing::info;

use bonk2d::{Circle, Transform};
use systems::render_circles;

use crate::{
    resources::Selection,
    systems::{process_physics, reset_selection, update_circles},
};

#[macroquad::main("Playground")]
async fn main() {
    if let Err(err) = try_main().await {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}

async fn try_main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .init();
    info!(version = ?env!("CARGO_PKG_VERSION"), "Launching playground");

    let mut world = World::new();
    let mut update = Schedule::default();
    let mut render = Schedule::default();

    world.insert_resource(Selection::default());

    update.add_systems((reset_selection, (update_circles,), process_physics).chain());

    render.add_systems(render_circles);

    info!("Entering main loop");
    loop {
        {
            update.run(&mut world);

            {
                let selection = world.resource_mut::<Selection>();
                if selection.entity.is_some() {
                    let entity = selection.entity.unwrap();
                    let mut transform_component = world.get_mut::<Transform>(entity)
                        .context("Failed to get Transform component")?;
                    let mouse_pos: Vec2 = mouse_position().into();
                    let delta = mouse_pos - *transform_component.pos();
                    transform_component.set_vel(delta);
                }
            };
        };

        {
            clear_background(DARKGRAY);
            render.run(&mut world);
        };

        {
            egui_macroquad::ui(|ctx| {
                SidePanel::right("Inspect").show(ctx, |ui| {
                    ui.heading("Spawn");
                });

                TopBottomPanel::top("Menu").show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        menu::bar(ui, |ui| {
                            ui.menu_button("Selection", |ui| {
                                let mut selection = world.resource_mut::<Selection>();
                                if selection.entity.is_some() {
                                    if ui.button("Reset").clicked() {
                                        selection.entity = None;
                                        selection.changed = true;
                                    }
                                } else {
                                    ui.label("No selection");
                                }
                            });

                            ui.menu_button("Spawn", |ui| {
                                if ui.button("Circle").clicked() {
                                    let mut transform = Transform::default();
                                    transform.set_pos(vec2(screen_width() / 2.0, screen_height() / 2.0));
                                    world.spawn((transform, Circle::new(25.0)));
                                }
                            });
                        });
                    });
                });
            });
            egui_macroquad::draw();
        };

        next_frame().await
    }
}
