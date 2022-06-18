use std::ops::RangeInclusive;

pub fn edit_vec3(ui:&mut egui::Ui,value: &mut nalgebra::Vector3<f32>,range: RangeInclusive<f32>){
    ui.vertical(|ui|{
        ui.horizontal(|ui| {
          ui.add(egui::Slider::new(&mut value.x,range.clone()));
          ui.label("x");
        });
        ui.horizontal(|ui| {
          ui.add(egui::Slider::new(&mut value.y, range.clone()));
          ui.label("y");
        });
        ui.horizontal(|ui| {
          ui.add(egui::Slider::new(&mut value.z, range));
          ui.label("z");
        });
      });
}