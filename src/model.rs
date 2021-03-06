use russimp::scene::{Scene, PostProcess};

#[test]
fn test_load_model() {
   let scene = Scene::from_file(
      "assets/models/blend/box.blend",
      vec![
         PostProcess::CalculateTangentSpace,
         PostProcess::Triangulate,
         PostProcess::JoinIdenticalVertices,
         PostProcess::SortByPrimitiveType,
      ],
   );
   assert!(scene.is_ok());
}

use crate::render_gl;
use crate::render_gl::{buffer, data};
use crate::resources::Resources;

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
   #[location = 0]
   pos: data::f32_f32_f32,
   #[location = 1]
   clr: data::f32_f32_f32,
   #[location = 2]
   normal: data::f32_f32_f32,
}

pub struct Box {
   program: render_gl::Program,
   _vbo: buffer::ArrayBuffer,
   vao: buffer::VertexArray,
}

impl Box {
   pub fn new(
      res: &Resources,
      gl: &gl::Gl
   ) -> Result<Box, anyhow::Error> {

      let program = render_gl::Program::from_res(
         gl, res, "shaders/box"
      )?;

      let scene = Scene::from_file(
         "assets/models/blend/box.blend",
         vec![
            PostProcess::CalculateTangentSpace,
            PostProcess::Triangulate,
            PostProcess::JoinIdenticalVertices,
            PostProcess::SortByPrimitiveType,
         ],
      ).unwrap();

      let vertices: Vec<Vertex> = vec![

      ];

      let vbo = buffer::ArrayBuffer::new(gl);
      vbo.bind();
      vbo.static_draw_data(&vertices);
      vbo.unbind();

      let vao = buffer::VertexArray::new(gl);

      vao.bind();
      vbo.bind();
      Vertex::vertex_attrib_pointers(gl);
      vbo.unbind();
      vao.unbind();

      Ok(Box {
         program,
         _vbo: vbo,
         vao,
      })
   }

   pub fn render(&self, gl: &gl::Gl) {
      self.program.set_used();
      self.vao.bind();

      unsafe {
         gl.DrawArrays(
            gl::TRIANGLES, 0, 3
         );
      }
   }
}