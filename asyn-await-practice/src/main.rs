#![allow(dead_code)]
use std::future::Future;

use tokio::sync::mpsc::{Receiver, Sender};

async fn formard<T>(mut rx: Receiver<T>, tx: Sender<T>) {
    while let Some(t) = rx.recv().await {
        let _ = tx.send(t).await;
    }
}

enum Forward<T> {
    NotStarted { recv: ReceiveFuture<T>, tx: Sender<T> },
    DoneReceive{send_fut: SendFuture<T>, tx: Sender<T>, msg: T},
}

impl<T> Future for Forward<T> {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match &*self {
            Forward::NotStarted { recv, tx } => {
                if let Some(Poll::Ready(msg)) = recv.poll() {
                    if Some(msg) = msg {

                      *self = Forward::DoneReceive { tx.send(), tx, msg};
                     return self.poll();
                    } else {
                    return Poll:Ready(());
                }
            
            } else {
                    return Poll::Pending

        }
            Forward::DoneReceive(rx, tx, msg) => {
                if let Some(tx.send(msg).poll())




            },
        }
    }
}

fn forward_without_async<T>(mut rx: Receiver<T>, tx: Sender<T>) -> Forward<T> {
    Forward::NotStarted { recv: rx.recv(), tx }
}

fn main() {
    println!("Hello, world!");
}
