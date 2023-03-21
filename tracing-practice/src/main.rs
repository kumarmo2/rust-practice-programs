use std::{thread, time::Duration};

use tracing::{info, span, subscriber::SetGlobalDefaultError, Level};
// use traci

thread_local! {
    static y: i32 = 456;
}

fn main() -> Result<(), SetGlobalDefaultError> {
    println!("Hello, world!");
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_target(false)
        .with_line_number(true)
        .with_thread_ids(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    let main_span = span!(Level::INFO, "main_span", level = 1);

    let _enter = main_span.enter();

    let handles: Vec<_> = (0..3)
        .map(|_| {
            let main_span = main_span.clone();
            thread::spawn(move || {
                let inner_span = span!(parent: &main_span, Level::INFO, "inner span");
                let _enter = inner_span.enter();
                info!("before sleep");
                thread::sleep(Duration::from_millis(2000));
                info!("after sleep");
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
