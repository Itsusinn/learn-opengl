use egui::{FontDefinitions, FontFamily};

pub fn install_fonts(egui_ctx: &egui::CtxRef) {
  let mut fonts = FontDefinitions::default();
  fonts.font_data.insert(
    "LXGWWenKai-Regular".to_owned(),
    std::borrow::Cow::Borrowed(include_bytes!("../assets/fonts/LXGWWenKai-Regular.ttf")),
  );
  fonts
    .fonts_for_family
    .get_mut(&FontFamily::Monospace)
    .unwrap()
    .insert(0, "LXGWWenKai-Regular".to_owned());
  fonts
    .fonts_for_family
    .get_mut(&FontFamily::Proportional)
    .unwrap()
    .insert(0, "LXGWWenKai-Regular".to_owned());

  egui_ctx.set_fonts(fonts);
}
