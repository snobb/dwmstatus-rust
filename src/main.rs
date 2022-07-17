extern crate chrono;

mod alsa;
mod x11;

use chrono::prelude::Local;
use std::{fs, io, thread, time};

const LA_PATH: &str = "/proc/loadavg";
const LINK_PATH: &str = env!("LINK_PATH");
const PROGRESS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

fn load_average() -> io::Result<Vec<f64>> {
    Ok(fs::read_to_string(LA_PATH)?
        .split_whitespace()
        .take(3)
        .map(|s| s.parse::<f64>().unwrap_or(-1f64))
        .collect())
}

fn volume() -> char {
    let vol = alsa::volume();
    if vol < 0 {
        return 'M';
    }

    PROGRESS[(vol as usize * PROGRESS.len()) / 100]
}

fn wifi() -> io::Result<String> {
    Ok(fs::read_to_string(LINK_PATH)?.trim_end().to_string())
}

fn main() {
    let mut display = x11::Display::new();

    loop {
        let time = Local::now();

        let la = match load_average() {
            Ok(res) => format!("{:.2} {:.2} {:.2}", res[0], res[1], res[2]),
            Err(_) => "? ? ?".to_string(),
        };

        let wifi = match wifi() {
            Ok(status) => status,
            Err(_) => "err".to_string(),
        };

        display.set_root_title(format!(
            "{} | vol:{} | wifi:{} | {}",
            la,
            volume(),
            wifi,
            time.format("%a %b %e %T")
        ));

        thread::sleep(time::Duration::from_secs(1));
    }
}
