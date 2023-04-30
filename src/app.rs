use bdk::psbt;
use egui::widget_text::RichText;
use egui::Color32;
use std::str::FromStr;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    #[serde(skip)]
    psbt_1: String,
    #[serde(skip)]
    psbt_2: String,
    #[serde(skip)]
    show_psbt_1: bool,
    #[serde(skip)]
    show_psbt_2: bool,
    #[serde(skip)]
    equal_psbt: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            psbt_1: "".to_owned(),
            psbt_2: "".to_owned(),
            show_psbt_1: false,
            show_psbt_2: false,
            equal_psbt: false,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            psbt_1,
            psbt_2,
            show_psbt_1,
            show_psbt_2,
            equal_psbt,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            egui::warn_if_debug_build(ui);

            ui.add(egui::github_link_file!(
                "https://github.com/matthiasdebernardini/psbt-plus-plus/blob/master/",
                "Source code."
            ));
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("bdk", "https://github.com/bitcoindevkit/bdk");
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.small_button("check_1").clicked() {
                let _psbt =
                    bitcoin::psbt::PartiallySignedTransaction::from_str(&self.psbt_1.as_str());
                if _psbt.is_ok() {
                    self.show_psbt_1 = true;
                };
            }
            if self.show_psbt_1 {
                let _psbt =
                    bitcoin::psbt::PartiallySignedTransaction::from_str(&self.psbt_1.as_str())
                        .unwrap();
                let unsigned_tx = _psbt.clone().unsigned_tx;
                let unsigned_tx = format!("{:?}", unsigned_tx);

                let inputs = _psbt.clone().inputs;
                let inputs = format!("{:?}", inputs);

                let outputs = _psbt.clone().outputs;
                let outputs = format!("{:?}", outputs);

                let xpub = _psbt.clone().xpub;
                for p in &xpub {
                    println!("{p:?}");
                }
                let label = match self.equal_psbt {
                    true => RichText::new("First PSBT").color(Color32::from_rgb(110, 255, 110)),
                    false => RichText::new("First PSBT"),
                };

                egui::Window::new(label).show(ctx, |ui| {
                    ui.collapsing("unsigned tx", |ui| {
                        ui.label(unsigned_tx);
                    });
                    ui.collapsing("inputs", |ui| {
                        ui.label(inputs);
                    });
                    ui.collapsing("outputs", |ui| {
                        ui.label(outputs);
                    });
                });
            };
            ui.add(egui::TextEdit::multiline(&mut self.psbt_1));

            if ui.small_button("check_2").clicked() {
                let _psbt =
                    bitcoin::psbt::PartiallySignedTransaction::from_str(&self.psbt_2.as_str());
                if _psbt.is_ok() {
                    self.show_psbt_2 = true;
                };
            }
            if self.psbt_1 == self.psbt_2 {
                self.equal_psbt = true
            } else {
                self.equal_psbt = false
            };
            if self.show_psbt_2 {
                let _psbt =
                    bitcoin::psbt::PartiallySignedTransaction::from_str(&self.psbt_2.as_str())
                        .unwrap();
                let unsigned_tx = _psbt.clone().unsigned_tx;
                let unsigned_tx = format!("{:?}", unsigned_tx);

                let inputs = _psbt.clone().inputs;
                let inputs = format!("{:?}", inputs);

                let outputs = _psbt.clone().outputs;
                let outputs = format!("{:?}", outputs);

                let xpub = _psbt.clone().xpub;
                for p in &xpub {
                    println!("{p:?}");
                }
                let label = match self.equal_psbt {
                    true => RichText::new("Second PSBT").color(Color32::from_rgb(110, 255, 110)),
                    false => RichText::new("Second PSBT"),
                };

                egui::Window::new(label)
                    //.frame(my_frame)
                    .show(ctx, |ui| {
                        ui.collapsing("unsigned tx", |ui| {
                            ui.label(unsigned_tx);
                        });
                        ui.collapsing("inputs", |ui| {
                            ui.label(inputs);
                        });
                        ui.collapsing("outputs", |ui| {
                            ui.label(outputs);
                        });
                    });
            };
            ui.add(egui::TextEdit::multiline(&mut self.psbt_2));
        });
    }
}
