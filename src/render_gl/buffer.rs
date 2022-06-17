use std::fmt::Debug;

use glow::HasContext;

use crate::GL;

pub trait BufferType {
  const BUFFER_TYPE: u32;
}

pub struct Buffer<B>
where
  B: BufferType,
{
  inner: glow::Buffer,
  _marker: std::marker::PhantomData<B>,
}

impl<B> Buffer<B>
where
  B: BufferType,
{
  pub fn new() -> Buffer<B> {
    let inner = unsafe { GL.create_buffer().unwrap() };

    Buffer {
      inner,
      _marker: ::std::marker::PhantomData,
    }
  }

  pub fn bind(&self) {
    unsafe {
      GL.bind_buffer(B::BUFFER_TYPE, Some(self.inner));
    }
  }

  pub fn unbind(&self) {
    unsafe {
      GL.bind_buffer(B::BUFFER_TYPE, None);
    }
  }

  pub fn static_draw_data<T>(&self, data: &[T])
  where
    T: Debug,
  {
    unsafe {
      let data = another::any_as_u8_slice(data);
      GL.buffer_data_u8_slice(B::BUFFER_TYPE, data, glow::STATIC_DRAW);
    }
  }
}
impl<B> Drop for Buffer<B>
where
  B: BufferType,
{
  fn drop(&mut self) {
    unsafe {
      GL.delete_buffer(self.inner);
    }
  }
}

//
//********Vertex Array Buffer Object
//
pub type ArrayBuffer = Buffer<BufferTypeArray>;
pub struct BufferTypeArray;
impl BufferType for BufferTypeArray {
  const BUFFER_TYPE: u32 = glow::ARRAY_BUFFER;
}

//
//********Element Array Buffer Object
//
pub type ElementArrayBuffer = Buffer<BufferTypeElementArray>;
pub struct BufferTypeElementArray;
impl BufferType for BufferTypeElementArray {
  const BUFFER_TYPE: u32 = glow::ELEMENT_ARRAY_BUFFER;
}

//
//********Vertex Array Object
//
pub struct VertexArray {
  vao: glow::VertexArray,
}
impl VertexArray {
  pub fn new() -> VertexArray {
    let vao = unsafe { GL.create_vertex_array().unwrap() };

    VertexArray { vao }
  }

  pub fn bind(&self) {
    unsafe {
      GL.bind_vertex_array(Some(self.vao));
    }
  }

  pub fn unbind(&self) {
    unsafe {
      GL.bind_vertex_array(None);
    }
  }
}

impl Drop for VertexArray {
  fn drop(&mut self) {
    unsafe {
      GL.delete_vertex_array(self.vao);
    }
  }
}
