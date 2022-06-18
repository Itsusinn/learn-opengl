pub mod ui;

use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct LateInit<T> {
  cell: OnceCell<T>,
}

impl<T> LateInit<T> {
  pub fn init(&self, value: T) {
    assert!(self.cell.set(value).is_ok())
  }
  pub const fn new() -> LateInit<T> {
    LateInit {
      cell: OnceCell::new(),
    }
  }
}

impl<T> Default for LateInit<T> {
  fn default() -> Self {
    LateInit::new()
  }
}

impl<T> std::ops::Deref for LateInit<T> {
  type Target = T;
  fn deref(&self) -> &T {
    // hahaha......
    unsafe { self.cell.get_unchecked() }
  }
}

pub unsafe fn any_as_u8_slice<T: Sized>(p: &[T]) -> &[u8] {
  std::slice::from_raw_parts(
    (p as *const [T]) as *const u8,
    std::mem::size_of::<T>() * p.len(),
  )
}
