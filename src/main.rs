extern crate i3ipc;
use i3ipc::I3Connection;
use i3ipc::reply::Node;
use std::path::PathBuf;
use std::{io,fs};
use std::process::Command;

fn main() {
    // establish a connection to Sway over a unix socket
    let mut connection = I3Connection::connect().expect("couldn't connect to sway socket");
    let root_node = connection.get_tree().expect("coudln't get tree");
    if let Some(path) = try_find_cwd(&root_node) {
        println!("{:?}", path);
        Command::new("alacritty").args(&["--working-directory", path.to_str().unwrap()]).spawn().unwrap();
    } else {
        println!("No cwd found");
        Command::new("alacritty").spawn().unwrap();
    }
}

fn get_focused_node(node: &Node) -> Option<&Node> {
    if node.focused {
        Some(node)
    } else {
        let want_id = *node.focus.get(0)?;
        let child = node.nodes.iter().find(|n| want_id == n.id)?;
        get_focused_node(child)
    }
}

fn try_find_cwd<'a>(node: &'a Node) -> Option<PathBuf> {
    let focused_node = get_focused_node(node)?;
    //println!("{:?}", focused_node);
    Some(get_cwd_from_pid(focused_node.pid?).expect("failed to read current working directory"))
}

fn get_cwd_from_pid(pid: i64) -> io::Result<PathBuf> {
    let output = Command::new("pgrep")
            .args(&["-P", &pid.to_string()])
            .output()?;
    let child_pid = String::from_utf8_lossy(&output.stdout);
    let child_pid = child_pid.split("\n").next().unwrap().trim();
    println!("child pid {}", child_pid);
    let mut cwd_path = PathBuf::from("/proc");
    cwd_path.push(&child_pid);
    cwd_path.push("cwd");
    fs::read_link(cwd_path)
}