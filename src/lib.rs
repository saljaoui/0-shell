use std::io;
use std::io::Write;

pub fn parse_command(input: &str) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];
    let mut start = 0;
    let mut in_token = false;
    let mut quotes = ('\0', false);

    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();

    while i < chars.len() {
        let c = chars[i];

        if quotes.1 {
            if c == quotes.0 {
                quotes.1 = false;
                tokens.push(input[start..i].to_string());
                in_token = false;
            }
            i += 1;
            continue;
        }

        if c == '"' || c == '\'' {
            quotes.0 = c;
            quotes.1 = true;
            start = i + 1;
            in_token = true;
            i += 1;
            continue;
        }

        if c.is_whitespace() {
            if in_token {
                tokens.push(input[start..i].to_string());
                in_token = false;
            }
            i += 1;
            continue;
        }

        if !in_token {
            start = i;
            in_token = true;
        }

        i += 1;
    }

    if in_token && !quotes.1 {
        tokens.push(input[start..].to_string());
    }

    if quotes.1 {
        let mut vec_in_loop = vec![];

        loop {
            print!("> ");
            match io::stdout().flush() {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{e}");
                    return vec![];
                }
            };

            let mut input_user = String::new();
            match io::stdin().read_line(&mut input_user) {
                Ok(0) => {
                    eprintln!("Syntax error: Unterminated quoted string"); // EOF (Ctrl+D)
                    break;
                }
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                    continue;
                }
            }
            if ckeck_quote(quotes.0, input_user.clone()) {
                tokens.push(input[start..].to_string());
                let correct_word: String = input_user.replace('\'', "");
                vec_in_loop.push(correct_word);

                if tokens[0] == "echo" {
                    let mut first = false;
                    for t in tokens.clone() {
                        if !first {
                            first = true
                        } else {
                            println!("{}", t)
                        }
                    }
                    for v in vec_in_loop {
                        print!("{}", v)
                    }
                    return vec![];
                }

                break;
            } else {
                vec_in_loop.push(input_user);
            }
        }

        let joined = format!("\n{}", vec_in_loop.join(""));
        let escaped = joined.replace("\"", "");
        if tokens.len() > 1 {
            tokens[1].push_str(&escaped);
        }
        return tokens;
    }

    tokens
}

fn ckeck_quote(quote: char, input: String) -> bool {
    let mut quotes = 0;
    for i in input.chars() {
        if i == quote {
            quotes += 1
        }
    }
    if quotes == 1 {
        return true;
    }
    false
}

#[derive(Debug, Clone, PartialEq)]
pub enum JobStatus {
    Running,
    Stopped,
    Terminated,
    Done,
}

#[derive(Debug, Clone)]
pub struct Job {
    pub id: usize,
    pub pid: u32,
    pub command: String,
    pub status: JobStatus,
}
pub static mut JOBS: Vec<Job> = Vec::new();

pub fn add_job(pid: u32, command: String, status: JobStatus) -> usize {
    let mut id = 0;
    unsafe {
        id = JOBS.len() + 1;
        JOBS.push(Job {
            id,
            pid,
            command,
            status,
        });
    }
    id
}

pub fn list_jobs() {
    unsafe {
        // for job in JOBS.iter() {
        //     println!("[{}] {:?}                 {}&", job.id, job.status, job.command);
        // }

        if JOBS.is_empty() {
            return;
        }
        let last_bg = JOBS[JOBS.len() - 1].id;
        let previous_bg = if JOBS.len() > 1 {
            JOBS[JOBS.len() - 2].id
        } else {
            2
        };

        for job in JOBS.iter() {
            if job.status == JobStatus::Done {
                continue;
            }

            let marker = if job.id == last_bg {
                "+"
            } else if job.id == previous_bg {
                "-"
            } else {
                ""
            };
            let status_str = match job.status {
                JobStatus::Running => "Running",
                JobStatus::Stopped => "Stopped",
                JobStatus::Terminated => "Terminated",
                JobStatus::Done => "Done",
            };

            println!(
                "[{}]{} {}                 {}&",
                job.id, marker, status_str, job.command
            );
        }
    }
}

pub fn list_jobs_with_flags(flags: &[&str]) {
    unsafe {
        let last_bg = JOBS[JOBS.len() - 1].id;
        let previous_bg = if JOBS.len() > 1 {
            JOBS[JOBS.len() - 2].id
        } else {
            2
        };

        let show_pid = flags.contains(&"-l");
        let pid_only = flags.contains(&"-p");
        let running_only = flags.contains(&"-r");
        let stopped_only = flags.contains(&"-s");

        for job in JOBS.iter() {
            if job.status == JobStatus::Done {
                continue;
            }

            if running_only && job.status != JobStatus::Running {
                continue;
            }

            if stopped_only && job.status != JobStatus::Stopped {
                continue;
            }

            if pid_only {
                println!("{}", job.pid);
            } else {
                let marker = if job.id == last_bg {
                    "+"
                } else if job.id == previous_bg {
                    "-"
                } else {
                    ""
                };
                let status_str = match job.status {
                    JobStatus::Running => "Running",
                    JobStatus::Stopped => "Stopped",
                    JobStatus::Terminated => "Terminated",
                    JobStatus::Done => "Done",
                };

                if show_pid {
                    println!(
                        "[{}]{} {} {}                 {}",
                        job.id, marker, job.pid, status_str, job.command
                    );
                } else {
                    println!(
                        "[{}]{} {}                 {}",
                        job.id, marker, status_str, job.command
                    );
                }
            }
        }
    }
}

// use nix::unistd::execvp;
// use std::ffi::CString;

// pub fn execute_external_command(cmd: &str, args: &[&str]) {
//     let cmd_cstring = CString::new(cmd).expect("Failed to convert cmd to CString");

//     let mut arg_cstrings: Vec<CString> = vec![cmd_cstring.clone()];
//     for arg in args {
//         arg_cstrings.push(CString::new(*arg).expect("Failed to convert arg to CString"));
//     }

   
//     let cstr_args: Vec<&CString> = arg_cstrings.iter().collect();

//     match execvp(&cmd_cstring, &cstr_args) {
//         Ok(_) => {
//             // This should never be reached since execvp replaces the process
//         }
//         Err(_) => {
//             eprintln!("{}: command not found", cmd);
//             std::process::exit(127); // Standard exit code for command not found
//         }
//     }
// }
use std::process::{Command, exit};

pub fn execute_external_command(cmd: &str, args: &[&str]) {
    let status = Command::new(cmd)
        .args(args)
        .status();

    match status {
        Ok(status) => {
            if !status.success() {
                eprintln!("{}: command exited with status {}", cmd, status);
                exit(status.code().unwrap_or(1));
            }
        }
        Err(_) => {
            eprintln!("{}: command not found", cmd);
            exit(127); // Standard exit code for command not found
        }
    }
}

pub fn cleanup_finished_jobs() {
    unsafe {
        JOBS.retain(|job| job.status != JobStatus::Done);
    }
}

pub fn update_job_status(pid: u32, new_status: JobStatus) {
    unsafe {
        for job in JOBS.iter_mut() {
            if job.pid == pid {
                job.status = new_status;
                break;
            }
        }
    }
}

use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};

pub fn check_background_jobs() {
    use crate::{update_job_status, JobStatus, JOBS};

    unsafe {
        for job in JOBS.iter_mut() {
            if job.status == JobStatus::Running {
                match waitpid(
                    Some(nix::unistd::Pid::from_raw(job.pid as i32)),
                    Some(WaitPidFlag::WNOHANG),
                ) {
                    Ok(WaitStatus::StillAlive) => {}
                    Ok(WaitStatus::Exited(_, _)) => {
                        update_job_status(job.pid, JobStatus::Done);
                        println!("[{}] Done\t{}", job.pid, job.command);
                    }
                    Ok(WaitStatus::Signaled(_, signal, _)) => {
                        if signal == nix::sys::signal::SIGKILL {
                            update_job_status(job.pid, JobStatus::Terminated);
                            // println!("[{}] Terminated\t{}", job.id, job.command);
                        } else if signal == nix::sys::signal::SIGCONT {
                            println!("e-----");
                            update_job_status(job.pid, JobStatus::Running);
                        } else {
                            println!(
                                "[{}] Signaled with signal: {}\t{}",
                                job.pid, signal, job.command
                            );
                        }
                        // update_job_status(job.pid, JobStatus::Terminated);
                        // println!("[{}] Terminated\t{}", job.pid, job.command);
                    }
                    Ok(WaitStatus::Stopped(_, _)) => {
                        update_job_status(job.pid, JobStatus::Stopped);
                        println!("[{}] Stopped\t{}", job.pid, job.command);
                    }
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error checking job {}: {}", job.pid, e);
                    }
                }
            }
        }
    }
}

use nix::sys::signal::{kill, Signal};
pub fn kill_job(job_id: u32, is_id: bool) {
    use nix::unistd::Pid;

    unsafe {
        if let Some(job) = JOBS.iter().find(|j| {
            if is_id {
                j.id == job_id as usize
            } else {
                j.pid == job_id
            }
        }) {
            if job.status == JobStatus::Running || job.status == JobStatus::Stopped {
                if let Err(e) = kill(Pid::from_raw(job.pid as i32), Signal::SIGKILL) {
                    eprintln!("Failed to kill job {}: {}", job_id, e);
                } else {
                    println!("[{}] Terminated\t{}", job.id, job.command);
                    // update_job_status(job.pid, JobStatus::Terminated);
                }
            } else {
                println!("Job {} is not running or stopped", job_id);
            }
        } else {
            println!("No job found with ID {}", job_id);
        }
    }
}

use nix::unistd::Pid;
static mut FOREGROUND_PID: Option<u32> = None;
static mut SHOULD_STOP: bool = false;

pub fn foreground_job() {
    use nix::sys::wait::waitpid;

    unsafe {
        if JOBS.is_empty() {
            println!(" fg: No current job");
            return;
        }
        if let Some(job) = JOBS.iter_mut().find(|j| j.id == JOBS.len()) {
             println!("{}", job.command);

            // if let Err(e) = kill(Pid::from_raw(job.pid as i32), nix::sys::signal::SIGCONT) {
            //     eprintln!("Failed to continue job {}: {}", job.id, e);
            //     return;
            // }

            job.status = JobStatus::Running;

            FOREGROUND_PID = Some(job.pid);
            SHOULD_STOP = false;

            signal(20, signal_handler_z_exit);
            signal(2, signal_handler_c_ln);

            loop {
                match waitpid(Pid::from_raw(job.pid as i32), Some(WaitPidFlag::WNOHANG)) {
                    Ok(WaitStatus::StillAlive) => {
                        if SHOULD_STOP {
                            if let Err(e) = kill(Pid::from_raw(job.pid as i32), nix::sys::signal::SIGSTOP) {
                                eprintln!("Failed to stop job {}: {}", job.pid, e);
                            } else {
                                job.status = JobStatus::Stopped;
                                println!("\n[{}]+  Stopped\t{}", job.id, job.command);
                            }
                            break;
                        }
                        // Wait for a short time before checking again
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }
                    Ok(WaitStatus::Exited(_, _)) => {
                        job.status = JobStatus::Done;
                        println!("[{}] Done\t{}", job.id, job.command);
                        break;
                    }
                    Ok(WaitStatus::Signaled(_, signal, _)) => {
                        // job.status = JobStatus::Terminated;
                        // println!("[{}] Terminated by signal {}\t{}", job.id, signal, job.command);
                        if signal == nix::sys::signal::SIGKILL {
                            update_job_status(job.pid, JobStatus::Terminated);
                            // println!("[{}] Terminated\t{}", job.id, job.command);
                        } else {
                            //SIGINT
                            if signal == nix::sys::signal::SIGINT {
                                update_job_status(job.pid, JobStatus::Done);
                                cleanup_finished_jobs();
                            }
                    
                        }
                        break;
                    }
                    Ok(WaitStatus::Stopped(_, _)) => {
                        update_job_status(job.pid, JobStatus::Stopped);
                        println!("[{}] Stopped\t{}", job.id, job.command);
                        break;
                    }
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Failed to wait on job {}: {}", job.id, e);
                        break;
                    }
                }
            }

            signal(20, signal_handler_z);
            signal(2, signal_handler_c);
            FOREGROUND_PID = None;
        } else {
            println!("No stopped jobs found");
        }
    }
}

pub fn background(){
    use nix::sys::wait::waitpid;

    unsafe {
        if JOBS.is_empty() {
            println!(" bg: No current job");
            return;
        }
        if let Some(job) = JOBS.iter_mut().find(|j| j.id == JOBS.len()) {
            if job.status == JobStatus::Stopped {
                // if let Err(e) = kill(Pid::from_raw(job.pid as i32), nix::sys::signal::SIGCONT) {
                //     eprintln!("Failed to continue job {}: {}", job.id, e);
                //     return;
                // }
                job.status = JobStatus::Running;
                println!("[{}]+ \t{}", job.id, job.command);
            } else {
                println!("Job {} is not stopped", job.id);
            }
        } else {
            println!("No stopped jobs found");
        }
    }
}

pub extern "C" fn signal_handler_z_exit(_signal: i32) {
       unsafe {
        SHOULD_STOP = true;
    }
    //  std::process::exit(0);
}

pub extern "C" fn signal_handler_z(_signal: i32) {
    println!();
    print!("\x1b[32m0-shell\x1b[0m:$ ");
    io::stdout().flush().unwrap();
}
unsafe extern "C" {
    fn signal(signal: i32, handler: extern "C" fn(i32));
}
pub extern "C" fn signal_handler_c(_signal: i32) {
    println!();
    print!("\x1b[32m0-shell\x1b[0m:$ ");
    io::stdout().flush().unwrap(); // bach n9dro ndiro print dyal "$ " f terminal
}

pub extern "C" fn signal_handler_c_ln(_signal: i32) {
    println!()
}