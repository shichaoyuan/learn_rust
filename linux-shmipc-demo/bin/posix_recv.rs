use libc::{
    c_char, close, ftruncate, mmap, munmap, off_t, shm_open, size_t, strncpy, MAP_SHARED, O_RDONLY,
    PROT_READ, S_IRUSR, S_IWUSR,
};

use std::{error::Error, ptr, str};

const STORAGE_ID: *const c_char = b"test-posix\0".as_ptr() as *const c_char;
const STORAGE_SIZE: size_t = 128;
fn main() -> Result<(), Box<dyn Error>> {
    let (fd, addr) = unsafe {
        let null = ptr::null_mut();
        let fd = shm_open(STORAGE_ID, O_RDONLY, S_IRUSR | S_IWUSR);
        let _res = ftruncate(fd, STORAGE_SIZE as off_t);
        let addr = mmap(null, STORAGE_SIZE, PROT_READ, MAP_SHARED, fd, 0);

        (fd, addr)
    };

    let mut data = [0_u8; STORAGE_SIZE];
    let pdata = data.as_mut_ptr() as *mut c_char;

    unsafe {
        strncpy(pdata, addr as *const c_char, STORAGE_SIZE);
    }
    println!("recv: {}", str::from_utf8(&data)?);

    unsafe {
        munmap(addr, STORAGE_SIZE);
        close(fd);
    }

    Ok(())
}
