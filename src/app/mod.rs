use std::time::Duration;

use eframe::{
    egui::{self, Button, Frame, Label, Layout, Margin, RichText, Sense, Ui},
    emath::{Align, Align2},
    epaint::{vec2, Color32, Rounding},
    CreationContext,
};

use self::colors::ORANGE;
use self::widgets::title;
use self::{colors::BLUE, widgets::countdown_circle};

mod audio;
mod colors;
mod settings;
mod timer;
mod widgets;

const ANSWER_TIME: Duration = Duration::from_secs(20);
const PREJUMP_TIME: Duration = ANSWER_TIME;
const APPEAL_TIME: Duration = Duration::from_secs(30);
const TIME_OUT_TIME: Duration = Duration::from_secs(60);

pub struct App {
    timer: timer::Timer,
    audio_player: audio::Player,
    settings_open: bool,
    content_margin: Margin,
}

impl App {
    pub const NAME: &str = "FMBQ Timer";

    pub fn new(ctx: &CreationContext) -> Self {
        log::info!("detected OS: {:?}", ctx.egui_ctx.os());
        log::info!("screen PPI: {}", ctx.egui_ctx.pixels_per_point());
        // ctx.egui_ctx.set_pixels_per_point(4.0);

        let mut content_margin = Margin::default();

        #[cfg(target_os = "android")]
        {
            let app = crate::platform::android::APP.get().unwrap();
            let content_rect = app.content_rect();
            // TODO: Why is the top of the content rect so large?
            content_margin.top = 0.0;//(content_rect.top as f32 / ctx.egui_ctx.pixels_per_point()).clamp(0.0, 2.0);
            content_margin.bottom = ctx.integration_info.window_info.size.y
                - (content_rect.bottom as f32 / ctx.egui_ctx.pixels_per_point());
        }

        let mut style = ctx.egui_ctx.style().as_ref().clone();
        let rounding = Rounding::same(6.0);
        style.spacing.button_padding = vec2(8.0, 4.0);
        style.visuals.widgets.inactive.rounding = rounding;
        style.visuals.widgets.active.rounding = rounding;
        style.visuals.widgets.hovered.rounding = rounding;
        style.visuals.widgets.active.bg_fill = BLUE;
        style.visuals.selection.bg_fill = BLUE;
        style.visuals.button_frame = false;
        style.visuals.widgets.inactive.fg_stroke.color = Color32::WHITE;
        ctx.egui_ctx.set_style(style);

        Self {
            timer: Default::default(),
            audio_player: audio::Player::new(),
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
            let app = crate::platform::android::APP.get().unwrap();
            let content_rect = app.content_rect();
            // TODO: Why is the top of the content rect so large?
            self.content_margin.top = (content_rect.top as f32 / ctx.pixels_per_point()).clamp(0.0, 32.0);
            self.content_margin.bottom = frame.info().window_info.size.y
                - (content_rect.bottom as f32 / ctx.pixels_per_point());
        }

        let timer_result = self.timer.update();

        egui::CentralPanel::default().show(ctx, |ui| {
            // Account for mobile display margins
            ui.add_space(self.content_margin.top);

            if self.settings_open {
                settings::settings_page(ui);
                ui.label(format!("screen PPI: {}", ctx.pixels_per_point()));
                ui.label(format!("content margin: {:?}", self.content_margin));
            } else {
                main_page(ui, &mut self.timer, timer_result);
            }
        });

        if !self.settings_open {
            egui::Area::new("countdown_circle_overlay")
                .anchor(Align2::CENTER_TOP, vec2(0.0, 56.0 + self.content_margin.top))
                .interactable(false)
                .show(ctx, |ui| {
                    let percent = if let timer::UpdateResult::Running(remaining) = timer_result {
                        remaining.as_secs_f32() / ANSWER_TIME.as_secs_f32()
                    } else {
                        1.0
                    };
                    ui.add(countdown_circle(160.0, 5.0, percent));
                });

            egui::Area::new("reset_button_overlay")
                .anchor(Align2::CENTER_TOP, vec2(56.0, 176.0 + self.content_margin.top))
                .show(ctx, |ui| {
                    ui.add_visible_ui(
                        matches!(timer_result, timer::UpdateResult::Running(_)),
                        |ui| {
                            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                                if ui
                                    .add(
                                        Button::new(RichText::new("⟲").size(16.0))
                                            .frame(false)
                                            .min_size(vec2(40.0, 40.0))
                                            .rounding(Rounding::same(40.0))
                                            .fill(ORANGE),
                                    )
                                    .clicked()
                                {
                                    self.timer.reset();
                                }
                            });
                        },
                    );
                });
        }

        egui::TopBottomPanel::bottom("toolbar")
            .frame(Frame::none().fill(Color32::from_white_alpha(2)))
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.add_space(6.0);

                ui.columns(2, |columns| {
                    columns[0].with_layout(Layout::top_down(Align::Center), |ui| {
                        if ui
                            .selectable_label(!self.settings_open, "⏱\nTimer")
                            .clicked()
                        {
                            self.settings_open = false;
                        }
                    });

                    columns[1].vertical_centered(|ui| {
                        if ui
                            .selectable_label(self.settings_open, "⚙\nSettings")
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

        if timer_result == timer::UpdateResult::Expired {
            self.audio_player.play_timer_sound();
        }

        // If we just started the timer this frame then we also need to start
        // repainting.
        if matches!(self.timer.update(), timer::UpdateResult::Running(_)) {
            ctx.request_repaint_after(Duration::from_millis(10));
        }
    }
}

fn main_page(ui: &mut Ui, timer: &mut timer::Timer, timer_result: timer::UpdateResult) {
    ui.add(title("Timer"));

    ui.add_space(48.0);

    ui.vertical_centered(|ui| {
        let value = if let timer::UpdateResult::Running(remaining) = timer_result {
            remaining.as_secs_f32()
        } else {
            0.0
        };

        if ui
            .add(
                Label::new(RichText::new(format!("{:.1}", value)).size(64.0)).sense(Sense::click()),
            )
            .clicked()
        {
            timer.start(ANSWER_TIME);
        }
    });

    ui.add_space(64.0);

    ui.vertical_centered_justified(|ui| {
        if ui
            .add(
                Button::new(RichText::new("Answer").size(20.0))
                    .min_size(vec2(200.0, 48.0))
                    .fill(Color32::from_rgb(0x2E, 0xCC, 0x40)),
            )
            .clicked()
        {
            timer.start(ANSWER_TIME);
        }

        ui.add_space(8.0);

        if ui.add(Button::new("Prejump").fill(BLUE)).clicked() {
            timer.start(PREJUMP_TIME);
        }

        ui.add_space(8.0);

        ui.columns(2, |columns| {
            columns[0].with_layout(Layout::top_down_justified(Align::Center), |ui| {
                if ui.add(Button::new("Appeal").fill(BLUE)).clicked() {
                    timer.start(APPEAL_TIME);
                }
            });

            columns[1].with_layout(Layout::top_down_justified(Align::Center), |ui| {
                if ui.add(Button::new("Time Out").fill(BLUE)).clicked() {
                    timer.start(TIME_OUT_TIME);
                }
            });
        });
    });
}
