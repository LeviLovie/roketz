use anyhow::Result;
use egui::{Align, CentralPanel, Layout, RichText, Ui};
use macroquad::prelude::*;
use std::{cell::RefCell, rc::Rc};
use tracing::error;

use crate::{
    game::{GameData, Scene},
    scenes::{BattleType, SCENE_BATTLE, SCENE_QUIT},
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuState {
    #[default]
    Main,

    Singleplayer,
    Multiplayer,
    Options,
    Credits,
}

pub const SCENE_MENU: &str = "Menu";
pub struct Menu {
    data: Rc<RefCell<GameData>>,
    state: MenuState,
    transfer: Option<String>,
}

impl Scene for Menu {
    fn name(&self) -> &str {
        SCENE_MENU
    }

    fn should_transfer(&self) -> Option<String> {
        self.transfer.clone()
    }

    fn create(data: Rc<RefCell<GameData>>) -> Result<Self> {
        Ok(Self {
            data,
            state: MenuState::default(),
            transfer: None,
        })
    }

    fn reload(&mut self) -> Result<()> {
        self.state = MenuState::Main;
        self.transfer = None;
        Ok(())
    }

    fn render(&mut self) {
        clear_background(DARKGRAY);
    }

    fn ui(&mut self, ctx: &egui::Context) -> Result<()> {
        match self.state {
            MenuState::Main => {
                self.show_main(ctx);
            }
            MenuState::Singleplayer => {
                self.show_singleplayer(ctx);
            }
            MenuState::Multiplayer => {
                self.show_multiplayer(ctx);
            }
            MenuState::Options => {
                self.show_options(ctx);
            }
            MenuState::Credits => {
                self.show_credits(ctx);
            }
        }
        Ok(())
    }
}

impl Menu {
    fn play_click_sound(&self) {
        match self.data.borrow_mut().sound_engine.lock() {
            Ok(sound_engine) => {
                if let Err(e) = sound_engine.play("event:/ui/click") {
                    error!("Error playing click sound: {}", e);
                }
            }
            Err(e) => {
                error!("Failed to lock sound engine: {}", e);
            }
        }
    }

    fn show_back_to_main(&mut self, ui: &mut Ui) {
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.add_space(screen_height() / 12.0);
            ui.horizontal(|ui| {
                if ui
                    .button(
                        RichText::new("Back")
                            .font(egui::FontId::new(24.0, egui::FontFamily::Proportional)),
                    )
                    .clicked()
                {
                    self.play_click_sound();
                    self.state = MenuState::Main;
                }
            });
            ui.add_space(screen_height() / 12.0);
        });
    }

    fn show_main(&mut self, ctx: &egui::Context) {
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
                if ui
                    .button(RichText::new("Singleplayer").size(24.0))
                    .clicked()
                {
                    self.play_click_sound();
                    self.data.borrow_mut().battle_settings.ty = BattleType::Single;
                    self.state = MenuState::Singleplayer;
                }
                if ui.button(RichText::new("Multiplayer").size(24.0)).clicked() {
                    self.play_click_sound();
                    self.data.borrow_mut().battle_settings.ty = BattleType::MultiLeftRight;
                    self.state = MenuState::Multiplayer;
                }
                if ui.button(RichText::new("Options").size(24.0)).clicked() {
                    self.play_click_sound();
                    self.state = MenuState::Options;
                }
                if ui.button(RichText::new("Credits").size(24.0)).clicked() {
                    self.play_click_sound();
                    self.state = MenuState::Credits;
                }
                if ui.button(RichText::new("Quit").size(24.0)).clicked() {
                    self.play_click_sound();
                    self.transfer = Some(SCENE_QUIT.to_string());
                }
            });
        });
    }

    fn show_singleplayer(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            self.show_back_to_main(ui);

            ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                ui.label(RichText::new("Singleplayer").size(32.0));
                ui.add_space(screen_height() / 12.0);

                if ui.button(RichText::new("Play").size(24.0)).clicked() {
                    self.play_click_sound();
                    self.data.borrow_mut().battle_settings.ty = BattleType::Single;
                    self.transfer = Some(SCENE_BATTLE.to_string());
                }
            });
        });
    }

    fn show_multiplayer(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            self.show_back_to_main(ui);

            ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                ui.label(RichText::new("Multiplayer").size(32.0));
                ui.add_space(screen_height() / 12.0);

                let battle_type = self.data.borrow().battle_settings.ty;
                if ui
                    .add_enabled(
                        battle_type == BattleType::MultiLeftRight,
                        egui::Button::new(RichText::new("Horizontal split").size(24.0)),
                    )
                    .clicked()
                {
                    self.play_click_sound();
                    self.data.borrow_mut().battle_settings.ty = BattleType::MultiTopBottom;
                }
                if ui
                    .add_enabled(
                        battle_type == BattleType::MultiTopBottom,
                        egui::Button::new(RichText::new("Vertical split").size(24.0)),
                    )
                    .clicked()
                {
                    self.play_click_sound();
                    self.data.borrow_mut().battle_settings.ty = BattleType::MultiLeftRight;
                }

                ui.add_space(screen_height() / 12.0);
                if ui.button(RichText::new("Play").size(24.0)).clicked() {
                    self.play_click_sound();
                    self.transfer = Some(SCENE_BATTLE.to_string());
                }
            });
        });
    }

    fn show_options(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            self.show_back_to_main(ui);

            ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                ui.label("Options will be implemented later.");
            });
        });
    }

    fn show_credits(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            self.show_back_to_main(ui);

            ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                ui.label("Credits will be implemented later.");
            });
        });
    }
}
