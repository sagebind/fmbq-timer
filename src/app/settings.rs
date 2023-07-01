use eframe::{
    egui::{Checkbox, Layout, Response, RichText, Ui, Widget},
    emath::Align,
};

use super::widgets::{heading, title};

pub fn settings_page(ui: &mut Ui) {
    ui.add(title("Settings"));

    let mut my_bool = true;

    ui.add(Item {
        label: "Enable sounds".into(),
        description: None,
        value: Checkbox::without_text(&mut my_bool),
    });

    ui.add(Item {
        label: "Enable vibration".into(),
        description: None,
        value: Checkbox::without_text(&mut my_bool),
    });

    ui.add(Item {
        label: "Timer sound".into(),
        description: None,
        value: Checkbox::without_text(&mut my_bool),
    });

    ui.add(heading("Debug Info"));

    #[cfg(target_os = "android")]
    {
        let app = crate::platform::android::APP.get().unwrap();
        ui.label(format!("screen density: {:?}", app.config().density()));
        ui.label(format!("content rect: {:?}", app.content_rect()));
    }

    ui.add(heading("App Info"));
    ui.add_space(8.0);

    ui.add(Item {
        label: "Version".into(),
        description: None,
        value: |ui: &mut Ui| ui.label(env!("CARGO_PKG_VERSION")),
    });

    ui.label("Licenses");

    ui.separator();
}

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
