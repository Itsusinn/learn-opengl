extern crate another_gl as gl;
extern crate egui_sdl2_gl as egui_backend;
extern crate nalgebra as na;
#[macro_use]
extern crate lazy_static;

use crate::mesh::cube;
use crate::mesh::scene::Scene;
use anyhow::anyhow;
use egui_backend::{egui, DpiScaling};
use na::Vector3;
use render_gl::frame_buffer::FrameBuffer;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::video::{GLProfile, SwapInterval};
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;

#[macro_use]
extern crate render_gl_derive;
use crate::app_frame::Frame;
use crate::fonts::install_fonts;
use crate::resources::Resources;

pub mod app_frame;
pub mod fonts;
pub mod geom;
pub mod input;
pub mod mesh;
pub mod render_gl;
pub mod resources;
mod time;

const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1200;

fn main() -> Result<(), anyhow::Error> {
  println!("hello!");
  let res = Resources::from_relative_exe_path(Path::new("assets"))?;
  let sdl_context = sdl2::init().map_err(|msg| anyhow!("Sdl2 初始化失败 {}", msg))?;
  let video_subsystem = sdl_context
    .video()
    .map_err(|msg| anyhow!("视频子系统获取失败 {}", msg))?;

  let mouse = sdl_context.mouse();

  let gl_attr = video_subsystem.gl_attr();
  gl_attr.set_context_profile(GLProfile::Core);
  // egui支持下限为320
  gl_attr.set_context_version(4, 5);

  let mut window = video_subsystem
    .window(
      "Egui dispaly  (SDL2 + OpenGL backend)",
      SCREEN_WIDTH,
      SCREEN_HEIGHT,
    )
    .resizable()
    .position_centered()
    .opengl()
    .build()?;

  let _ctx = window
    .gl_create_context()
    .map_err(|msg| anyhow!("创建GL上下文失败: {}", msg))?;

  let (mut painter, mut egui_state) = egui_backend::with_sdl2(&window, DpiScaling::Custom(4.0)); // UI缩放，将影响<设备像素密度>
  let mut egui_ctx = egui::CtxRef::default();
  // 安装中文字体
  install_fonts(&egui_ctx);
  let gl: Rc<gl::Gl> = Rc::new(gl::Gl::load_with(|s| {
    video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
  }));

  // 获取<事件泵>，这是在SDL2中处理事件的传统方式
  let mut event_pump = sdl_context
    .event_pump()
    .map_err(|msg| anyhow!("事件泵获取失败: {}", msg))?;

  let mut viewport = render_gl::Viewport::for_window(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
  viewport.refresh(&gl);

  let color_buffer = render_gl::ColorBuffer::from_color(Vector3::new(1.0, 1.0, 1.0));
  color_buffer.clear(&gl);

  let mut test_str: String = "用于输入的文本框。剪切、复制、粘贴命令可用".to_owned();
  let scene = cube::Cube::new(&res, &gl)?;
  let mut scene: Box<dyn Scene> = Box::new(scene);

  let mut quit = false;
  let mut input_enable = false;
  let mut vsync = false;
  let frame_buffer = FrameBuffer::new(&gl, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
  let frame = Frame::new(&res, &gl, &frame_buffer)?;
  time::update();
  unsafe {
    gl.Enable(gl::BLEND);
  }

  let start_time = Instant::now();
  'running: loop {
    if !vsync {
      window
      .subsystem()
      .gl_set_swap_interval(SwapInterval::Immediate)
      .unwrap();
    } else {
      window
      .subsystem()
      .gl_set_swap_interval(SwapInterval::VSync)
      .unwrap();
    }

    egui_state.input.time = Some(start_time.elapsed().as_secs_f64());
    egui_ctx.begin_frame(egui_state.input.take());

    viewport.refresh(&gl);
    // 自定义的OpenGL渲染部分
    time::update();
    scene.get_camera().handle_sdl_input();
    frame_buffer.bind();
    unsafe {
      gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
      gl.Enable(gl::DEPTH_TEST);
    }
    scene.render(&gl);
    frame_buffer.detach();
    unsafe {
      gl.Disable(gl::DEPTH_TEST);
    }
    frame.render(&gl);

    // egui的UI定义部分
    egui::Window::new("Egui with SDL2 and GL").show(&egui_ctx, |ui| {
      ui.separator();
      ui.label(format!("FPS: {}", 1.0 / time::get_delta()));
      ui.label("");
      ui.label("这是egui的演示用文本");
      ui.label(" ");
      if ui.selectable_label(vsync,"垂直同步").clicked() {
        vsync = !vsync
      };
      ui.text_edit_multiline(&mut test_str);
      ui.label(" ");
      if ui.button("Quit").clicked() {
        quit = true;
      }
    });
    // egui前端完成渲染，生成后端无关的<绘制指令>
    let (egui_output, paint_cmds) = egui_ctx.end_frame();
    egui_state.process_output(&egui_output);
    // 将egui<绘制指令>转化为<网格>(Mesh),即几何体集合
    let paint_jobs = egui_ctx.tessellate(paint_cmds);
    // 由egui后端完成实际的绘制
    painter.paint_jobs(None, paint_jobs, &egui_ctx.texture());
    // 用OpenGL渲染结果更新窗口
    window.gl_swap_window();
    for event in event_pump.poll_iter() {
      input::handle_sdl_input(&event);
      match event {
        Event::Quit { .. } => break 'running,
        Event::Window {
          win_event: WindowEvent::Resized(w, h),
          ..
        } => {
          viewport.update_size(w, h);
          window.set_size(w as u32, h as u32).unwrap();
        }
        _ => {
          // 将捕捉的输入传递给egui
          egui_state.process_input(&window, event, &mut painter);
        }
      }
    }
    if input::get_key(Keycode::Escape, false) {
      quit = true;
    }
    if quit {
      break;
    }
    if input::get_key_with_cooldown(Keycode::LCtrl, false, 0.2) {
      input_enable = !input_enable;
      mouse.set_relative_mouse_mode(input_enable);
      if input_enable {
        scene.get_camera().enable();
      } else {
        scene.get_camera().disable();
      }
    }
  }
  Ok(())
}
