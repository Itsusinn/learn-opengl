use crate::render_gl::debug;

pub struct FrameBuffer {
    gl: gl::Gl,
    pub id:u32,
    pub weight:i32,
    pub height:i32,
    pub texture_id:u32
}
impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteFramebuffers(1, &mut self.id);
        }
    }
}
impl FrameBuffer {
    pub fn new(
        gl: &gl::Gl,
        weight:i32,
        height:i32
    ) -> Self {
        let mut fbo_id: u32 = 0;
        unsafe {
            gl.GenFramebuffers(1, &mut fbo_id);
            gl.BindFramebuffer(gl::FRAMEBUFFER, fbo_id);
        }
        let mut texture_id:u32 = 0;
        // 生成空白纹理并attach到FBO上
        unsafe {
            gl.GenTextures(1, &mut texture_id);
            gl.BindTexture(gl::TEXTURE_2D, texture_id);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl.TexImage2D(
                gl::TEXTURE_2D, 0,
                gl::RGB as i32, weight, height, 0,
                gl::RGB, gl::UNSIGNED_BYTE, std::ptr::null());
            gl.BindTexture(gl::TEXTURE_2D, 0);
            gl.FramebufferTexture2D(
                gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,  texture_id, 0
            );
        }
        let mut rbo_id:u32 =0;
        // 生成render buffer以缓冲深度和模板信息
        unsafe {
            gl.GenRenderbuffers(1,&mut rbo_id);
            gl.BindRenderbuffer(gl::RENDERBUFFER,rbo_id);
            // 通过glRenderbufferStorage API给RBO创建、初始化存储空间
            gl.RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT32, weight, height);
            // glFramebufferRenderbuffer API 将指定的RBO关联到GPU当前的FBO上。
            gl.FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                gl::DEPTH_ATTACHMENT,
                gl::RENDERBUFFER,
                rbo_id);
            if gl.CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                println!("帧缓冲创建失败");
                debug::check_error(gl);
            }
            debug::check_error(gl);
            gl.BindRenderbuffer(gl::RENDERBUFFER, 0);
            gl.BindFramebuffer(gl::FRAMEBUFFER, 0);
            debug::check_error(gl);
        }
        Self {
            gl: gl.clone(),
            id: fbo_id,
            weight,
            height,
            texture_id
        }
    }

    pub fn bind(&self){
        unsafe  {
            self.gl.BindFramebuffer(gl::FRAMEBUFFER,self.id);
        }
    }
    pub fn detach(&self) {
        unsafe  {
            self.gl.BindFramebuffer(gl::FRAMEBUFFER,0);
        }
    }

}

