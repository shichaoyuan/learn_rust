use libc::{c_void, close, size_t};
use sendfd::RecvWithFd;
use std::os::unix::net::{UnixListener, UnixStream};

const STORAGE_SIZE: size_t = 128;
fn handle_client(stream: UnixStream) {
    let mut recv_bytes = [0; 128];
    let mut recv_fds = [0];

    stream.recv_with_fd(&mut recv_bytes, &mut recv_fds).unwrap();

    let fd = recv_fds[0];
    let mut data = [0_u8; STORAGE_SIZE];

    println!("recv fd: {}", fd);
    unsafe {
        let pdata = data.as_mut_ptr() as *mut c_void;
        libc::read(fd, pdata, STORAGE_SIZE);
    }
    println!("recv: {}", std::str::from_utf8(&data).unwrap());

    unsafe {
        close(fd);
    }
}

fn main() -> std::io::Result<()> {
    let listener = UnixListener::bind("./memfd.sock")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(_err) => {
                break;
            }
        }
    }

    Ok(())
}
