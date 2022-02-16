use eframe::{egui::{self, FontDefinitions, FontFamily, FontData}, epi};

use crate::keys;
pub struct KanjiTreeApp{
    activeKanji: String,
}

impl Default for KanjiTreeApp {
    fn default() -> Self {
        Self {
            activeKanji: String::from("狐"),
        }
    }
}

impl epi::App for KanjiTreeApp {
    fn name(&self) -> &str {
        "eframe template"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        let mut font = FontDefinitions::default();
        font.font_data.insert(
            keys::FONT_GOTHIC.to_string(),
            FontData{
                font: std::borrow::Cow::Borrowed(
                    include_bytes!("../fonts/msgothic.ttc")
                ),
                index: 0
            }
        );
        font.fonts_for_family.get_mut(&FontFamily::Monospace).unwrap().insert(0, keys::FONT_GOTHIC.to_string());
        font.fonts_for_family.get_mut(&FontFamily::Proportional).unwrap().insert(0, keys::FONT_GOTHIC.to_string());
        _ctx.set_fonts(font);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        let Self { 
            activeKanji
        } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("Kanji Tree");
            ui.label(&*activeKanji);
        });
    }
}