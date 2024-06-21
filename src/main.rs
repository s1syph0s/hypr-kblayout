use clap::Parser;
use hypr_kblayout::args::Args;
use hypr_kblayout::error::KbError;
use hypr_kblayout::parser::KeyboardConfig;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::{env, error, process};

const HYPR_SOCKET: &str = "HYPRLAND_INSTANCE_SIGNATURE";
const XDG_RUNTIME_DIR: &str = "XDG_RUNTIME_DIR";
const SOCKET_NAME: &str = ".socket.sock";
const SOCKET2_NAME: &str = ".socket2.sock";

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() {
    match try_main() {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    }
}

fn try_main() -> Result<()> {
    let args = Args::parse();
    let hypr_socket = env::var(HYPR_SOCKET).unwrap_or_else(|x| {
        eprintln!("Env variable {} not found: {x}", HYPR_SOCKET);
        process::exit(1);
    });
    let xdg_dir = env::var(XDG_RUNTIME_DIR).unwrap_or_else(|x| {
        eprintln!("Env variable {} not found: {x}", XDG_RUNTIME_DIR);
        process::exit(1);
    });
    let sock_path = Path::new(&xdg_dir).join("hypr").join(hypr_socket).join(SOCKET2_NAME);
    println!("{}", sock_path.display());

    init_layout(&args.name)?;

    let mut stream = UnixStream::connect(sock_path)?;
    loop {
        let mut buf: [u8; 1024] = [0; 1024];
        let _len = stream.read(&mut buf)?;
        if let Some(end) = newline_idx(&buf) {
            let msg = std::str::from_utf8(&buf)?;
            let msg = &msg[..end];

            let Some(kbd_conf) = KeyboardConfig::new(msg, &args.name) else {
                continue;
            };
            let lang = kbd_conf.layout().split_once(' ').unwrap().0;

            let json_msg = JsonMsg { text: lang };

            let j = serde_json::to_string(&json_msg)?;
            println!("{}", j);
        }
    }
}

fn init_layout(target_kb: &str) -> Result<()> {
    let hypr_socket = env::var(HYPR_SOCKET).unwrap_or_else(|x| {
        eprintln!("Env variable {} not found: {x}", HYPR_SOCKET);
        process::exit(1);
    });
    let xdg_dir = env::var(XDG_RUNTIME_DIR).unwrap_or_else(|x| {
        eprintln!("Env variable {} not found: {x}", XDG_RUNTIME_DIR);
        process::exit(1);
    });
    let sock_path = Path::new(&xdg_dir).join("hypr").join(hypr_socket).join(SOCKET_NAME);

    let mut stream = UnixStream::connect(sock_path)?;
    stream.write_all(b"devices")?;

    let mut buf: [u8; 8192] = [0; 8192];
    let _len = stream.read(&mut buf)?;
    let msg = std::str::from_utf8(&buf)?;

    let msg = get_active_layout(msg, target_kb)?;
    let lang = msg.split_once(' ').unwrap().0;

    let json_msg = JsonMsg { text: lang };

    let j = serde_json::to_string(&json_msg)?;
    println!("{}", j);
    Ok(())
}

fn get_active_layout<'a>(msg: &'a str, target_kb: &str) -> Result<&'a str> {
    let curr_kmap = &msg[msg.find("Keyboards:").unwrap()..];

    let Some(i) = curr_kmap.find(target_kb) else {
        return Err(Box::new(KbError::LayoutNotFound(target_kb.to_string())));
    };

    let curr_kmap = &curr_kmap[i..];

    let curr_kmap = &curr_kmap[curr_kmap.find("keymap:").unwrap() + 8..];
    let curr_kmap = &curr_kmap[..curr_kmap.find("\n\t").unwrap()];

    Ok(curr_kmap)
}

fn newline_idx(arr: &[u8]) -> Option<usize> {
    for (i, b) in arr.iter().enumerate() {
        if *b == 10u8 {
            return Some(i);
        }
    }
    None
}

#[derive(Serialize, Deserialize)]
struct JsonMsg<'a> {
    text: &'a str,
}

#[cfg(test)]
mod tests {
    use super::init_layout;

    #[test]
    fn test_init_layout() {
        let _ = init_layout("hp,-inc-hyperx-alloy-origins");
    }
}
