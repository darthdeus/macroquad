use std::{cell::RefCell, rc::Rc, sync::{Arc, Mutex}};

use macroquad::{experimental::coroutines::start_coroutine, telemetry, window::next_frame};

#[macroquad::test]
async fn coroutine_value() {
    let mut coroutine = start_coroutine(async move {
        next_frame().await;
        1
    });

    coroutine.set_manual_poll();

    assert_eq!(coroutine.retrieve(), None);

    coroutine.poll(0.0);
    coroutine.poll(0.0);

    assert_eq!(coroutine.retrieve(), Some(1));
}

#[macroquad::test]
async fn coroutine_memory() {
    use macroquad::prelude::*;

    for _ in 0..20 {
        start_coroutine(async move {
            next_frame().await;
        });

        next_frame().await;
    }

    // wait for the last one to finish
    next_frame().await;

    assert_eq!(telemetry::active_coroutines_count(), 0);
}

#[macroquad::test]
async fn select_test() {
    use macroquad::prelude::coroutines::*;

    let val = Arc::new(Mutex::new(0));
    let val_inner = val.clone();

    let mut coroutine = start_coroutine(async move {
        futures::select! {
            _ = wait_seconds(0.5) => {
                *val_inner.lock().unwrap() = 1;
            }
            _ = next_frame() => {
                *val_inner.lock().unwrap() = 2;
            }
        }
    });

    coroutine.set_manual_poll();

    assert_eq!(*val.lock().unwrap(), 0);

    coroutine.poll(0.0);
    coroutine.poll(0.0);

    assert_eq!(*val.lock().unwrap(), 2);
}



