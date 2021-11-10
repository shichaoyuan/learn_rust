use libc::{
    c_char, c_void, ftruncate, memcpy, memfd_create, mmap, munmap, off_t, size_t, MAP_SHARED,
    PROT_WRITE,
};
use sendfd::SendWithFd;
use std::{os::unix::net::UnixStream, ptr};

const STORAGE_ID: *const c_char = b"test-memfd\0".as_ptr() as *const c_char;
const STORAGE_SIZE: size_t = 128;
fn main() -> std::io::Result<()> {
    let stream = UnixStream::connect("./memfd.sock")?;

    let (fd, addr) = unsafe {
        let null = ptr::null_mut();
        let fd = memfd_create(STORAGE_ID, 0);
        let _res = ftruncate(fd, STORAGE_SIZE as off_t);
        let addr = mmap(null, STORAGE_SIZE, PROT_WRITE, MAP_SHARED, fd, 0);

        (fd, addr)
    };

    let data = b"Hello, World!\0";
    let pdata = data.as_ptr() as *const c_void;

    unsafe {
        memcpy(addr, pdata, data.len());
    }

    let fds = [fd];
    stream.send_with_fd("a".as_bytes(), &fds[..])?;

    println!("fds: {}", fds[0]);
    unsafe {
        munmap(addr, STORAGE_SIZE);
    }

    Ok(())
}
