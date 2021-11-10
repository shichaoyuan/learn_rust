use libc::{
    c_char, c_void, close, ftruncate, memcpy, mmap, munmap, off_t, shm_open, size_t, MAP_SHARED,
    O_CREAT, O_RDWR, PROT_WRITE, S_IRUSR, S_IWUSR,
};

use std::{error::Error, ptr, str};

const STORAGE_ID: *const c_char = b"test-posix\0".as_ptr() as *const c_char;
const STORAGE_SIZE: size_t = 128;
fn main() -> Result<(), Box<dyn Error>> {
    let (fd, addr) = unsafe {
        let null = ptr::null_mut();
        let fd = shm_open(STORAGE_ID, O_RDWR | O_CREAT, S_IRUSR | S_IWUSR);
        let _res = ftruncate(fd, STORAGE_SIZE as off_t);
        let addr = mmap(null, STORAGE_SIZE, PROT_WRITE, MAP_SHARED, fd, 0);

        (fd, addr)
    };

    let data = b"Hello, World!\0";
    let pdata = data.as_ptr() as *const c_void;

    unsafe {
        memcpy(addr, pdata, data.len());
    }

    unsafe {
        munmap(addr, STORAGE_SIZE);
        close(fd);
    }

    println!("send: {}", str::from_utf8(data)?);

    Ok(())
}
