use anyhow::Context as _;
use extendr_api::prelude::*;
use named_pipe::PipeClient;
use std::io::Write;
use std::os::windows::ffi::OsStrExt;

#[extendr]
fn named_pipe(path:&str, message:&str) {
  let path = format!(r##"\\.\pipe\{path}"##);
  let mut pipe = PipeClient::connect(path).unwrap();

  let message_bytes = message.as_bytes();
  pipe.write_all(message_bytes).unwrap();
}

#[extendr]
fn memory_mapped_file(path:&str) {
  let wide_name: Vec<u16> = std::ffi::OsStr::new(path)
    .encode_wide()
    .chain(std::iter::once(0))
    .collect();
  let handle = unsafe { kernel32::OpenFileMappingW(
    winapi::um::winnt::SECTION_MAP_WRITE,
    winapi::shared::minwindef::FALSE,
    wide_name.as_ptr()
  ) };
  if handle.is_null() { panic!("panic"); }

  let view = unsafe { kernel32::MapViewOfFile(
    handle,
    winapi::um::winnt::SECTION_MAP_WRITE,
    0, 0, 0,
  ) };

  if view.is_null() {
    unsafe { kernel32::CloseHandle(handle) };
    // anyhow::bail!("mmf err");
    panic!("panic");
  }
  let memory_slice = unsafe {
    std::slice::from_raw_parts_mut(view as *mut i32, 640 * 480 + 1)
  };
  let a = memory_slice[0];
  let b = memory_slice[1];
  let c = memory_slice[2];
  println!("{a} {b} {c}");
  
  unsafe {
    kernel32::UnmapViewOfFile(view);
    kernel32::CloseHandle(handle);
  };
}

extendr_module! {
  mod rsquid;
  fn named_pipe;
  fn memory_mapped_file;
}
