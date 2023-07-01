//! Common shared widgets.

use std::fmt::Display;

use eframe::{
    egui::{Label, RichText, Ui, Widget},
    epaint::Color32,
};

pub fn title(text: impl Display) -> impl Widget {
    move |ui: &mut Ui| {
        ui.vertical(|ui| {
            ui.label(
                RichText::new(text.to_string())
                    .heading()
                    .size(24.0)
                    .color(Color32::LIGHT_BLUE),
            );

            ui.add_space(12.0);
        })
        .response
    }
}

pub fn heading(text: impl Display) -> impl Widget {
    Label::new(
        RichText::new(text.to_string())
            .heading()
            .color(Color32::LIGHT_BLUE),
    )
}
