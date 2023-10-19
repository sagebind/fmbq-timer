use appstorage::Storage;
use eframe::{
    egui::{Checkbox, ComboBox, Layout, Response, RichText, ScrollArea, Ui, Widget},
    emath::Align,
};

use crate::sounds::SoundLibrary;

use super::{widgets::{heading, title}, timer::Timer};

pub fn settings_page(ui: &mut Ui, storage: &Storage, timer: &mut Timer, _platform_ctx: &crate::PlatformContext) {
    ui.add(title("Settings"));

    ScrollArea::vertical().show(ui, |ui| {
        let mut enable_sounds = storage.get::<bool>("enable-sounds").unwrap_or(true);
        let mut sound = storage.get::<String>("sound").unwrap_or_else(|| "correct".to_string());
        let mut my_bool = true;

        ui.add(Item {
            label: "Enable sounds".into(),
            description: None,
            value: Checkbox::without_text(&mut enable_sounds),
        });

        ui.add(Item {
            label: "Enable vibration".into(),
            description: None,
            value: Checkbox::without_text(&mut my_bool),
        });

        ui.add(Item {
            label: "Timer sound".into(),
            description: None,

            value: |ui: &mut Ui| {
                ComboBox::from_id_source("timer_sound")
                    .selected_text(&sound)
                    .show_ui(ui, |ui| {
                        for name in SoundLibrary::get().get_names() {
                            if ui
                                .selectable_value(&mut sound, name.clone(), &name)
                                .changed()
                            {
                                storage.set("sound", sound.clone());
                                storage.flush();
                                timer.test_audio();
                            }
                        }
                    })
                    .response
            },
        });

        ui.add(heading("About"));
        ui.add_space(8.0);

        ui.add(Item {
            label: "Version".into(),
            description: None,
            value: |ui: &mut Ui| {
                ui.with_layout(Layout::top_down(Align::Max), |ui| {
                    ui.label(env!("CARGO_PKG_VERSION"));
                    ui.weak(format!("build date {}", crate::BUILD_TIME_STR));
                })
                .response
            },
        });

        ui.label("Licenses");

        ui.separator();

        storage.set("enable-sounds", enable_sounds);
    });
}

/// An item on the settings page.
struct Item<W> {
    label: RichText,
    description: Option<RichText>,
    value: W,
}

impl<W: Widget> Widget for Item<W> {
    fn ui(self, ui: &mut Ui) -> Response {
        let response = ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.strong(self.label);

                if let Some(description) = self.description {
                    ui.weak(description);
                }
            });

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                self.value.ui(ui);
            });
        });

        ui.separator();

        response.response
    }
}
