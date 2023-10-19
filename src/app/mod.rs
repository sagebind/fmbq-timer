use std::time::Duration;

use eframe::{
    egui::{self, Button, Frame, Label, Layout, Margin, RichText, Sense, Ui},
    emath::{Align, Align2},
    epaint::{text::LayoutJob, vec2, Color32, FontId, Rect, Rounding},
    CreationContext,
};

use crate::{app::timer::Timer, PlatformContext};

use self::{
    colors::*,
    widgets::{countdown_circle, title, toolbar_button},
};

mod colors;
mod settings;
mod timer;
mod widgets;

const ANSWER_TIME: Duration = Duration::from_secs(20);
const PREJUMP_TIME: Duration = ANSWER_TIME;
const APPEAL_TIME: Duration = Duration::from_secs(30);
const TIME_OUT_TIME: Duration = Duration::from_secs(60);

pub struct App {
    platform_ctx: PlatformContext,
    timer: timer::Timer,
    settings_open: bool,
    content_margin: Margin,
}

impl App {
    pub const NAME: &str = "FMBQ Timer";

    pub fn new(ctx: &CreationContext, platform_ctx: PlatformContext) -> Self {
        log::info!("detected OS: {:?}", ctx.egui_ctx.os());
        log::info!("screen PPI: {}", ctx.egui_ctx.pixels_per_point());

        if ctx.storage.is_none() {
            log::debug!("storage is required");
        }

        let content_margin = Margin::default();

        let mut style = ctx.egui_ctx.style().as_ref().clone();
        let rounding = Rounding::same(6.0);
        style.spacing.button_padding = vec2(8.0, 8.0);
        style.visuals.widgets.inactive.rounding = rounding;
        style.visuals.widgets.active.rounding = rounding;
        style.visuals.widgets.hovered.rounding = rounding;
        style.visuals.widgets.inactive.bg_fill = PURPLE;
        style.visuals.widgets.inactive.weak_bg_fill = PURPLE;
        style.visuals.widgets.active.bg_fill = PURPLE;
        style.visuals.widgets.active.weak_bg_fill = PURPLE;
        style.visuals.widgets.hovered.bg_fill = PURPLE;
        style.visuals.widgets.hovered.weak_bg_fill = PURPLE;
        style.visuals.selection.bg_fill = PURPLE;
        style.visuals.button_frame = true;
        style.visuals.widgets.inactive.fg_stroke.color = Color32::WHITE;
        ctx.egui_ctx.set_style(style);

        Self {
            timer: Timer::new(platform_ctx.storage.clone()),
            platform_ctx,
            settings_open: false,
            content_margin,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Scale up everything by 50%
        if let Some(ppp) = frame.info().native_pixels_per_point {
            ctx.set_pixels_per_point(ppp * 1.5);
        }

        #[cfg(target_os = "android")]
        {
            let content_rect = self.platform_ctx.android_app.content_rect();
            // TODO: Why is the top of the content rect so large?
            self.content_margin.top =
                (content_rect.top as f32 / ctx.pixels_per_point()).clamp(0.0, 32.0);
            self.content_margin.bottom = frame.info().window_info.size.y
                - (content_rect.bottom as f32 / ctx.pixels_per_point());
        }

        let timer_result = self.timer.state();

        egui::CentralPanel::default().show(ctx, |ui| {
            // Account for mobile display margins
            ui.add_space(self.content_margin.top);

            if self.settings_open {
                settings::settings_page(
                    ui,
                    &self.platform_ctx.storage,
                    &mut self.timer,
                    &self.platform_ctx,
                );
            } else {
                main_page(ui, &mut self.timer, timer_result);
            }
        });

        egui::TopBottomPanel::bottom("toolbar")
            .frame(Frame::none().fill(Color32::from_white_alpha(2)))
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.add_space(6.0);

                ui.columns(2, |columns| {
                    columns[0].with_layout(Layout::top_down(Align::Center), |ui| {
                        if ui
                            .add(toolbar_button('⏱', "Timer", !self.settings_open))
                            .clicked()
                        {
                            self.settings_open = false;
                        }
                    });

                    columns[1].vertical_centered(|ui| {
                        if ui
                            .add(toolbar_button('⚙', "Settings", self.settings_open))
                            .clicked()
                        {
                            self.settings_open = true;
                        }
                    });
                });

                ui.add_space(4.0);

                // Account for mobile display margins
                ui.add_space(self.content_margin.bottom);
            });

        // If we just started the timer this frame then we also need to start
        // repainting.
        if self.timer.is_running() {
            ctx.request_repaint_after(Duration::from_millis(10));
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.platform_ctx.storage.flush();
    }
}

fn main_page(ui: &mut Ui, timer: &mut timer::Timer, timer_result: timer::State) {
    ui.add(title("FMBQ Timer"));

    // Shift layout depending on screen orientation.
    let landscape_mode = ui.available_size().x > ui.available_size().y;

    if landscape_mode {
        ui.columns(2, |columns| {
            timer_display(&mut columns[0], timer, timer_result);
            timer_buttons(&mut columns[1], timer);
        });
    } else {
        timer_display(ui, timer, timer_result);
        timer_buttons(ui, timer);
    }
}

fn timer_display(ui: &mut Ui, timer: &mut Timer, timer_result: timer::State) {
    let response = ui
        .vertical_centered_justified(|ui| {
            let percent = if let timer::State::Running { remaining, total } = timer_result {
                remaining.as_secs_f32() / total.as_secs_f32()
            } else {
                1.0
            };

            let circle_response = ui.add(countdown_circle(160.0, 5.0, percent));

            let value = if let timer::State::Running { remaining, .. } = timer_result {
                remaining.as_secs_f32()
            } else {
                0.0
            };

            // Draw a label directly at the center of the countdown circle.
            ui.put(
                circle_response.rect,
                Label::new(
                    RichText::new(format!("{:.1}", value))
                        .size(60.0)
                        .color(ui.visuals().strong_text_color()),
                ),
            );

            // If the timer is running, show a cancel button that allows the user
            // to reset the timer.
            if timer_result.is_running() {
                let cancel_button_area = Rect::from_two_pos(
                    circle_response.rect.center_bottom() + vec2(40.0, -40.0),
                    circle_response.rect.center_bottom() + vec2(80.0, 0.0),
                );

                // Place the button on top of the countdown circle at the bottom
                // right edge.
                if ui
                    .put(
                        cancel_button_area,
                        Button::new(RichText::new("✖").size(16.0))
                            .frame(false)
                            .min_size(vec2(40.0, 40.0))
                            .rounding(Rounding::same(40.0))
                            .fill(ORANGE),
                    )
                    .clicked()
                {
                    timer.reset();
                };
            }

            ui.add_space(16.0);
        })
        .response;

    // Tapping anywhere in the area of the timer will also start the timer for
    // the default answer time. For lower experienced latency, the timer starts
    // when a press starts rather than waiting for release.
    if response
        .interact(egui::Sense::click())
        .is_pointer_button_down_on()
        && ui.input(|i| i.pointer.primary_clicked())
    {
        timer.start(ANSWER_TIME);
        ui.ctx().request_repaint();
    }
}

fn timer_buttons(ui: &mut Ui, timer: &mut Timer) {
    ui.vertical_centered_justified(|ui| {
        if ui
            .add(
                Button::new(RichText::new("Answer").size(20.0))
                    .min_size(vec2(200.0, 48.0))
                    .fill(BABY_BLUE),
            )
            .clicked()
        {
            timer.start(ANSWER_TIME);
            ui.ctx().request_repaint();
        }

        ui.add_space(8.0);

        if ui.add(Button::new("Prejump")).clicked() {
            timer.start(PREJUMP_TIME);
            ui.ctx().request_repaint();
        }

        ui.add_space(8.0);

        ui.columns(2, |columns| {
            columns[0].with_layout(Layout::top_down_justified(Align::Center), |ui| {
                if ui.add(Button::new("Appeal")).clicked() {
                    timer.start(APPEAL_TIME);
                    ui.ctx().request_repaint();
                }
            });

            columns[1].with_layout(Layout::top_down_justified(Align::Center), |ui| {
                if ui.add(Button::new("Time Out")).clicked() {
                    timer.start(TIME_OUT_TIME);
                    ui.ctx().request_repaint();
                }
            });
        });

        ui.add_space(8.0);

        if ui.add(Button::new("Test timer sound")).clicked() {
            timer.test_audio();
            ui.ctx().request_repaint();
        }
    });
}
