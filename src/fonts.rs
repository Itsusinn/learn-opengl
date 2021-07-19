use egui::{FontDefinitions, FontFamily};

pub fn install_fonts(egui_ctx: &egui::CtxRef){
    let mut font_definitions = FontDefinitions::default();
    font_definitions.font_data.insert(
        "LXGWWenKai-Regular".to_owned(),
        std::borrow::Cow::Borrowed(include_bytes!("../static-assets/fonts/LXGWWenKai-Regular.ttf")),
    );
    font_definitions.fonts_for_family.insert(
        FontFamily::Monospace,
        vec!["LXGWWenKai-Regular".to_owned()]
    );
    font_definitions.fonts_for_family.insert(
        FontFamily::Proportional,
        vec!["LXGWWenKai-Regular".to_owned()]
    );
    egui_ctx.set_fonts(font_definitions);
}