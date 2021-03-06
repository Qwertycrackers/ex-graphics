use std::ffi::{CString, CStr};

type GLuint = gl::types::GLuint;

fn compile_shader(source: &CStr, kind: GLuint) -> Result<GLuint, String> {
  let id = unsafe { gl::CreateShader(kind) };
  unsafe {
    gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
    gl::CompileShader(id);
  }
  let mut success: gl::types::GLint = 1;
  unsafe {
    gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
  }
  if success == 0 { // failure
    let mut len: gl::types::GLint = 0;
    unsafe { gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len); }
    // allocate buffer and fill it with spaces
    let error = create_whitespace_cstring(len as usize);
    unsafe {
      gl::GetShaderInfoLog(
        id,
        len,
        std::ptr::null_mut(),
        error.as_ptr() as *mut gl::types::GLchar
      );
    }
    return Err(error.to_string_lossy().into_owned());
  }
  Ok(id)
}

pub struct Shader {
  id: gl::types::GLuint,
}

impl Shader {
  pub fn id(&self) -> GLuint { self.id }
  pub fn from_source(source: &CStr, kind: gl::types::GLuint) -> Result<Self, String> {
    let id = compile_shader(source, kind)?;
    Ok(Shader { id })
  }

  pub fn from_source_vert(source: &CStr) -> Result<Self, String> {
    Shader::from_source(source, gl::VERTEX_SHADER)
  }

  pub fn from_source_frag(source: &CStr) -> Result<Self, String> {
    Shader::from_source(source, gl::FRAGMENT_SHADER)
  }
}

impl Drop for Shader {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteShader(self.id);
    }
  }
}

fn create_whitespace_cstring(len: usize) -> CString {
  let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
  buffer.extend([b' '].iter().cycle().take(len));
  unsafe { CString::from_vec_unchecked(buffer) }
}

pub struct Program {
  id: GLuint
}

impl Program {
  pub fn from_shaders(shaders: &[Shader]) -> Result<Self, String> {
    let program: GLuint = unsafe { gl::CreateProgram() };
    for shader in shaders {
      unsafe { gl::AttachShader(program, shader.id()) };
    }
    unsafe { gl::LinkProgram(program) }
    // Handle goddamn errors
  let mut success: gl::types::GLint = 1;
  unsafe {
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
  }
  if success == 0 { // failure
    let mut len: gl::types::GLint = 0;
    unsafe { gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len); }
    // allocate buffer and fill it with spaces
    let error = create_whitespace_cstring(len as usize);
    unsafe {
      gl::GetProgramInfoLog(
        program,
        len,
        std::ptr::null_mut(),
        error.as_ptr() as *mut gl::types::GLchar
      );
    }
    return Err(error.to_string_lossy().into_owned());
  }

    for shader in shaders {
      unsafe { gl::DetachShader(program, shader.id()) };
    }
    Ok(Program { id: program })
  }

  pub fn id(&self) -> GLuint {
    self.id
  }

  pub fn set_used(&self) {
    unsafe { gl::UseProgram(self.id()) }
  }

}



