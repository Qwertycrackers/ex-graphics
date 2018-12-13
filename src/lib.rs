extern crate sdl2;
extern crate gl;
use std::ffi::{CString, CStr};

mod render_gl;

type GLuint = gl::types::GLuint;

pub fn run() {
  let sdl = sdl2::init().unwrap();
  let video_system = sdl.video().unwrap();
  let gl_attr = video_system.gl_attr();

  gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
  gl_attr.set_context_version(3, 3);

  let window = video_system
    .window("Game", 900, 700)
    .resizable()
    .opengl() // add opengl flag on window
    .build()
    .unwrap();

  let mut event_pump = sdl.event_pump().unwrap();
  let _gl_context = window.gl_create_context().unwrap();
  let _gl = gl::load_with(
    |s| video_system.gl_get_proc_address(s) as *const std::os::raw::c_void
  );
  
  // Shader Time
  let vertex = render_gl::Shader::from_source_vert(
    &CString::new(include_str!("triangle.vert")).unwrap()
  ).unwrap();
  let fragment = render_gl::Shader::from_source_frag(
    &CString::new(include_str!("triangle.frag")).unwrap()
  ).unwrap();
  let program = render_gl::Program::from_shaders(
    &[vertex, fragment]
  ).unwrap();
  program.set_used();

  // Our one triangle
  let vertices: Vec<f32> = vec![
    -0.5, -0.5, 0.0,
    0.5, -0.5, 0.0,
    0.0, 0.0, 0.5,
  ];
  // Gimme a buffer
  let mut vbo: GLuint = 0;
  unsafe { 
    gl::GenBuffers(1, &mut vbo); 
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
         gl::ARRAY_BUFFER,
         (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
         vertices.as_ptr() as *const gl::types::GLvoid,
         gl::STATIC_DRAW,
    );
    gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind
  }
  let mut vao: GLuint = 0;
  unsafe {
    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(
      0,
      3,
      gl::FLOAT,
      gl::FALSE,
      3 * (std::mem::size_of::<f32>()) as gl::types::GLint,
      std::ptr::null(),
    );
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    gl::BindVertexArray(0);
  }



  unsafe {
    gl::ClearColor(0.3, 0.3, 0.5, 1.0); // Set clearcolor to blue
  }

  'main: loop {
    for event in event_pump.poll_iter() {
      match event {
        sdl2::event::Event::Quit {..} => break 'main,
        _ => (),
      }
    }
    // render down here
    unsafe {
      gl::Clear(gl::COLOR_BUFFER_BIT);
      gl::BindVertexArray(vao);
      gl::DrawArrays(
        gl::TRIANGLES,
        0,
        3,
      );
    }

    window.gl_swap_window();
  }
}

