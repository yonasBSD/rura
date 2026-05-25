use anyhow::Result;
use log::error;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::thread::sleep;
use std::time::{Duration, SystemTime};

pub fn debouncer_task<F>(rx: Receiver<()>, duration: Duration, on_debounce: F) -> Result<()>
where
    F: Fn() -> () + Send + 'static,
{
    'outer: loop {
        if let Err(e) = rx.recv() {
            error!("Disconnected: {}", e);
            break 'outer Ok(());
        };

        let now = SystemTime::now();

        'debouncing: loop {
            sleep(Duration::from_millis(10));
            let elapsed = now.elapsed()?;

            match rx.try_recv() {
                Ok(_) => {
                    if elapsed > duration {
                        break;
                    }
                }

                Err(TryRecvError::Empty) => {
                    if elapsed > duration {
                        break 'debouncing;
                    }
                }

                Err(TryRecvError::Disconnected) => {
                    error!("Disconnected");
                    break 'outer Ok(());
                }
            }
        }

        on_debounce();
    }
}
