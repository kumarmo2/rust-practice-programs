use libc::{
    close, poll, pollfd, sigaddset, sigemptyset, signalfd, sigset_t, POLLIN, POLLPRI, SIGINT,
};
use std::mem::zeroed;
fn main() {
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
    let mut fds = vec![poll_fd];
    let fds = fds.as_mut_ptr();
    println!("will wait now");
    let poll_rs = unsafe { poll(fds, 1, -1) };
    println!("wait over, {poll_rs}");
    unsafe { close(fd) };
    let _ = unsafe { Box::from_raw(signal) };
}
