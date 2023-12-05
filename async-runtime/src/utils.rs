pub fn err_handle(err: libc::c_int) -> std::io::Result<libc::c_int> {
  if err == -1 {
    return Err(std::io::Error::last_os_error());
  }
  Ok(err)
}