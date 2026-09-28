use std::ffi::CString;

pub enum ShaderType{
    Vertex,
    Fragment,
}

//compile the shader into shader binary (IG ThAtS WhAT Its CAlLed)
pub fn compile_shader(shader_source: &str, shader_type: ShaderType) -> Result<u32, String>{
    let shader = unsafe{ gl::CreateShader(match shader_type{
        ShaderType::Vertex => gl::VERTEX_SHADER,
        ShaderType::Fragment => gl::FRAGMENT_SHADER,
    }) };

    unsafe{ 
        gl::ShaderSource(
            shader,
            1,
            &CString::new(shader_source.to_string()).unwrap().as_ptr(),
            std::ptr::null()
        );

        gl::CompileShader(shader);
    }

    let mut success = 0;
    let mut len = 0;

    unsafe { gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success) };

    if success == 0{
        unsafe { gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len) };

        let mut buffer = vec![0u8; len as usize];
        
        unsafe{
            gl::GetShaderInfoLog(
                shader,
                len,
                std::ptr::null_mut(),
                buffer.as_mut_ptr() as *mut i8,
            );
        }
        return Err(String::from_utf8_lossy(&buffer).to_string());
    }
    Ok(shader)
}

//link 2 shader into 1 program
pub fn link_shader(vertex: u32, fragment: u32) -> Result<u32, String>{
    let program = unsafe { gl::CreateProgram() };
    
    unsafe{
        gl::AttachShader(program, vertex);
        gl::AttachShader(program, fragment);
        gl::LinkProgram(program);
    }

    let mut success = 0;
    let mut len = 0;
    unsafe{ gl::GetProgramiv(program, gl::LINK_STATUS, &mut success) };

    if success == 0{
        unsafe { gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len) };

        let mut buffer = vec![0u8; len as usize];
        
        unsafe{
            gl::GetProgramInfoLog(
                program, 
                len,
                std::ptr::null_mut(),
                buffer.as_mut_ptr() as * mut i8,
            );
        }
        return Err(String::from_utf8_lossy(&buffer).to_string());
    }
    Ok(program)
}


