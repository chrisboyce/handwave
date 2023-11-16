/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    #[serde(skip)]
    leds: [bool; 64],

    decimal_value: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            leds: [false; 64],
            decimal_value: String::new(),
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("handwave character creator");

            let decimal_value_as_string = self.decimal_value.clone();

            if let Ok(decimal_value) = decimal_value_as_string.parse::<u64>() {
                for i in 0..64 {
                    let led_on = (decimal_value & (1_u64 << i)) != 0;
                    self.leds[i] = led_on;
                }
            }

            for x in 0..8 {
                ui.horizontal(|ui| {
                    for y in 0..8 {
                        let checkbox =
                            egui::Checkbox::without_text(&mut self.leds[x + (7 - y) * 8]);
                        ui.add(checkbox);
                    }
                });
            }

            ui.vertical(|ui| {
                let mut ret = 0;
                for (i, led_on) in self.leds.iter().enumerate() {
                    if *led_on {
                        ret |= 1_u64 << i;
                    }
                }
                self.decimal_value = format!("{ret}");
                ui.text_edit_singleline(&mut self.decimal_value);

                // Uncomment to show binary representation
                // ui.label(format!("{:064b}", ret));
            });

            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
