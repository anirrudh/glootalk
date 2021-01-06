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
use std::thread;

// Logging
use log::{debug, info, LevelFilter};
use simplelog::*;

mod amserver;
use amserver::init_submodule;

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

    thread::spawn(move || {
        let web_localhost: String = "127.0.0.1:".to_owned();
        let url = web_localhost + &port.to_string();
        info!("glootalkrs | Starting WebSocket Server on {}", url);
        let server = TcpListener::bind(url).unwrap();
        for stream in server.incoming() {
            spawn(move || {
                let mut websocket = accept(stream.unwrap()).unwrap();

                // -- The following payload has been extracted raw from the node js version.
                // It sends to the client an automerge "change" with the initial content text, "Hello"
                let mut bin_msg = Vec::new();
                bin_msg.extend(
                    [
                        133, 111, 74, 131, 238, 252, 154, 111, 1, 142, 1, 16, 77, 254, 31, 198, 51,
                        153, 72, 94, 144, 91, 62, 9, 75, 31, 110, 189, 1, 1, 224, 174, 215, 255, 5,
                        0, 0, 0, 1, 4, 0, 2, 7, 0, 2, 4, 0, 2, 7, 2, 9, 6, 0, 3, 5, 0, 0, 1, 11, 9,
                        0, 2, 126, 0, 3, 3, 1, 2, 125, 13, 18, 126, 5, 100, 111, 99, 73, 100, 8,
                        116, 101, 120, 116, 65, 114, 101, 97, 0, 7, 28, 4, 2, 5, 1, 1, 34, 8, 126,
                        1, 4, 5, 1, 126, 3, 1, 46, 9, 126, 230, 1, 0, 5, 22, 126, 0, 22, 47, 20,
                        97, 117, 116, 111, 109, 101, 114, 103, 101, 45, 114, 111, 111, 109, 104,
                        101, 108, 108, 111, 72, 56, 5, 7, 0, 126, 1, 0, 57, 2, 127, 0, 59, 2, 127,
                        3,
                    ]
                    .iter()
                    .copied(),
                );

                websocket
                    .write_message(tungstenite::Message::Binary(bin_msg))
                    .unwrap();

                // Instead of this mockup payload, we should send here the initial automerge "change" for
                // the given document.
                //-----

                loop {
                    let msg = websocket.read_message().unwrap();
                    if msg.is_binary() || msg.is_text() {
                        info!("Recieved message: {}", msg);

                    // -- Some examples of messages to write through the websocket ...

                    // -- echo received msg
                    //  websocket.write_message(msg).unwrap();

                    // -- send a text msg
                    // websocket
                    //     .write_message(tungstenite::Message::Text("TEST".to_string()))
                    //     .unwrap();

                    // -- send a binary msg, represented as a Vec<u8>
                    //let mut bin_msg = Vec::new();
                    //bin_msg.extend([3, 1, 4, 1, 5, 9].iter().copied());
                    // websocket
                    //     .write_message(tungstenite::Message::Binary(bin_msg))
                    //     .unwrap();
                    } else {
                        info!("Recieved non bin non txt message: {}", msg);
                    }
                }
            });
        }
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
