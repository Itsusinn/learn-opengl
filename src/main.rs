extern crate egui_sdl2_gl as egui_backend;
extern crate nalgebra as na;

use crate::scene::scene::Scene;
use crate::scene::spin;
use another::LateInit;
use anyhow::anyhow;
use egui_backend::{DpiScaling, ShaderVersion};
use glow::HasContext;
use na::Vector3;

use render_gl::offscreen::OffScreen;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::video::{GLProfile, SwapInterval};
use std::ops::DerefMut;
use std::path::Path;
use std::sync::RwLock;
use std::time::Instant;

#[macro_use]
extern crate render_gl_derive;

use crate::fonts::install_fonts;
use crate::resources::Resources;

pub mod fonts;
pub mod geom;
pub mod input;
pub mod render_gl;
pub mod resources;
pub mod scene;
mod time;

static GL: LateInit<glow::Context> = LateInit::new();

fn main() -> Result<(), anyhow::Error> {
  let mut screen_width = 1920;
  let mut screen_height = 1200;

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

  let window = video_subsystem
    .window("Another (SDL2 + OpenGL后端)", screen_width, screen_height)
    .resizable()
    .position_centered()
    .opengl()
    .build()?;

  let _ctx = window
    .gl_create_context()
    .map_err(|msg| anyhow!("创建GL上下文失败: {}", msg))?;

  let shader_ver = ShaderVersion::Adaptive;
  let (mut painter, mut egui_state) =
    egui_backend::with_sdl2(&window, shader_ver, DpiScaling::Custom(1.5)); // UI缩放，将影响<设备像素密度>
  let mut egui_ctx = egui::CtxRef::default();
  // 安装中文字体
  install_fonts(&egui_ctx);
  GL.init(unsafe {
    glow::Context::from_loader_function(|s| video_subsystem.gl_get_proc_address(s) as *const _)
  });

  // 获取<事件泵>，这是在SDL2中处理事件的传统方式
  let mut event_pump = sdl_context
    .event_pump()
    .map_err(|msg| anyhow!("事件泵获取失败: {}", msg))?;

  let mut viewport = render_gl::Viewport::for_window(screen_width as i32, screen_height as i32);
  viewport.refresh();

  let color_buffer = render_gl::ColorBuffer::from_color(Vector3::new(0.0, 0.0, 0.0));
  color_buffer.clear();

  let mut scene_manager: Vec<RwLock<Box<dyn Scene>>> = Vec::new();
  scene_manager.push(RwLock::new(Box::new(spin::Cube::new(&res)?)));

  scene_manager.push(RwLock::new(Box::new(scene::cube::Cube2::new(&res)?)));

  scene_manager.push(RwLock::new(Box::new(scene::phong::Cube::new(&res)?)));

  render_gl::debug::check_error();
  let mut scene_index = 0;

  let mut quit = false;
  let mut input_enable = false;
  let mut vsync = true;

  // todo
  let offscreen = OffScreen::new(&res, screen_width as i32, screen_height as i32)?;

  time::update();
  unsafe {
    GL.enable(glow::BLEND);
  }
  let start_time = Instant::now();
  'running: loop {
    window
      .subsystem()
      .gl_set_swap_interval(if vsync {
        SwapInterval::VSync
      } else {
        SwapInterval::Immediate
      })
      .unwrap();

    egui_state.input.time = Some(start_time.elapsed().as_secs_f64());
    egui_ctx.begin_frame(egui_state.input.take());

    viewport.refresh();
    // 自定义的OpenGL渲染部分
    time::update();
    let mut scene_rwlock = match scene_manager.get(scene_index) {
      Some(scene) => scene.write().unwrap(),
      None => scene_manager.get(0).unwrap().write().unwrap(),
    };
    let scene = &mut *scene_rwlock;
    if input_enable {
      scene.deref_mut().get_camera().handle_sdl_input();
    } else {
      let _ = input::fetch_motion();
    }
    offscreen.bind();
    unsafe {
      GL.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
      GL.enable(glow::DEPTH_TEST);
    }
    scene.render(screen_width as f32 / screen_height as f32);
    offscreen.detach();
    unsafe {
      GL.disable(glow::DEPTH_TEST);
    }
    offscreen.render_output();

    // egui的UI定义部分
    egui::Window::new("Egui 主窗口").show(&egui_ctx, |ui| {
      ui.label("使用LCtrl进入/退出摄像机模式");
      ui.label(format!("FPS: {}", (1.0 / time::get_delta()) as i32));
      ui.checkbox(&mut vsync, "垂直同步").clicked();
      ui.separator();
      ui.label(format!("视窗变换 宽 {} 高 {}", viewport.w, viewport.h));
      ui.label(format!(
        "窗口大小 宽 {} 高 {}",
        window.size().0,
        window.size().1
      ));
      ui.separator();
      if ui.button("Quit").clicked() {
        quit = true;
      }
    });
    egui::Window::new("场景轮换指示器")
      .resizable(false)
      .show(&egui_ctx, |ui| {
        ui.label("使用Tab + <-/-> 切换场景");
        ui.label(format!("场景索引 {}", scene_index));
        ui.label(format!("场景名称 {}", scene.get_name()));
      });
    scene.render_window(&egui_ctx);

    // egui前端完成渲染，生成后端无关的<绘制指令>
    let (egui_output, paint_cmds) = egui_ctx.end_frame();
    egui_state.process_output(&window, &egui_output);
    // 将egui<绘制指令>转化为<网格>(Mesh),即几何体集合
    let paint_jobs = egui_ctx.tessellate(paint_cmds);
    // 由egui后端完成实际的绘制
    painter.paint_jobs(None, paint_jobs, &egui_ctx.font_image());
    // 用OpenGL渲染结果更新窗口
    window.gl_swap_window();
    drop(scene_rwlock);

    for event in event_pump.poll_iter() {
      input::handle_sdl_input(&event);
      match event {
        Event::Quit { .. } => break 'running,
        Event::Window {
          win_event: WindowEvent::Resized(w, h),
          ..
        } => {
          screen_width = w as u32;
          screen_height = h as u32;
          viewport.update_size(screen_width as i32, screen_height as i32);
          offscreen.resize(screen_width as i32, screen_height as i32)?;
        }
        _ => {
          if !input_enable {
            // 将捕捉的输入传递给egui
            egui_state.process_input(&window, event, &mut painter);
          }
        }
      }
    }
    if input::get_key(Keycode::Escape) {
      quit = true;
    }
    if quit {
      break;
    }
    if input::get_key(Keycode::Tab) {
      if input::get_key_with_cooldown(Keycode::Left, 0.2) {
        if scene_index == 0 {
          scene_index = scene_manager.len() - 1;
        } else {
          scene_index -= 1;
        }
      }
      if input::get_key_with_cooldown(Keycode::Right, 0.2) {
        if scene_index + 1 == scene_manager.len() {
          scene_index = 0;
        } else {
          scene_index += 1;
        }
      }
    }
    if input::get_key_with_cooldown(Keycode::LCtrl, 0.2) {
      input_enable = !input_enable;
      mouse.set_relative_mouse_mode(input_enable);
    }
  }
  Ok(())
}
