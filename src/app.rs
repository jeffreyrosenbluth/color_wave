use eframe::{
    egui,
    epaint::{Color32, ColorImage},
    epi,
};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use wassily::prelude::{*, palette::{Hsluv, FromColor}};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state

fn wave(color_wave: &ColorWaveApp) -> ColorImage {
    let mut canvas = Canvas::new(720, 300);
    let mut proc_color = ProcColor::default();
    proc_color.scale = color_wave.scale;
    proc_color.seed = color_wave.seed;
    proc_color.c2.a = color_wave.channel2_a;
    proc_color.c2.b = color_wave.channel2_b;
    proc_color.c2.freq = color_wave.channel2_freq;
    proc_color.c3.a = color_wave.channel3_a;
    proc_color.c3.b = color_wave.channel3_b;
    proc_color.c3.freq = color_wave.channel3_freq;
    for i in 0..360 {
        let c = proc_color.proc_color(i as f32 / 180.0 * PI);
        let c_hsluv = Hsluv::new(c.0 * 360.0, c.1 * 100.0, c.2 * 100.0);
        let rgba = palette::Srgba::from_color(c_hsluv);
        let colr = Color::from_srgba(rgba);
        ShapeBuilder::new()
            .line(pt(2.0 * i as f32, 0), pt(2.0 * i as f32, canvas.h_f32() / 2.0))
            .stroke_weight(2.0)
            .stroke_color(colr)
            .build()
            .draw(&mut canvas);
        let rgb = okhsl_to_srgb(c.0, c.1, c.2);
        let colr = Color::from_rgba(rgb.0, rgb.1, rgb.2, 1.0).unwrap();
        ShapeBuilder::new()
            .line(pt(2.0 * i as f32, canvas.h_f32() / 2.0), pt(2.0 * i as f32, canvas.h_f32()))
            .stroke_weight(2.0)
            .stroke_color(colr)
            .build()
            .draw(&mut canvas);
    }
    ColorImage::from_rgba_unmultiplied([canvas.w_usize(), canvas.h_usize()], canvas.data())
}
pub struct ColorWaveApp {
    scale: f32,
    seed: f32,
    channel2_a: f32,
    channel2_b: f32,
    channel2_freq: f32,
    channel3_a: f32,
    channel3_b: f32,
    channel3_freq: f32,
}

impl Default for ColorWaveApp {
    fn default() -> Self {
        Self {
            scale: 0.2,
            seed: 0.0,
            channel2_a: 0.5,
            channel2_b: 0.5,
            channel2_freq: 1.0,
            channel3_a: 0.5,
            channel3_b: 0.35,
            channel3_freq: 1.0,
        }
    }
}

fn normal_approx<R: Rng + ?Sized>(rng: &mut R, min: f32, max: f32) -> f32 {
    let s: [f32; 8] = rng.gen();
    min + s.into_iter().sum::<f32>() * (max - min) * 0.125
}

impl Distribution<ColorWaveApp> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ColorWaveApp {
        let scale = rng.gen_range(0.0..1.0);
        let seed = rng.gen_range(0.0..10.0);
        let channel2_a: f32 = normal_approx(rng, 0.0, 1.0);
        let a2 = channel2_a.min(1.0 - channel2_a);
        let channel2_b = normal_approx(rng, a2 / 2.0, a2);
        let channel2_freq = normal_approx(rng, 0.0, 5.0);
        let channel3_a: f32 = normal_approx(rng, 0.0, 1.0);
        let a3 = channel3_a.min(1.0 - channel3_a);
        let channel3_b = normal_approx(rng, 0.0, a3);
        let channel3_freq = normal_approx(rng, 0.0, 5.0);
        ColorWaveApp {
            scale,
            seed,
            channel2_a,
            channel2_b,
            channel2_freq,
            channel3_a,
            channel3_b,
            channel3_freq,
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
            scale,
            seed,
            channel2_a,
            channel2_b,
            channel2_freq,
            channel3_a,
            channel3_b,
            channel3_freq,
        } = self;
        frame.set_window_size(eframe::epaint::Vec2 {
            x: 1000.0,
            y: 420.0,
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
                ui.label(egui::RichText::new("Channel 1").color(Color32::LIGHT_BLUE));
                ui.add(egui::Slider::new(scale, 0.0..=1.0).text("colors"));
                ui.add(egui::Slider::new(seed, 0.0..=10.0).step_by(0.001).text("seed"));
                ui.add_space(20.0);
                ui.label(egui::RichText::new("Channel 2").color(Color32::LIGHT_BLUE));
                ui.add(egui::Slider::new(channel2_a, 0.0..=1.0).text("a"));
                ui.add(egui::Slider::new(channel2_b, 0.0..=1.0).text("b"));
                ui.add(egui::Slider::new(channel2_freq, 0.0..=10.0).text("frequency"));
                ui.add_space(20.0);
                ui.label(egui::RichText::new("Channel 3").color(Color32::LIGHT_BLUE));
                ui.add(egui::Slider::new(channel3_a, 0.0..=1.0).text("a"));
                ui.add(egui::Slider::new(channel3_b, 0.0..=1.0).text("b"));
                ui.add(egui::Slider::new(channel3_freq, 0.0..=10.0).text("frequency"));
                ui.add_space(20.0);
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    if ui.button("Reset").clicked() {
                        *scale = 0.2;
                        *seed = 0.0;
                        *channel2_a = 0.5;
                        *channel2_b = 0.5;
                        *channel2_freq = 1.0;
                        *channel3_a = 0.5;
                        *channel3_b = 0.35;
                        *channel3_freq = 1.0;
                    }
                    ui.add_space(20.0);
                    if ui.button("Random").clicked() {
                        let mut rng = rand::thread_rng();
                        let vals: ColorWaveApp = rng.gen();
                        *scale = vals.scale;
                        *seed = vals.seed;
                        *channel2_a = vals.channel2_a;
                        *channel2_b = vals.channel2_b;
                        *channel2_freq = vals.channel2_freq;
                        *channel3_a = vals.channel3_a;
                        *channel3_b = vals.channel3_b;
                        *channel3_freq = vals.channel3_freq;
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
