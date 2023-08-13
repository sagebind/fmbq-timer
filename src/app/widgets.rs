//! Common shared widgets.

use std::{f32::consts::PI, fmt::Display};

use eframe::{
    egui::{Label, RichText, Sense, Ui, Widget},
    epaint::{pos2, vec2, Color32},
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

pub fn countdown_circle(size: f32, stroke: f32, percent: f32) -> impl Widget {
    move |ui: &mut Ui| {
        let (rect, response) = ui.allocate_at_least(vec2(size, size), Sense::hover());

        // Add some lively animation to percent changes.
        let percent = ui.ctx().animate_value_with_time(
            ui.make_persistent_id("countdown_circle_percent"),
            percent,
            0.25,
        );

        let radius = rect.width().min(rect.height()) / 2.0;

        // To draw an arc we will draw a number of discrete points along the
        // circumference. To figure out how many points we need, calculate the arc
        // length and divide by the size of the circles we are using to draw
        // points.
        let circumference = 2.0 * PI * radius;
        let arc_length = circumference * percent;
        let arc_length_in_circles = arc_length / stroke;

        // Multiply to add some overlap between the circles so that the arc looks
        // relatively smooth.
        let chunks = arc_length_in_circles.ceil() as i32 * 4;

        let chunks_radians = (percent * 2.0 * PI) / chunks as f32;
        let starting_radians = PI / 2.0;

        for i in 0..chunks {
            let radians = chunks_radians * i as f32 - starting_radians;
            let x = rect.center().x + radius * radians.cos();
            let y = rect.center().y + radius * radians.sin();

            ui.painter()
                .circle_filled(pos2(x, y), stroke / 2.0, ui.visuals().strong_text_color());
        }

        response
    }
}
