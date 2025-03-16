use hexahedron::engine::{*, settings::{EngineSettings, Fullscreen, Resolution}};
use hexahedron::{
    log::{info, warn, error},
    env_logger::Builder,
};

use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;

fn main() {
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("engine_test.log")
        .unwrap();
    let file_logger = Mutex::new(log_file);

    Builder::new()
        .format(move |_buf, record| {
            let mut file = file_logger.lock().unwrap();
            writeln!(file, "[{}] in {} :: {}", record.level(), record.target(), record.args()).unwrap();
            Ok(())
        })
        .filter_module("hexahedron", log::LevelFilter::Trace)
        .filter_module("engine_test", log::LevelFilter::Trace)
        .init();

    info!("Running Engine (Test)");

    Engine::run(EngineSettings {
        vsync: false,
        fullscreen: false,
        preferred_resolution: Resolution::FHD,
        title: "Hexahedron Engine Test".to_owned(),
        max_framerate: Some(180),
    });
}