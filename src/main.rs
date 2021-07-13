extern crate another_gl as gl;
extern crate  nalgebra as na;
extern crate egui_sdl2_gl as egui_backend;
#[macro_use]
extern crate lazy_static;

use na::Vector3;
use egui_backend::egui;
use sdl2::Sdl;
use sdl2::{keyboard::Keycode, mouse::MouseUtil};
use sdl2::event::{Event, WindowEvent};
use sdl2::video::GLProfile;
use std::path::Path;
use std::rc::Rc;
use egui_backend::egui::{vec2, Pos2, Rect};
use anyhow::anyhow;
use render_gl::frame_buffer::FrameBuffer;

#[macro_use] extern crate render_gl_derive;
use crate::fonts::install_fonts;
use crate::frame::Frame;
use crate::resources::Resources;

pub mod model;
pub mod input;
pub mod triangles;
pub mod render_gl;
pub mod geom;
pub mod resources;
pub mod fonts;
mod time;
pub mod frame;

const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1200;

fn main()-> Result<(),anyhow::Error> {
    println!("hello!");
    let res =
        Resources::from_relative_exe_path(Path::new("assets"))?;
    let sdl_context = sdl2::init()
      .map_err(|msg| anyhow!("Sdl2 初始化失败 {}",msg))?;
    let video_subsystem = sdl_context.video()
      .map_err(|msg| anyhow!("视频子系统获取失败 {}", msg))?;

    let mouse = sdl_context.mouse();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    // egui支持下限为320
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window(
            "Egui演示  (SDL2 + GL后端)",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        )
        .resizable()
        .position_centered()
        .opengl()
        .build()?;

    let _ctx = window.gl_create_context()
        .map_err(|msg| anyhow!("创建GL上下文失败: {}",msg))?;

    let mut painter = egui_backend::Painter::new(&video_subsystem, SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut egui_ctx = egui::CtxRef::default();
    // 安装中文字体
    install_fonts(&egui_ctx);

   let gl: Rc<gl::Gl> = Rc::new(
      gl::Gl::load_with(|s| {
         video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
      })
   );
    // UI缩放，将影响<设备像素密度>
    let ui_zoom = 4.2f32;
   // 获取<事件泵>，这是在SDL2中处理事件的传统方式
    let mut event_pump = sdl_context.event_pump()
        .map_err(|msg| anyhow!("事件泵获取失败: {}",msg))?;

    //  fixme:查明换算公式
    let native_pixels_per_point =  (ui_zoom * 96f32) / video_subsystem.display_dpi(0).unwrap().0;

    let (width, height) = window.size();

    // egui输入状态，主要影响ui的大小以及保证输入位置的准确性
    let mut egui_input_state = egui_backend::EguiInputState::new(egui::RawInput {
        screen_rect: Some(Rect::from_min_size(
            Pos2::new(0f32, 0f32),
            vec2(width as f32, height as f32) / native_pixels_per_point,
        )),
        pixels_per_point: Some(native_pixels_per_point),
        ..Default::default()
    });
    let mut viewport =
        render_gl::Viewport::for_window(SCREEN_WIDTH as i32,SCREEN_HEIGHT as i32);
    viewport.refresh(&gl);

    let color_buffer =
        render_gl::ColorBuffer::from_color(Vector3::new(1.0, 1.0, 1.0));
    color_buffer.clear(&gl);

    let mut test_str: String = "用于输入的文本框。剪切、复制、粘贴命令可用".to_owned();
    let mut square = triangles::Square::new(&res, &gl)?;

    let mut quit = false;
    let mut input_enable = false;
    let frame_buffer =FrameBuffer::new(&gl,SCREEN_WIDTH as i32,SCREEN_HEIGHT as i32);
    let frame = Frame::new(&res, &gl, &frame_buffer)?;
    time::update();
    unsafe {
        gl.Enable(gl::BLEND);
    }
    'running: loop {
        egui_ctx.begin_frame(egui_input_state.input.take());
        // 每次渲染都会丢失设备像素的数据，推测是egui的行为
        egui_input_state.input.pixels_per_point = Some(native_pixels_per_point);

        // 每次渲染都会丢失视窗变换的数据，推测是egui的行为
        viewport.refresh(&gl);
        // 自定义的OpenGL渲染部分
        time::update();
        square.camera.handle_sdl_input();
        frame_buffer.bind();
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl.Enable(gl::DEPTH_TEST);
        }
        square.render(&gl);
        frame_buffer.detach();
        unsafe {
            gl.Disable(gl::DEPTH_TEST);
        }
        frame.render(&gl);
        // egui的UI定义部分
        egui::Window::new("Egui with SDL2 and GL").show(&egui_ctx, |ui| {
            ui.separator();
            ui.label(format!("FPS: {}",1.0 / time::get_delta()));
            ui.label("");
            ui.label("这是egui的演示用文本");
            ui.label(" ");
            ui.text_edit_multiline(&mut test_str);
            ui.label(" ");
            if ui.button("Quit").clicked() {
                quit = true;
            }
        });
        // egui前端完成渲染，生成后端无关的<绘制指令>
        let (_, paint_cmds) = egui_ctx.end_frame();
        // 将egui<绘制指令>转化为<网格>(Mesh),即几何体集合
        let paint_jobs = egui_ctx.tessellate(paint_cmds);
        // 由egui后端完成实际的绘制
        painter.paint_jobs(
            None,
            paint_jobs,
            &egui_ctx.texture(),
            native_pixels_per_point,
        );
        // 用OpenGL渲染结果更新窗口
        window.gl_swap_window();
        for event in event_pump.poll_iter() {
            input::handle_sdl_input(&event);
            match event {
                Event::Quit { .. } => break 'running,
                Event::Window {
                    win_event: WindowEvent::Resized(w,h),
                    ..
                 } => {
                    viewport.update_size(w, h);
                    egui_input_state.input.screen_rect = Some(Rect::from_min_size(
                        Pos2::new(viewport.x as f32, viewport.y as f32),
                        vec2(viewport.w as f32, viewport.h as f32) / native_pixels_per_point,
                    ))
                 },
                _ => {
                    // 将捕捉的输入传递给egui
                    egui_backend::input_to_egui(event, &mut egui_input_state);
                }
            }
        }
        if input::get_key(Keycode::Escape, false){
            quit = true;
        }
        if quit { break; }
        if input::get_key_with_cooldown(Keycode::LCtrl, false,0.2) {
            input_enable = !input_enable;
            mouse.set_relative_mouse_mode(input_enable);
            if input_enable {
                square.camera.enable();
            } else {
                square.camera.disable();
            }
        }

        // let dur = std::time::Duration::from_millis(16);
        // std::thread::sleep(dur)
        //todo: soft-vsync
    }
    Ok(())
}
