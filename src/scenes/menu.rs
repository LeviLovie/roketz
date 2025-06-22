use anyhow::Result;
use egui::{Align, CentralPanel, Context, Layout, RichText, Ui};
use macroquad::prelude::*;
use std::sync::{Arc, Mutex};

use crate::game::{GameData, Scene};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuState {
    #[default]
    Main,

    Options,
    Credits,
}

pub struct Menu {
    _data: Arc<Mutex<GameData>>,
    state: MenuState,
    transfer: Option<String>,
}

impl Scene for Menu {
    fn create(data: Arc<Mutex<GameData>>) -> Result<Self> {
        Ok(Self {
            _data: data.clone(),
            state: MenuState::Main,
            transfer: None,
        })
    }

    fn name(&self) -> &str {
        "Menu"
    }

    fn should_transfer(&self) -> Option<String> {
        self.transfer.clone()
    }

    fn render(&self) {
        clear_background(DARKGRAY);
    }

    fn ui(&mut self, ctx: &Context) {
        match self.state {
            MenuState::Main => {
                self.show_main(ctx);
            }
            MenuState::Options => {
                self.show_options(ctx);
            }
            MenuState::Credits => {
                self.show_credits(ctx);
            }
        }
    }
}

impl Menu {
    fn show_back_to_main(&mut self, ui: &mut Ui) {
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.add_space(screen_height() / 12.0);
            ui.horizontal(|ui| {
                let button = ui.button(
                    RichText::new("Back")
                        .font(egui::FontId::new(24.0, egui::FontFamily::Proportional)),
                );
                if button.clicked() {
                    self.state = MenuState::Main;
                }
            });
        });
    }

    fn show_main(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.add_space(screen_height() / 6.0);
                ui.label(
                    RichText::new("Roketz")
                        .font(egui::FontId::new(128.0, egui::FontFamily::Proportional)),
                );
            });

            let available_height = ui.available_height();
            ui.add_space(available_height * 0.3);

            ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                if ui.button(RichText::new("Play").size(24.0)).clicked() {
                    self.transfer = Some("Battle".to_string());
                }
                if ui.button(RichText::new("Options").size(24.0)).clicked() {
                    self.state = MenuState::Options;
                }
                if ui.button(RichText::new("Credits").size(24.0)).clicked() {
                    self.state = MenuState::Credits;
                }
                if ui.button(RichText::new("Quit").size(24.0)).clicked() {
                    self.transfer = Some("__quit".to_string());
                }
            });
        });
    }

    fn show_options(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            self.show_back_to_main(ui);

            ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                ui.label("Options will be implemented later.");
            });
        });
    }

    fn show_credits(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            self.show_back_to_main(ui);

            ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                ui.label("Credits will be implemented later.");
            });
        });
    }
}
