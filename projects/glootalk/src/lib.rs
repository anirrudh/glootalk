use tokio_tungstenite;
use tungstenite;

// Websocket
use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;
use tungstenite::*;

// stdlib
use std::fs::File;
use std::println;

// Python Wrappers
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Thread
use std::sync::Arc;
use std::thread;

// Logging
use log::{debug, info, LevelFilter};
use simplelog::*;

mod amserver;
use amserver::init_submodule;

use automerge_backend as amb;
use automerge_frontend::{Frontend, InvalidChangeRequest, LocalChange, Path, Value};
use automerge_protocol as amp;

// Start a tungstenite based websocket server
#[pyfunction]
fn start_server(port: usize, log_path: &str) {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create(log_path.to_owned() + "/gt_wss.log").unwrap(),
        ),
    ])
    .unwrap();
    let mbackend = Arc::new(std::sync::Mutex::new(amb::Backend::init()));
    let mdoc = Arc::new(std::sync::Mutex::new(Frontend::new()));

    thread::spawn(move || {
        let web_localhost: String = "127.0.0.1:".to_owned();
        let url = web_localhost + &port.to_string();
        info!("glootalkrs | Starting WebSocket Server on {}", url);
        let server = TcpListener::bind(url).unwrap();
        for stream in server.incoming() {}
    });
}

// The main python module - glootalk
#[pymodule]
fn glootalk(py: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(start_server, module)?)?;
    let submod = PyModule::new(py, "automerge")?;
    init_submodule(submod)?;
    module.add_submodule(submod)?;
    Ok(())
}
