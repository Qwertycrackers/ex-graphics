extern crate sdl2;
extern crate gl;
use std::ffi::{CString, CStr};

mod render_gl;

type GLuint = gl::types::GLuint;

pub fn run() {
  let sdl = sdl2::init().unwrap();
  let video_system = sdl.video().unwrap();
  let window = video_system
    .window("Game", 900, 700)
    .resizable()
    .opengl() // add opengl flag on window
    .build()
    .unwrap();
  let mut event_pump = sdl.event_pump().unwrap();
  let mut gl_context = window.gl_create_context().unwrap();
  let mut gl = gl::load_with(
    |s| video_system.gl_get_proc_address(s) as *const std::os::raw::c_void
  );
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
    }
    window.gl_swap_window();
  }
}

