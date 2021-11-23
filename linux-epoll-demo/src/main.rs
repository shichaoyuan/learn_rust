use std::io;
use std::io::prelude::*;

mod epoll_syscall;

use epoll_syscall::{add_interest, epoll_create, epoll_wait};

fn main() {

    let mut events: Vec<libc::epoll_event> = Vec::with_capacity(1024);

    let pfd = unsafe {
        libc::open(b"/home/ubuntu/learn_rust/linux-epoll-demo/testpipe\0".as_ptr() as *const libc::c_char, libc::O_RDONLY | libc::O_NONBLOCK)
    };
    println!("pipe: {}", pfd);
    println!("{}", std::io::Error::last_os_error());


    let evt = libc::epoll_event {
        events: (libc::EPOLLIN | libc::EPOLLET) as u32,
        u64: 100,
    };

    let epoll_fd = epoll_create().expect("can create epoll queue");
    add_interest(epoll_fd, pfd, evt);

    let mut data = [0_u8; 128];
    let pdata = data.as_mut_ptr() as *mut libc::c_void;

    loop {
        println!("start loop ...");

        events.clear();

        let res = match epoll_wait(
            epoll_fd,
            events.as_mut_ptr() as *mut libc::epoll_event,
        ) {
            Ok(v) => v,
            Err(e) => panic!("error during epoll wait: {}", e),
        };

        unsafe { events.set_len(res as usize) };

        for ev in &events {
            match ev.u64 {
                100 => {
                    println!("evt: {}", ev.events);
                    //unsafe {
                    //    libc::read(pfd, pdata, 128);
                    //    println!("data: {}", std::str::from_utf8_unchecked(&data));
                    //}
                }
                _ => {
                    println!("unknown");
                }
            }
        }
    }
}
