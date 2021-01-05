// TODO: Take out before release
use std::fs::File;
use std::println;
use std::thread::spawn;

// Logging
use log::{info, LevelFilter};
use simplelog::*;

// Python Wrappers
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use pyo3::{wrap_pyfunction, wrap_pymodule};

// Automerge Libraries
use automerge_backend;
use automerge_protocol;

#[derive(Clone)]
pub struct Backend {
    handle: automerge_backend::Backend,
}

#[pyfunction]
fn initialize_backend(log_fs_path: &str) {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create(log_fs_path.to_owned() + "/gt_automerge.log").unwrap(),
        ),
    ])
    .unwrap();
    info!("Initializing automerge backend...");
}

pub fn init_submodule(module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(initialize_backend, module)?)?;
    Ok(())
}
