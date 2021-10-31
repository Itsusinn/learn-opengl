use gl::types::*;

pub trait BufferType {
  const BUFFER_TYPE: GLuint;
}

pub struct Buffer<B>
where
  B: BufferType,
{
  gl: gl::Gl,
  id: GLuint,
  _marker: std::marker::PhantomData<B>,
}

impl<B> Buffer<B>
where
  B: BufferType,
{
  pub fn new(gl: &gl::Gl) -> Buffer<B> {
    let mut id = 0u32;
    unsafe {
      gl.GenBuffers(1, &mut id);
    }

    Buffer {
      gl: gl.clone(),
      id,
      _marker: ::std::marker::PhantomData,
    }
  }

  pub fn bind(&self) {
    unsafe {
      self.gl.BindBuffer(B::BUFFER_TYPE, self.id);
    }
  }

  pub fn unbind(&self) {
    unsafe {
      self.gl.BindBuffer(B::BUFFER_TYPE, 0);
    }
  }

  pub fn static_draw_data<T>(&self, data: &[T]) {
    unsafe {
      self.gl.BufferData(
        B::BUFFER_TYPE,                                                   // target
        (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr, // size of data in bytes
        data.as_ptr() as *const gl::types::GLvoid,                        // pointer to data
        gl::STATIC_DRAW,                                                  // usage
      );
    }
  }
}
impl<B> Drop for Buffer<B>
where
  B: BufferType,
{
  fn drop(&mut self) {
    unsafe {
      self.gl.DeleteBuffers(1, &mut self.id);
    }
  }
}

//
//********Vertex Array Buffer Object
//
pub type ArrayBuffer = Buffer<BufferTypeArray>;
pub struct BufferTypeArray;
impl BufferType for BufferTypeArray {
  const BUFFER_TYPE: GLuint = gl::ARRAY_BUFFER;
}

//
//********Element Array Buffer Object
//
pub type ElementArrayBuffer = Buffer<BufferTypeElementArray>;
pub struct BufferTypeElementArray;
impl BufferType for BufferTypeElementArray {
  const BUFFER_TYPE: GLuint = gl::ELEMENT_ARRAY_BUFFER;
}

//
//********Vertex Array Object
//
pub struct VertexArray {
  gl: gl::Gl,
  vao: gl::types::GLuint,
}
impl VertexArray {
  pub fn new(gl: &gl::Gl) -> VertexArray {
    let mut vao: gl::types::GLuint = 0;
    unsafe {
      gl.GenVertexArrays(1, &mut vao);
    }

    VertexArray {
      gl: gl.clone(),
      vao,
    }
  }

  pub fn bind(&self) {
    unsafe {
      self.gl.BindVertexArray(self.vao);
    }
  }

  pub fn unbind(&self) {
    unsafe {
      self.gl.BindVertexArray(0);
    }
  }
}

impl Drop for VertexArray {
  fn drop(&mut self) {
    unsafe {
      self.gl.DeleteVertexArrays(1, &mut self.vao);
    }
  }
}
