use libc::{
    close, epoll_create, pollfd, sigaddset, sigemptyset, signalfd, sigset_t, POLLIN, POLLPRI,
    SIGINT,
};
use std::mem::zeroed;
fn main() {
    println!("Hello, world!");
    // let mut signals: [u32; 32] = [0; 32];
    let signal: sigset_t = unsafe { zeroed() };
    let signal = Box::into_raw(Box::new(signal));
    let result = unsafe { sigemptyset(signal) };
    if result == 0 {
        println!("signal was set to empty successfull");
    } else {
        println!("unsuccessfull while setting the signal");
        let _ = unsafe { Box::from_raw(signal) };
        return;
    }

    if let 0 = unsafe { sigaddset(signal, SIGINT) } {
        println!("successfull added sigint");
    } else {
        eprintln!("error while adding signint");
        let _ = unsafe { Box::from_raw(signal) };
        return;
    }
    let fd = unsafe { signalfd(-1, signal, 0) };
    if let -1 = fd {
        eprintln!("error while creating fd");
        let _ = unsafe { Box::from_raw(signal) };
        return;
    } else {
        println!("created the fd successfully")
    }
    let poll_fd = pollfd {
        fd: fd,
        events: POLLIN | POLLPRI,
        revents: POLLIN | POLLPRI,
    };
    let fds = vec![poll_fd];
    let fds = fds.as_ptr();

    unsafe { close(fd) };
    let _ = unsafe { Box::from_raw(signal) };

    // let x = signals.as_mut_ptr();
    // let x = signalfd(-1, x, flags);
}
