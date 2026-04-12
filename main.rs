use eframe::egui;
use std::time::Instant;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust Windows Clock",
        native_options,
        Box::new(|_cc| Ok(Box::new(ClockApp::default()))),
    )
}

struct ClockApp {
    sw_start_time: Option<Instant>,
    sw_elapsed: std::time::Duration,
    timer_seconds: f32, 
    timer_running: bool,
}

impl Default for ClockApp {
    fn default() -> Self {
        Self {
            sw_start_time: None,
            sw_elapsed: std::time::Duration::ZERO,
            timer_seconds: 60.0,
            timer_running: false,
        }
    }
}

impl eframe::App for ClockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let dt = ctx.input(|i| i.stable_dt);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust Windows Clock");
            ui.separator();

            // --- ストップウォッチ ---
            ui.collapsing("StopWacth", |ui| {
                let display_dur = if let Some(start) = self.sw_start_time {
                    self.sw_elapsed + start.elapsed()
                } else {
                    self.sw_elapsed
                };

                ui.label(format!("{:02}.{:03}s", display_dur.as_secs(), display_dur.subsec_millis()));
                
                ui.horizontal(|ui| {
                    if ui.button("Start").clicked() && self.sw_start_time.is_none() {
                        self.sw_start_time = Some(Instant::now());
                    }
                    if ui.button("Stop").clicked() {
                        if let Some(start) = self.sw_start_time.take() {
                            self.sw_elapsed += start.elapsed();
                        }
                    }
                    if ui.button("Reset").clicked() {
                        self.sw_start_time = None;
                        self.sw_elapsed = std::time::Duration::ZERO;
                    }
                });
            });

            ui.separator();

            // --- タイマー ---
            ui.heading("Timer");
            ui.add(egui::Slider::new(&mut self.timer_seconds, 0.0..=3600.0).text("秒"));

            if ui.button(if self.timer_running { "Stop" } else { "Start" }).clicked() {
                self.timer_running = !self.timer_running;
            }

            if self.timer_running {
                if self.timer_seconds > 0.0 {
                    self.timer_seconds -= dt;
                } else {
                    self.timer_seconds = 0.0;
                    self.timer_running = false;
                }
            }

            let display_time = format!("{:.1} sec", self.timer_seconds);
            ui.label(egui::RichText::new(display_time).size(40.0).color(egui::Color32::LIGHT_BLUE));
        });

        // 常に再描画して秒数を更新
        ctx.request_repaint();
    }
}
