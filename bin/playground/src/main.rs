mod resources;
mod systems;

use anyhow::{Context, Result};
use bevy_ecs::prelude::*;
use egui::{SidePanel, TopBottomPanel, menu};
use macroquad::prelude::*;
use tracing::info;

use bonk2d::{Circle, Transform};
use resources::RenderConfig;
use systems::render_circles;

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

    let mut zoom = 0.005;
    let mut last_mouse_position: Vec2 = mouse_position().into();
    let mut camera = Camera2D::default();

    let mut world = World::new();
    let mut update = Schedule::default();
    let mut render = Schedule::default();

    world.insert_resource(RenderConfig::default());

    render.add_systems(render_circles);

    info!("Entering main loop");
    loop {
        {
            zoom *= 1.0 + mouse_wheel().0 * 0.1;
            zoom = zoom.clamp(0.001, 0.1);

            let mouse_position: Vec2 = mouse_position().into();
            if is_mouse_button_down(MouseButton::Left) {
                let delta = mouse_position - last_mouse_position;
                camera.target -= delta * zoom * 100.0;
                println!("Camera target: {:?}", camera.target);
            }
            last_mouse_position = mouse_position;

            camera.zoom = vec2(zoom, zoom * screen_width() / screen_height());
        };

        {
            update.run(&mut world);
            bonk2d::process(&mut world).context("Failed to process world")?;
        };

        {
            clear_background(DARKGRAY);
            set_camera(&camera);
            render.run(&mut world);
            set_default_camera();
        };

        {
            egui_macroquad::ui(|ctx| {
                SidePanel::right("Inspect").show(ctx, |ui| {
                    ui.heading("Spawn");
                    if ui.button("Circle").clicked() {
                        world.spawn((Transform::default(), Circle::new(10.0)));
                    }
                });

                TopBottomPanel::top("Menu").show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        menu::bar(ui, |ui| {
                            let mut config = world.resource_mut::<RenderConfig>();
                            ui.checkbox(&mut config.filled_in, "Filled in");
                        });
                    });
                });
            });
            egui_macroquad::draw();
        };

        next_frame().await
    }
}
