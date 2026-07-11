use serde_json::{Value, json};
use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
};

pub struct MpvIpc;

const SOCKET: &str = "/tmp/synthpulse.sock";

impl MpvIpc {
    fn request(command: Value) -> Option<Value> {
        use std::io::{BufRead, BufReader};

        let mut socket = UnixStream::connect(SOCKET).ok()?;

        socket.write_all(command.to_string().as_bytes()).ok()?;
        socket.write_all(b"\n").ok()?;

        let mut reader = BufReader::new(socket);

        let mut response = String::new();

        reader.read_line(&mut response).ok()?;

        serde_json::from_str(&response).ok()
    }

    pub fn playback_time() -> Option<f64> {
        let reply = Self::request(json!({
            "command": ["get_property", "playback-time"]
        }))?;

        reply["data"].as_f64()
    }

    pub fn duration() -> Option<f64> {
        let reply = Self::request(json!({
            "command": ["get_property", "duration"]
        }))?;

        reply["data"].as_f64()
    }

    pub fn volume() -> Option<f64> {
        let reply = Self::request(json!({
            "command": ["get_property", "volume"]
        }))?;

        reply["data"].as_f64()
    }

    pub fn set_volume(volume: f64) {
        let _ = Self::request(json!({
            "command": ["set_property", "volume", volume]
        }));
    }
}
