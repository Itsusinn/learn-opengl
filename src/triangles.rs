use std::ffi::c_void;

use crate::render_gl;
use crate::render_gl::debug::check_error;
use crate::render_gl::{buffer, data,texture};
use crate::resources::Resources;
#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::f32_f32_f32,
    #[location = 1]
    clr: data::u2_u10_u10_u10_rev_float,
    #[location = 2]
    tex: data::f32_f32
}

pub struct Square {
    program: render_gl::Program,
    _vbo: buffer::ArrayBuffer,
    _ebo: buffer::ElementArrayBuffer,
    vao: buffer::VertexArray,
    texture: texture::Texture
}

impl Square{
    pub fn new(
        res: &Resources,
        gl: &gl::Gl
    ) -> Result<Square, anyhow::Error> {

        let program = render_gl::Program::from_res(
            gl, res, "shaders/square"
        )?;

        let vertices: Vec<Vertex> = vec![
            Vertex {
                pos: ( 0.5, -0.5, 0.0).into(),
                clr: ( 1.0, 0.0, 0.0, 1.0).into(),
                tex: ( 1.0, 0.0 ).into()
            }, // bottom right
            Vertex {
                pos: (-0.5, -0.5, 0.0).into(),
                clr: (0.0, 1.0, 0.0, 1.0).into(),
                tex: ( 0.0, 0.0 ).into()
            }, // bottom left
            Vertex {
                pos: (-0.5,  0.5, 0.0).into(),
                clr: (0.0, 0.0, 1.0, 1.0).into(),
                tex: ( 0.0, 1.0 ).into()
            },  // top left
            Vertex {
                pos: (0.5,  0.5, 0.0).into(),
                clr: (0.0, 0.0, 1.0, 1.0).into(),
                tex: ( 1.0, 1.0 ).into()
            }  // top right
        ];
        let indices : Vec<u32> = vec![
            0,3,2,
            0,2,1
        ];

        let vbo = buffer::ArrayBuffer::new(gl);
        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();
        let ebo = buffer::ElementArrayBuffer::new(gl);
        ebo.bind();
        ebo.static_draw_data(&indices);
        ebo.unbind();
        let vao = buffer::VertexArray::new(gl);

        vao.bind();
        vbo.bind();
        ebo.bind();
        Vertex::vertex_attrib_pointers(gl);
        // 注意这里有一个自动绑定机制
        vao.unbind();
        let texture = texture::Texture::from_res(
            &gl, &res, "textures/wall.jpg"
        )?;
        Ok(Square {
            program,
            _vbo: vbo,
            _ebo: ebo,
            vao,
            texture
        })
    }

    pub fn render(&self, gl: &gl::Gl) {
        check_error(gl);
        self.program.set_used();
        self.vao.bind();
        unsafe {
            gl.BindTexture(gl::TEXTURE_2D, self.texture.id);
            gl.DrawElements(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_INT,
                0 as *const c_void
            )
        }
        self.vao.unbind();
        self.program.detach();
        check_error(gl);
    }
}