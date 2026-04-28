#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use anyhow::{Ok as AnyOk, Result, bail};
use eframe::egui::{self, ViewportCommand};
use image::RgbaImage;
use sheetsmithlib::algorithms::{self, Algorithm};
use std::time::Instant;

struct SheetSmithApp {
    input_path: String,
    output_path: String,
    size: String,
    padding: i32,
    trim_transparent: bool,
    auto_size: bool,
    alg: algorithms::Algorithm,
    status: String,
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("SheetSmith GUI")
            .with_inner_size(egui::vec2(400.0, 400.0))
            .with_resizable(true)
            .with_decorations(false)
            .with_transparent(true),
        ..Default::default()
    };

    eframe::run_native(
        "SheetSmith GUI",
        options,
        Box::new(|_cc| {
            Ok(Box::new(SheetSmithApp {
                input_path: "../example".into(),
                output_path: "../output.png".into(),
                size: "896x256".into(),
                padding: 0,
                trim_transparent: false,
                auto_size: false,
                alg: algorithms::Algorithm::Guillotiere,
                status: "Waiting...".into(),
            }))
        }),
    )
}

impl eframe::App for SheetSmithApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ui.ctx());
        custom_window_frame(ui, "SheetSmith", |ui| {
            let ui_builder = egui::UiBuilder::new();

            ui.vertical_centered(|ui| {
                ui.add_space(5.0);
                ui.add(
                    egui::Image::new(egui::include_image!("../../example-logo.png"))
                        .max_height(128.0),
                );
                ui.add_space(25.0);

                ui.scope_builder(ui_builder, |ui| {
                    ui.multiply_opacity(1.0);

                    egui::Grid::new("param_grid")
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            self.param_grid(ui);
                        });

                    ui.add_space(20.0);

                    ui.vertical_centered(|ui| {
                        if ui.button("Pack!").clicked() {
                            let start = Instant::now();
                            if let Err(e) = self.handle_pack() {
                                eprintln!("Error occurred: {}", e);
                                self.status = format!(
                                    "[{}ms] {}",
                                    start.elapsed().as_millis(),
                                    e.to_string()
                                );
                            } else {
                                self.status = format!(
                                    "[{}ms] Packing completed successfully at {}.",
                                    start.elapsed().as_millis(),
                                    self.output_path
                                );
                            }
                        }
                    });

                    ui.add_space(20.0);

                    code_block(ui, self.status.trim_start_matches('\n'));
                });
            });
        });

        ui.ctx().send_viewport_cmd(ViewportCommand::InnerSize(ui.ctx().globally_used_rect().size()));
    }
}

impl SheetSmithApp {
    fn param_grid(&mut self, ui: &mut egui::Ui) {
        let Self {
            input_path,
            output_path,
            size,
            padding,
            trim_transparent,
            auto_size,
            alg,
            status,
        } = self;

        ui.label("Input:");
        ui.text_edit_singleline(input_path);
        ui.end_row();

        ui.label("Output:");
        ui.text_edit_singleline(output_path);
        ui.end_row();

        ui.label("Max Size:");
        ui.text_edit_singleline(size);
        ui.end_row();

        ui.label("Padding:");
        ui.add(egui::Slider::new(padding, 0..=100));
        ui.end_row();

        ui.label("Trim Transparent");
        ui.checkbox(trim_transparent, "Trim Transparent");
        ui.end_row();

        ui.label("Auto Size");
        ui.checkbox(auto_size, "Auto Size");
        ui.end_row();

        ui.label("Algorithm:");
        egui::ComboBox::new("algorithm", "")
            .selected_text(format!("{alg:?}"))
            .show_ui(ui, |ui| {
                if ui.button("Guillotiere").clicked() {
                    *alg = algorithms::Algorithm::Guillotiere;
                }
            });

        ui.end_row();
    }

    fn handle_pack(&mut self) -> Result<()> {
        let (mut image_files, image_count) =
            sheetsmithlib::walk_input_directory(&self.input_path, false)?;

        if image_count == 0 {
            bail!(
                "No image files found in input directory '{}'.",
                self.input_path
            );
        }

        if self.trim_transparent {
            for (_, image) in image_files.iter_mut() {
                *image = sheetsmithlib::trim_image(image, false)?;
            }
        }

        let mut size;

        if self.auto_size {
            size = sheetsmithlib::find_optimal_size(image_files.clone(), self.padding as u32)?;
        } else {
            size = sheetsmithlib::parse_size_arg(&self.size)?
        }

        // Pack images using selected alg
        let mut output_image = RgbaImage::new(0, 0);
        if self.alg == algorithms::Algorithm::Guillotiere {
            output_image = sheetsmithlib::pack_images_guillotiere(
                algorithms::guillotiere_alg::GuillotiereArgs {
                    size,
                    padding: self.padding,
                    image_files,
                },
            )?;
        }

        // Save output image
        output_image.save(&self.output_path)?;

        AnyOk(())
    }
}

fn custom_window_frame(ui: &mut egui::Ui, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
    let panel_frame = egui::Frame::new()
        .fill(ui.global_style().visuals.window_fill())
        .corner_radius(10)
        .stroke(ui.global_style().visuals.widgets.noninteractive.fg_stroke)
        .outer_margin(1);

    panel_frame.show(ui, |ui| {
        let title_bar_height = 32.0;
        let (title_bar_rect, _) = ui.allocate_exact_size(
            egui::vec2(ui.available_width(), title_bar_height),
            egui::Sense::hover(),
        );
        title_bar_ui(ui, title_bar_rect, title);

        egui::Frame::new().inner_margin(4.0).show(ui, |ui| {
            add_contents(ui);
        });
    });
}

fn title_bar_ui(ui: &mut egui::Ui, title_bar_rect: eframe::epaint::Rect, title: &str) {
    use egui::{Align2, FontId, Id, PointerButton, Sense, UiBuilder, vec2};

    let painter = ui.painter();

    let title_bar_response = ui.interact(
        title_bar_rect,
        Id::new("title_bar"),
        Sense::click_and_drag(),
    );

    // Paint the title:
    painter.text(
        title_bar_rect.center(),
        Align2::CENTER_CENTER,
        title,
        FontId::proportional(20.0),
        ui.style().visuals.text_color(),
    );

    // Paint the line under the title:
    painter.line_segment(
        [
            title_bar_rect.left_bottom() + vec2(1.0, 0.0),
            title_bar_rect.right_bottom() + vec2(-1.0, 0.0),
        ],
        ui.visuals().widgets.noninteractive.bg_stroke,
    );

    // Interact with the title bar (drag to move window):
    if title_bar_response.double_clicked() {
        let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
        ui.send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
    }

    if title_bar_response.drag_started_by(PointerButton::Primary) {
        ui.send_viewport_cmd(ViewportCommand::StartDrag);
    }

    ui.scope_builder(
        UiBuilder::new()
            .max_rect(title_bar_rect)
            .layout(egui::Layout::right_to_left(egui::Align::Center)),
        |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            close_maximize_minimize(ui);
        },
    );
}

/// Show some close button for the native window.
fn close_maximize_minimize(ui: &mut egui::Ui) {
    use egui::{Button, RichText};

    let button_height = 24.0;

    let close_response = ui
        .add(Button::new(RichText::new("❌").size(button_height)))
        .on_hover_text("Close the window");
    if close_response.clicked() {
        ui.send_viewport_cmd(egui::ViewportCommand::Close);
    }
}

fn code_block(ui: &mut egui::Ui, code: &str) {
    let language = "rs";
    let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());
    egui_extras::syntax_highlighting::code_view_ui(ui, &theme, code, language);
}
