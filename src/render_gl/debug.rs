use core::panic;

pub  fn check_error(gl:&gl::Gl){
    let code= unsafe{ gl.GetError() };
    if code == 0 {return;}

    match code{
        gl::INVALID_ENUM => {
            panic!("枚举参数不合法")
        }
        gl::INVALID_VALUE => {
            panic!("值参数不合法")
        }
        gl::INVALID_OPERATION => {
            panic!("一个指令的状态对指令的参数不合法")
        }
        gl::STACK_OVERFLOW => {
            panic!("压栈操作造成栈上溢(Overflow)")
        }
        gl::STACK_UNDERFLOW => {
            panic!("弹栈操作时栈在最低点")
        }
        gl::OUT_OF_MEMORY => {
            panic!("内存溢出")
        }
        gl::INVALID_FRAMEBUFFER_OPERATION => {
            panic!("读取或写入一个不完整的帧缓冲")
        }
        _ => {
            panic!("未知的OpenGL错误,错误码：{}",&code)
        }
    }

}