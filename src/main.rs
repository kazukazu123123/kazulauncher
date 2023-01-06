#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{
    egui::{self, menu, FontData, FontDefinitions, Id, ScrollArea},
    epaint::FontFamily,
};
use egui_notify::Toasts;
use native_dialog::{MessageDialog, MessageType};
use rust_i18n::{set_locale, t};
use std::{process, time::Duration};

mod launcher;

rust_i18n::i18n!("locales");

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some([640.0, 360.0].into()),
        ..Default::default()
    };

    eframe::run_native(
        "KazuLauncher",
        native_options,
        Box::new(|cc| Box::new(KazuLauncher::new(cc))),
    )
}

#[derive(Default)]
struct KazuLauncher {
    toasts: Toasts,
    setting_window: bool,
    debug_draw_enabled: bool,
    app_button_size: f32,
    search: String,
}

impl KazuLauncher {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = FontDefinitions::default();

        const FONT_NAME: &str = "noto-sans";

        fonts.font_data.insert(
            FONT_NAME.to_owned(),
            FontData::from_static(include_bytes!("../NotoSansJP-Medium.otf")),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, FONT_NAME.to_owned());
        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .insert(0, FONT_NAME.to_owned());
        cc.egui_ctx.set_fonts(fonts);

        Self::default()
    }
}

impl eframe::App for KazuLauncher {
    fn on_close_event(&mut self) -> bool {
        MessageDialog::new()
            .set_type(MessageType::Info)
            .set_title("Confirm")
            .set_text("Do you want to close?")
            .show_confirm()
            .unwrap()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.0);
        ctx.set_debug_on_hover(self.debug_draw_enabled);

        self.app_button_size = 64.0;

        if self.setting_window {
            egui::Window::new(t!("menu.file.setting.title"))
                .collapsible(true)
                .id(Id::new("setting_window"))
                .open(&mut self.setting_window)
                .vscroll(false)
                .resizable(true)
                .min_width(240.0)
                .min_height(160.0)
                .default_size((240.0, 160.0))
                .show(ctx, |ui| {
                    egui::TopBottomPanel::bottom("settings_bottom_panel")
                        .resizable(false)
                        .min_height(20.0)
                        .show_inside(ui, |ui| {
                            ui.horizontal(|ui| {
                                if ui.button(t!("menu.file.setting.save")).clicked() {};

                                if ui.button(t!("menu.file.setting.discard")).clicked() {};
                            })
                        });

                    egui::CentralPanel::default()
                        .frame(egui::Frame::none())
                        .show_inside(ui, |ui| {
                            ScrollArea::both().show(ui, |ui| {
                                ui.label(egui::RichText::new("ZA".repeat(200)).weak());
                                ui.separator();
                                ui.label(egui::RichText::new("PA".repeat(200)).weak());
                                ui.allocate_space(ui.available_size())
                            });
                        });
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button(t!("menu.file.title"), |ui| {
                    if ui.button(t!("menu.file.setting.title")).clicked() {
                        ui.close_menu();
                        self.setting_window = true;
                    }
                    if ui.button(t!("menu.file.quit")).clicked() {
                        process::exit(0);
                    }
                });
                ui.menu_button(t!("menu.launcher.title"), |ui| {
                    if ui.button("Add new item").clicked() {
                        ui.close_menu();
                    }
                });
                if cfg!(debug_assertions) {
                    ui.menu_button("Debug", |ui| {
                        if ui
                            .checkbox(&mut self.debug_draw_enabled, "DEBUG_DRAW_ENABLED")
                            .clicked()
                        {
                            ui.close_menu();
                        }
                    });
                }
            });

            egui::TopBottomPanel::top("launcher_category")
                .resizable(false)
                .show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        ScrollArea::horizontal().show(ui, |ui| {
                            for _ in 0..10 {
                                if ui.button("Launch notepad").clicked() {
                                    if let Err(error) = launcher::launch("notepad") {
                                        self.toasts
                                            .error(format!(
                                                "Failed to launch process.\nReason: {}",
                                                error
                                            ))
                                            .set_duration(Some(Duration::from_secs(2)));
                                    }
                                }
                            }
                        });
                    });
                    ui.horizontal(|ui| {
                        ui.label("Search:");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.search)
                                .desired_width(ui.available_size().x),
                        );
                    });
                });

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.spacing_mut().item_spacing = egui::Vec2::splat(4.0);
                ui.horizontal_wrapped(|ui| {
                    for i in 0..32 {
                        let item = egui::Button::new("Item here");
                        if ui.add_sized([self.app_button_size, self.app_button_size], item)
                            .on_hover_text_at_pointer(format!("Some hover text: {}", i))
                            .context_menu(|ui| {
                                if ui.button("Click me!").clicked() {
                                    println!("You right-clicked menu item: {}", i);
                                    ui.close_menu();
                                }
                            })
                            .clicked() {
                                println!("Clicked app: {}", i);
                            };
                    }
                });
                ui.allocate_space(ui.available_size())
            });
        });
        self.toasts.show(ctx);
    }
}
