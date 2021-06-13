extern crate gl;
extern crate sdl2;
extern crate nalgebra as na;
#[macro_use] extern crate render_gl_derive;

use std::rc::Rc;
use crate::resources::Resources;
use std::path::Path;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use anyhow::anyhow;

mod triangle;
pub mod render_gl;
pub mod resources;



fn main() -> Result<(),anyhow::Error>{
   let res =
           Resources::from_relative_exe_path(Path::new("assets"))?;
   let sdl_context = sdl2::init()
      .map_err(|msg| anyhow!("Sdl2 初始化失败 {}",msg))?;
   let video_subsystem = sdl_context.video()
      .map_err(|msg| anyhow!("Video subsystem获取失败 {}", msg))?;

   let gl_attr = video_subsystem.gl_attr();
   gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
   gl_attr.set_context_version(4, 5);

   let window = video_subsystem
      .window("another", 800, 600)
      .opengl()
      .resizable()
      .position_centered()
      .build()?;

   let _gl_context = window.gl_create_context()
      .map_err(|msg| anyhow!("创建GL上下文失败: {}",msg))?;

   let gl: Rc<gl::Gl> = Rc::new(
      gl::Gl::load_with(|s| {
         video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
      })
   );

   let mut event_pump = sdl_context.event_pump()
      .map_err(|msg| anyhow!("事件轮询器获取失败: {}",msg))?;

   let mut viewport =
           render_gl::Viewport::for_window(900,700);
   viewport.set_used(&gl);

   let color_buffer =
           render_gl::ColorBuffer::from_color(na::Vector3::new(0.3, 0.3, 0.5));
   color_buffer.set_used(&gl);
   color_buffer.clear(&gl);

   unsafe {
      gl.Enable(gl::DEPTH_TEST);
      gl.Enable(gl::BLEND);
   }

   let triangle = triangle::Triangle::new(&res, &gl)?;

   'main: loop {
      for event in event_pump.poll_iter() {
         match event {
            Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'main,
            Event::Window {
               win_event: WindowEvent::Resized(w,h),
               ..
            } => {
               viewport.update_size(w, h);
               viewport.set_used(&gl);
            },
            _ => {}
         }
      }
      unsafe {
         gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
      }
      triangle.render(&gl);
      window.gl_swap_window()
   };
   Ok(())
}