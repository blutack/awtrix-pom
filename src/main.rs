use argh::FromArgs;
use chrono::prelude::*;
use log::*;
use serde_json::json;
use std::sync::mpsc::channel;
use std::time::Duration;
use url::Url;

const APP_NAME: &str = "pomodoro";
const BEEP: &str = "Beep::32a";

#[derive(PartialEq, Clone, Copy, Debug)]
enum PomMode {
    Work,
    ShortBreak,
    LongBreak,
}

#[derive(FromArgs)]
/// A pomodoro timer for awtrix-light.
struct PomConfig {
    #[argh(positional)]
    /// base URL of an awtrix-light instance (http://my-clock.local or http://192.168.2.3)
    hostname: Url,

    #[argh(option, default = "25")]
    /// minutes to work for [25]
    work: u64,

    #[argh(option, default = "5")]
    /// minutes to rest for on short breaks [5]
    short: u64,

    #[argh(option, default = "20")]
    /// minutes to rest for on long breaks [20]
    long: u64,

    #[argh(option, default = "4")]
    /// how many short breaks before a long break [4]
    cycles: u64,

    #[argh(option, default = "true")]
    /// beep on transition between work and breaks [true]
    beep: bool,

    #[argh(option, default = "true")]
    /// switch to the timer app on the clock automatically [true]
    switch: bool,

    #[argh(switch, short = 'v')]
    /// print debug information
    verbose: bool,

    #[argh(switch)]
    /// interpret times as seconds, not minutes - useful for demos
    seconds: bool,
}

fn update(config: &PomConfig, mode: &PomMode, remaining: u64, total: u64) {
    let local: DateTime<Local> = Local::now();

    let text = match mode {
        PomMode::Work => format!("{}   {remaining:>2}", local.format("%H:%M")),
        PomMode::ShortBreak => format!("SHORT  {remaining:>2}"),
        PomMode::LongBreak => format!("LONG   {remaining:>2}"),
    };

    let progress_colour = match mode {
        PomMode::Work => "#00aaff",
        PomMode::ShortBreak => "#55aa7f",
        PomMode::LongBreak => "#55007f",
    };

    let remaining_percent = 100 - (((remaining as f64 / total as f64) * 100.0) as u64);

    let app = json!({
        "text": text,
        "progress": remaining_percent,
        "center": false,
        "lifetime": 90,
        "lifetimeMode": 1,
        "progressC": progress_colour,
    });

    let mut path = config.hostname.join("/api/custom").unwrap();
    path.set_query(Some(format!("name={}", APP_NAME).as_str()));

    debug!("app json: {:?}", app);

    ureq::post(path.as_str()).send_json(app).unwrap();
}

fn switch(config: &PomConfig) {
    debug!("switching to custom app {APP_NAME}");

    let path = config.hostname.join("/api/switch").unwrap();
    ureq::post(path.as_str())
        .send_json(json!({ "name": APP_NAME }))
        .unwrap();
}

fn beep(config: &PomConfig) {
    debug!("beep!");

    let path = config.hostname.join("/api/rtttl").unwrap();
    ureq::post(path.as_str()).send_string(BEEP).unwrap();
}

fn cleanup(config: &PomConfig) {
    debug!("removing custom app");

    let mut path = config.hostname.join("/api/custom").unwrap();
    path.set_query(Some(format!("name={}", APP_NAME).as_str()));

    ureq::post(path.as_str()).call().unwrap();
}

fn main() {
    let config: PomConfig = argh::from_env();

    stderrlog::new()
        .module(module_path!())
        .quiet(!config.verbose)
        .verbosity(log::Level::Trace)
        .init()
        .unwrap();

    let (running_sender, running_receiver) = channel();

    ctrlc::set_handler(move || {
        info!("cleaning up...");
        running_sender.send(()).expect("Failed to clean up");
    })
    .expect("Error setting Ctrl-C handler");

    let mut mode = PomMode::Work;

    let one_tick = if config.seconds {
        Duration::from_secs(1)
    } else {
        Duration::from_secs(60)
    };

    let mode_time = |mode| match mode {
        PomMode::Work => config.work,
        PomMode::ShortBreak => config.short,
        PomMode::LongBreak => config.long,
    };

    let mut remaining_cycles = config.cycles;
    let mut switch_to_app_needed = config.switch;

    info!("starting up...");

    'outer: loop {
        for remaining in (1..mode_time(mode) + 1).rev() {
            update(&config, &mode, remaining, mode_time(mode));

            if switch_to_app_needed {
                switch(&config);
                switch_to_app_needed = false;
            }

            if config.beep && remaining == 0 {
                beep(&config);
            }

            if running_receiver.recv_timeout(one_tick).is_ok() {
                break 'outer;
            }
        }

        mode = match mode {
            PomMode::Work if remaining_cycles <= 1 => {
                remaining_cycles = config.cycles;
                PomMode::LongBreak
            }
            PomMode::Work => {
                remaining_cycles -= 1;
                PomMode::ShortBreak
            }
            PomMode::ShortBreak => PomMode::Work,
            PomMode::LongBreak => PomMode::Work,
        };

        debug!("Mode {:?}, remaining cycles {}", mode, remaining_cycles);
    }

    cleanup(&config);
}
