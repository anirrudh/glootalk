extern crate tokio_tungstenite;
extern crate tungstenite;

// Websocket
use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;

// stdlib
use std::println;

// Python Wrappers
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Thread
use std::thread;
use std::sync::mpsc::channel;

// Start a tungstenite based websocket server 
#[pyfunction]
fn start_server(port: usize) {
    thread::spawn(move || {
    let mut web_localhost: String = "127.0.0.1:".to_owned();
    let url = web_localhost + &port.to_string(); 
    println!("Starting WebSocket Server on {}", url);
    let server = TcpListener::bind(url).unwrap();
    for stream in server.incoming() {
    spawn (move || {
        let mut websocket = accept(stream.unwrap()).unwrap();
        loop {
            let msg = websocket.read_message().unwrap();
            if msg.is_binary() || msg.is_text() {
                println!("Recieved message: {}", msg);
                websocket.write_message(msg).unwrap();
            }
        }
    });
}
});
}


/// The main python module - glootalk
#[pymodule]
fn glootalk(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(start_server, m)?)?;
    Ok(())
}
