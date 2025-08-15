use crate::commands::*;
use std::io::{self, Write};
use crate::signal_handler_c;

use fork::{fork, Fork};
use shell::*;
// use commands::utls_file::*;
// use crate::JobStatus;
// use crate::add_job;
//  use crate::list_jobs;


// handle 3la 7sap chno dkhl user lina fe input
pub fn dispatch(input: &str) {
    check_background_jobs();

     let mut input = input.trim();

    let is_background = input.ends_with('&');
    if is_background {
        input = input.trim_end_matches('&').trim();
    }

   
    let parts_string: Vec<String> = parse_command(input);
    let parts: Vec<&str> = parts_string.iter().map(|s| s.as_str()).collect();
    if parts.len() == 0 {
        return;
    }
    let cmd = parts[0];
    let args = &parts[1..];
    
     if cmd == "jobs" {
        if args.is_empty() {
                list_jobs();
            } else {
                list_jobs_with_flags(args);
        }
        return;
    } else if cmd == "kill"{
        let mut is_id = false;
        if args.len() != 1 {
            eprintln!("Usage: kill <job_id>");
            return;
        }
        let job_id: u32 = match args[0].parse() {
            Ok(id) => id,
            Err(_) => {
                if args[0].starts_with('%') {
                is_id = true;
                let id_str = &args[0][1..];
                match id_str.parse::<u32>() {
                    Ok(id) => id,
                    Err(_) => {
                        eprintln!("Invalid job ID");
                        return;
                    }
                }
                } else {
                eprintln!("Invalid job ID");
                return;
                }
                }
        };
        kill_job(job_id, is_id);
        return;
    } else if cmd == "fg"{
        foreground_job();
        return;
    } else if cmd == "cd" {
        cd::builtin_cd(args);
        return;
    } else if cmd == "bg"{
        background();
        return;
    }
    if check_flag(cmd, args) {
        return;
    }


    match fork() {
        Ok(Fork::Parent(child)) => {
            if is_background {
                let id = add_job(child as u32, input.to_string(), JobStatus::Running);
                println!("[{}] {}", id, child);
            } else {
                unsafe {
                    signal(2, signal_handler_ln); // CTRL+C
                }
                match fork::waitpid(child) {
                    Ok(_) => {}
                    Err(_) => eprintln!("Failed to wait on child"),
                }
                unsafe {
                    signal(2, signal_handler_c); // restore
                }
            }
            
        }
        Ok(Fork::Child) => {
            match cmd {
                "echo" => echo::builtin_echo(args),
                "ls" => ls::builtin_ls(args),
                "cat" => cat::builtin_cat(args),
                "mkdir" => mkdir::builtin_mkdir(args),
                "rm" => rm::builtin_rm(args),
                "cp" => cp::builtin_cp(args),
                "pwd" => pwd::builtin_pwd(args),
                "mv" => mv::builtin_mv(args),
                "clear"=>clear_screen(),
                _ => {
                    execute_external_command(cmd, args);
                    // eprintln!("Command '{}' not found", cmd);
                    // std::process::exit(1);
                },
            }
            std::process::exit(0);
        }
        Err(_) => println!("Fork failed"),
    }
}

fn check_flag(cmd: &str, args: &[&str]) -> bool {
    if matches!(cmd, "echo" | "mkdir" | "cp" | "pwd" | "mv") {
        for arg in args {
            if arg.starts_with('-') {
                println!(
                    "{}: error — you are using an unsupported flag '{}' (don't use flag for this command)",
                    cmd, arg
                );
                return true;
            }
        }
    }
    false
}

extern "C" fn signal_handler_ln(_signal: i32) {
    println!();
}
unsafe extern "C" {
    fn signal(signal: i32, handler: extern "C" fn(i32));
}


fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
     match io::stdout().flush(){
            Ok(_)=>{},
            Err(e)=>{
                eprintln!("{e}");
            }
        };
}