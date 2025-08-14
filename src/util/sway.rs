use std::error::Error;
use crate::instance::InstanceManager;
use swayipc::Connection;
use swayipc::{Event, EventType, WindowChange, Fallible};
use std::fs;
use std::io;
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};

pub fn sway_ipc_connection_to_socket<P: AsRef<Path>>(path: P) -> io::Result<Connection> {
    let unix_stream = UnixStream::connect(path)?;
    Ok(Connection::from(unix_stream))
}

pub fn sway_load_script(socket: Box<Path>, instance_manager: &InstanceManager, cmd: String) -> Result<(), Box<dyn Error>> {
    let instance_manager = instance_manager.clone();

    let connection = match sway_ipc_connection_to_socket(&socket) {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to connect to Sway: {}", e);
            return Err("Failed to connect to Sway".into());
        }
    };

    let mut cmd_connection = match sway_ipc_connection_to_socket(&socket) {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to create command connection: {}", e);
            return Err("Failed to create command connection".into());
        }
    };

    // Subscribe to Window event
    let events = match connection.subscribe([EventType::Window]) {
        Ok(evts) => evts,
        Err(e) => {
            eprintln!("Failed to subscribe to Sway events: {}", e);
            return Err("Failed to subscribe to Sway events".into());
        }
    };

    // Launch game instances in Sway
    let mut sway_command = String::new();
    sway_command.push_str("exec sh -c '");
    sway_command.push_str(&cmd);
    sway_command.push_str("'");
    
    match cmd_connection.run_command(sway_command) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Failed to launch game instances: {}", e);
            return Err("Failed to launch game instances".into());
        }
    }

    let mut current_row = 0;
    let mut position_in_row = 0;
    let mut row_capacity = instance_manager.get_row(0).len() as u32;
    
    for event_result in events {
        match event_result {

            // Trigger only when a window spawn, a game instance normally
            Ok(Event::Window(window_event)) if window_event.change == WindowChange::New => {
                // Start a new row if needed
                if position_in_row == 0 {
                    if let Err(e) = position_new_row(&mut cmd_connection, window_event.container.id) {
                        eprintln!("Failed to position window: {}", e);
                    }
                }

                // Move to next row if current is full
                if position_in_row >= row_capacity {
                    current_row += 1;
                    position_in_row = 0;
                    row_capacity = instance_manager.get_row(current_row).len() as u32;
                }

                position_in_row += 1;
            },

            Ok(Event::Shutdown(_)) => {
                break;
            }

            Ok(_) => {}, // Ignore other events we're subscribed to
            Err(e) => eprintln!("Error receiving event: {}", e),
        }
    }

    Ok(())
}

pub fn position_new_row(connection: &mut Connection, container_id: i64) -> Fallible<()> {
    // Move window down to create a new row
    let cmd = format!("[con_id={}] move down", container_id);
    connection.run_command(&cmd)?;

    // Make this row use horizontal split
    let cmd = format!("[con_id={}] splith", container_id);
    connection.run_command(&cmd)?;

    Ok(())
}

/// Sway socket are store there `/run/user/1000/sway-ipc.1000.{number}.sock`.
///
/// Example: `/run/user/1000/sway-ipc.1000.145042.sock`
pub fn get_sway_socket() -> io::Result<Vec<PathBuf>> {
    let dir = Path::new("/run/user/1000");
    let prefix = "sway-ipc.1000.";
    let suffix = ".sock";

    let mut sockets: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {

                if filename.starts_with(prefix) && filename.ends_with(suffix) {
                    sockets.push(path);
                }
            }
        }
    }

    Ok(sockets)
}