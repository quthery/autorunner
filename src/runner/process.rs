use std::{
    io::{self, BufRead, BufReader},
    process::{Child, Command, Stdio},
    sync::{Arc, Mutex},
    thread,
};

pub fn spawn(command: &str) -> io::Result<(Child, thread::JoinHandle<()>, Arc<Mutex<bool>>)> {
    let mut child = Command::new(if cfg!(windows) { "cmd" } else { "sh" })
        .arg(if cfg!(windows) { "/c" } else { "-c" })
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let should_stop = Arc::new(Mutex::new(false));
    let stop_clone = Arc::clone(&should_stop);

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let handle = thread::spawn(move || {
        let out_reader = BufReader::new(stdout);
        let err_reader = BufReader::new(stderr);

        let out_lines = out_reader.lines();
        let err_lines = err_reader.lines();

        // Потоки для чтения stdout и stderr
        let out_thread = thread::spawn({
            let stop = Arc::clone(&stop_clone);
            move || {
                for line in out_lines {
                    if *stop.lock().unwrap() { break; }
                    if let Ok(line) = line {
                        println!("{}", line);
                    }
                }
            }
        });

        let err_thread = thread::spawn({
            let stop = Arc::clone(&stop_clone);
            move || {
                for line in err_lines {
                    if *stop.lock().unwrap() { break; }
                    if let Ok(line) = line {
                        eprintln!("{}", line);
                    }
                }
            }
        });

        let _ = out_thread.join();
        let _ = err_thread.join();
    });

    Ok((child, handle, should_stop))
}
