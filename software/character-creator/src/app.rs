/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    /// State of LEDs
    #[serde(skip)]
    leds: [bool; 64],

    /// The decimal value of the LED on/off state
    decimal_value: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
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
    // Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    // Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("handwave character creator");

            // Try to parse the current decimal value and use that to
            // set the LEDs. This allows the user to edit the value in the
            // text field and have it change the.

            // Fetch the current value
            let decimal_value_as_string = self.decimal_value.clone();

            // Try to parse the current value as a u64. If it succeeds..
            if let Ok(decimal_value) = decimal_value_as_string.parse::<u64>() {
                // For each bit..
                for i in 0..64 {
                    // Use bitwise logic to detemine if the i'th bit is a 0 or 1
                    let led_on = (decimal_value & (1_u64 << i)) != 0;

                    // Update the LED state based on the boolean value
                    self.leds[i] = led_on;
                }
            }

            // draw the checkboxes
            for x in 0..8 {
                ui.horizontal(|ui| {
                    for y in 0..8 {
                        // We pass in a mutable reference to the LED at the
                        // current location. With that mutable reference,
                        // egui handles updating the value when the checkbox
                        // changes.
                        let checkbox =
                            egui::Checkbox::without_text(&mut self.leds[(7 - x) + (y) * 8]);

                        // Add the checkbox to our layout
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

                ui.label(format!("{:064b}", ret));
                if ui.button("Clear").clicked() {
                    self.decimal_value = "0".to_string();
                }
            });
        });
    }
}
