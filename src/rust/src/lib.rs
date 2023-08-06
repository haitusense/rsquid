use anyhow::Context as _;
use extendr_api::prelude::*;
use named_pipe::PipeClient;
use std::io::Write;
use std::os::windows::ffi::OsStrExt;

#[allow(non_snake_case)]
#[extendr]
fn namedPipe(path: &str, message: Robj) {
  let path = format!(r##"\\.\pipe\{path}"##);
  let mut pipe = PipeClient::connect(path).unwrap();

  // Option<&'a str>
  if let Some(n) = message.as_str() {
    pipe.write_all(n.as_bytes()).unwrap();
  }

  // Rtype::List
  if message.rtype() == Rtype::List {
    let robj = R!("jsonlite::toJSON({{ message }})").unwrap();
    let json = robj.as_str().unwrap();
    pipe.write_all(json.as_bytes()).unwrap();
  }

}

struct MemoryMappedFile {
  handle : *mut std::ffi::c_void,
  view : *mut std::ffi::c_void,
}

impl MemoryMappedFile {

  fn open(path:&str) -> anyhow::Result<Self> {
    let wide_name: Vec<u16> = std::ffi::OsStr::new(path)
      .encode_wide()
      .chain(std::iter::once(0))
      .collect();
    let handle = unsafe { kernel32::OpenFileMappingW(
      winapi::um::winnt::SECTION_MAP_WRITE,
      winapi::shared::minwindef::FALSE,
      wide_name.as_ptr()
    ) };
  
    if handle.is_null() { anyhow::bail!("open mmf err") }

    let view = unsafe { kernel32::MapViewOfFile(
      handle,
      winapi::um::winnt::SECTION_MAP_WRITE,
      0, 0, 0,
    ) };

    if view.is_null() {
      unsafe { kernel32::CloseHandle(handle) };
      anyhow::bail!("open mmf err");
    }
    
    Ok(Self { handle, view })
  }

  fn close(&self) {
    unsafe {
      kernel32::UnmapViewOfFile(self.view);
      kernel32::CloseHandle(self.handle);
    };
  }

  fn to_vec(&self) -> Vec<i32> {
    let memory_size = unsafe {
      std::slice::from_raw_parts_mut(self.view as *mut i32, 1)[0] as usize
    };
    println!("memorysize : {memory_size}");

    let memory_slice = unsafe {
      &std::slice::from_raw_parts_mut(self.view as *mut i32, memory_size + 1)[1..=memory_size]
    };
    memory_slice.to_vec()
  }

  fn to_vector<T>(&self) -> Vec<T> where T: Clone {
    let memory_size = unsafe {
      std::slice::from_raw_parts_mut(self.view as *mut i32, 1)[0] as usize
    };
    println!("memorysize : {memory_size}");

    let memory_slice = unsafe {
      std::slice::from_raw_parts_mut(self.view.offset(4) as *mut T, memory_size)
    };
    memory_slice.to_vec()
  }

  fn from_vec(&self, src: Vec<i32>) {
    let memory_slice = unsafe {
      std::slice::from_raw_parts_mut(self.view as *mut i32, src.len() + 1)
    };
    memory_slice[0] = src.len() as i32;
    for i in 0..src.len() {
      memory_slice[i+1] = src[i];
    }
  }

  fn from_vector<T>(&self, src: Vec<T>) where T: Clone + Copy {
    let memory_size = unsafe {
      std::slice::from_raw_parts_mut(self.view as *mut i32, 1)
    };
    memory_size[0] = src.len() as i32;
    let memory_slice = unsafe {
      std::slice::from_raw_parts_mut(self.view.offset(4) as *mut T, src.len())
    };

    for i in 0..src.len() {
      memory_slice[i] = src[i];
    }
  }

}

#[allow(non_snake_case)]
#[extendr]
fn readMemoryMappedFile(path:&str) -> Vec<i32> {
  let mmf = MemoryMappedFile::open(path).unwrap();
  let dst = mmf.to_vec();
  mmf.close();
  dst
}

#[allow(non_snake_case)]
#[extendr]
fn readMemoryMappedFileFloat(path:&str) -> Vec<f64> {
  let mmf = MemoryMappedFile::open(path).unwrap();
  let dst = mmf.to_vector::<f64>();
  mmf.close();
  dst
}

#[allow(non_snake_case)]
#[extendr]
fn writeMemoryMappedFile(path:&str, src:Vec<i32>) {
  let mmf = MemoryMappedFile::open(path).unwrap();
  mmf.from_vec(src);
  mmf.close();
}

#[allow(non_snake_case)]
#[extendr]
fn writeMemoryMappedFileFloat(path:&str, src:Vec<f64>) {
  let mmf = MemoryMappedFile::open(path).unwrap();
  mmf.from_vector::<f64>(src);
  mmf.close();
}

#[allow(dead_code)]
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

  let memory_size = unsafe {
    std::slice::from_raw_parts_mut(view as *mut i32, 1)[0] as usize
  };
  println!("memorysize : {memory_size}");

  let memory_slice = unsafe {
    &std::slice::from_raw_parts_mut(view as *mut i32, memory_size + 1)[1..memory_size]
  };
  for i in 0..memory_slice.len() {
    let val = memory_slice[i];
    println!("index {i} : {val}");
  }

  unsafe {
    kernel32::UnmapViewOfFile(view);
    kernel32::CloseHandle(handle);
  };
}


#[extendr]
fn ggsave2svg(val: Robj) -> Robj {
  R!("
    local({
      tmp <- tempfile(pattern = 'tmp', tmpdir = tempdir(), fileext = '.svg')
      args <- {{ val }}
      args$file <- tmp
      do.call(ggplot2::ggsave, args)
      svg <- XML::xmlRoot(XML::xmlInternalTreeParse(tmp))
      file.remove(tmp)
      svg
    })
  ").unwrap()
}

#[allow(non_snake_case)]
#[extendr]
fn asImageDataFrame(src: Robj, width: i32, height: i32) -> Robj {
  R!("
    local({
      x <- rep(1:{{ width }}, length.out = {{ width }} * {{ height }})
      y <- rep(1:{{ height }}, each = {{ width }})
      data.frame(x = x, y = y, value = {{ src }})
    })
  ").unwrap()
}

extendr_module! {
  mod rsquid;
  fn namedPipe;
  fn readMemoryMappedFile;
  fn readMemoryMappedFileFloat;
  fn writeMemoryMappedFile;
  fn writeMemoryMappedFileFloat;
  fn ggsave2svg;
  fn asImageDataFrame;
}
