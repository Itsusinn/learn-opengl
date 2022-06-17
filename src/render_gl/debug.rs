use core::panic;
use glow::HasContext;

use crate::GL;

pub fn check_error() {
  let code = unsafe { GL.get_error() };
  if code == 0 {
    return;
  }

  match code {
    glow::INVALID_ENUM => {
      panic!("枚举参数不合法")
    }
    glow::INVALID_VALUE => {
      panic!("值参数不合法")
    }
    glow::INVALID_OPERATION => {
      panic!("一个指令的状态对指令的参数不合法")
    }
    glow::STACK_OVERFLOW => {
      panic!("压栈操作造成栈上溢(Overflow)")
    }
    glow::STACK_UNDERFLOW => {
      panic!("弹栈操作时栈在最低点")
    }
    glow::OUT_OF_MEMORY => {
      panic!("内存溢出")
    }
    glow::INVALID_FRAMEBUFFER_OPERATION => {
      panic!("读取或写入一个不完整的帧缓冲")
    }
    _ => {
      panic!("未知的OpenGL错误,错误码：{}", &code)
    }
  }
}
