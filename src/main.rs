fn main() {
    let app = kanji_tree::KanjiTreeApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
