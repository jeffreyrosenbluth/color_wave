use eframe::{
    egui,
    epaint::{Color32, ColorImage},
    epi,
};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use wassily::prelude::*;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state

fn wave(color_wave: &ColorWaveApp) -> ColorImage {
    let mut canvas = Canvas::new(720, 300);
    let mut coscol = CosColor::default();
    coscol.r.a = color_wave.red_a;
    coscol.r.b = color_wave.red_b;
    coscol.r.freq = color_wave.red_freq;
    coscol.r.phase = color_wave.red_phase * TAU;
    coscol.g.a = color_wave.green_a;
    coscol.g.b = color_wave.green_b;
    coscol.g.freq = color_wave.green_freq;
    coscol.g.phase = color_wave.green_phase * TAU;
    coscol.b.a = color_wave.blue_a;
    coscol.b.b = color_wave.blue_b;
    coscol.b.freq = color_wave.blue_freq;
    coscol.b.phase = color_wave.blue_phase * TAU;
    for i in 0..360 {
        let c = coscol.cos_color(i as f32 / 180.0 * PI);
        ShapeBuilder::new()
            .line(pt(2.0 * i as f32, 0), pt(2.0 * i as f32, canvas.h_f32()))
            .stroke_weight(2.0)
            .stroke_color(c)
            .build()
            .draw(&mut canvas);
    }
    let mut buffer: Vec<Color32> = vec![];
    for p in canvas.pixels() {
        buffer.push(Color32::from_rgba_unmultiplied(
            p.red(),
            p.green(),
            p.blue(),
            p.alpha(),
        ));
    }
    ColorImage::from_rgba_unmultiplied([canvas.w_usize(), canvas.h_usize()], canvas.data())
}
pub struct ColorWaveApp {
    red_a: f32,
    red_b: f32,
    red_freq: f32,
    red_phase: f32,
    green_a: f32,
    green_b: f32,
    green_freq: f32,
    green_phase: f32,
    blue_a: f32,
    blue_b: f32,
    blue_freq: f32,
    blue_phase: f32,
}

impl Default for ColorWaveApp {
    fn default() -> Self {
        Self {
            red_a: 0.5,
            red_b: 0.5,
            red_freq: 1.0,
            red_phase: 0.0,
            green_a: 0.5,
            green_b: 0.5,
            green_freq: 1.0,
            green_phase: 0.1,
            blue_a: 0.5,
            blue_b: 0.5,
            blue_freq: 1.0,
            blue_phase: 0.2,
        }
    }
}

impl Distribution<ColorWaveApp> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ColorWaveApp {
        let red_a = rng.gen_range(0.25..=0.75);
        let red_b = 1.0 - red_a;
        let red_freq = rng.gen_range(0.5..=2.0);
        let red_phase = rng.gen_range(0.0..=0.5);
        let green_a = rng.gen_range(0.25..=0.75);
        let green_b = 1.0 - green_a;
        let green_freq = rng.gen_range(0.5..=2.0);
        let green_phase = rng.gen_range(0.0..=0.5);
        let blue_a = rng.gen_range(0.25..0.75);
        let blue_b = 1.0 - blue_a;
        let blue_freq = rng.gen_range(0.5..=2.0);
        let blue_phase = rng.gen_range(0.0..=0.5);
        ColorWaveApp {
            red_a,
            red_b,
            red_freq,
            red_phase,
            green_a,
            green_b,
            green_freq,
            green_phase,
            blue_a,
            blue_b,
            blue_freq,
            blue_phase,
        }
    }
}

impl epi::App for ColorWaveApp {
    fn name(&self) -> &str {
        "Procedural Color Generator"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self {
            red_a,
            red_b,
            red_freq,
            red_phase,
            green_a,
            green_b,
            green_freq,
            green_phase,
            blue_a,
            blue_b,
            blue_freq,
            blue_phase,
        } = self;
        frame.set_window_size(eframe::epaint::Vec2 {
            x: 1000.0,
            y: 480.0,
        });

        // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        //     // The top panel is often a good place for a menu bar:
        //     egui::menu::bar(ui, |ui| {
        //         ui.menu_button("File", |ui| {
        //             if ui.button("Quit").clicked() {
        //                 frame.quit();
        //             }
        //         });
        //     });
        // });

        egui::SidePanel::left("side_panel")
            .resizable(false)
            .min_width(250.0)
            .show(ctx, |ui| {
                ui.add_space(10.0);
                ui.heading("Controls");
                ui.add_space(20.0);
                ui.label(egui::RichText::new("Red").color(Color32::RED));
                ui.add(egui::Slider::new(red_a, 0.0..=1.0).text("a"));
                ui.add(egui::Slider::new(red_b, 0.0..=1.0).text("b"));
                ui.add(egui::Slider::new(red_freq, 0.0..=2.0).text("frequency"));
                ui.add(egui::Slider::new(red_phase, 0.0..=1.0).text("phase"));
                ui.add_space(20.0);
                ui.label(egui::RichText::new("Green").color(Color32::GREEN));
                ui.add(egui::Slider::new(green_a, 0.0..=1.0).text("a"));
                ui.add(egui::Slider::new(green_b, 0.0..=1.0).text("b"));
                ui.add(egui::Slider::new(green_freq, 0.0..=2.0).text("frequency"));
                ui.add(egui::Slider::new(green_phase, 0.0..=1.0).text("phase"));
                ui.add_space(20.0);
                ui.label(egui::RichText::new("Blue").color(Color32::LIGHT_BLUE));
                ui.add(egui::Slider::new(blue_a, 0.0..=1.0).text("a"));
                ui.add(egui::Slider::new(blue_b, 0.0..=1.0).text("b"));
                ui.add(egui::Slider::new(blue_freq, 0.0..=2.0).text("frequency"));
                ui.add(egui::Slider::new(blue_phase, 0.0..=1.0).text("phase"));
                ui.add_space(20.0);
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    if ui.button("Reset").clicked() {
                        *red_a = 0.5;
                        *red_b = 0.5;
                        *red_freq = 1.0;
                        *red_phase = 0.0;
                        *green_a = 0.5;
                        *green_b = 0.5;
                        *green_freq = 1.0;
                        *green_phase = 0.1;
                        *blue_a = 0.5;
                        *blue_b = 0.5;
                        *blue_freq = 1.0;
                        *blue_phase = 0.2;
                    }
                    ui.add_space(20.0);
                    if ui.button("Random").clicked() {
                        let mut rng = rand::thread_rng();
                        let vals: ColorWaveApp = rng.gen();
                        *red_a = vals.red_a;
                        *red_b = vals.red_b;
                        *red_freq = vals.red_freq;
                        *red_phase = vals.red_phase;
                        *green_a = vals.green_a;
                        *green_b = vals.green_b;
                        *green_freq = vals.green_freq;
                        *green_phase = vals.green_phase;
                        *blue_a = vals.blue_a;
                        *blue_b = vals.blue_b;
                        *blue_freq = vals.blue_freq;
                        *blue_phase = vals.blue_phase;
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.add_space(10.0);
            ui.heading("Color Palette");
            ui.add_space(40.0);
            egui::warn_if_debug_build(ui);

            let mut opt_texture: Option<egui::TextureHandle> = None;
            let texture: &egui::TextureHandle =
                opt_texture.get_or_insert_with(|| ui.ctx().load_texture("wave", wave(self)));
            let img_size = texture.size_vec2();
            ui.image(texture, img_size);
        });
    }
}
