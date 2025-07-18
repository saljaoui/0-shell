use crate::commands::*;
use fork::{fork, Fork};
use shell::*;

pub fn dispatch(input: &str) {
    let parts_string: Vec<String> = parse_command(input);
    let parts: Vec<&str> = parts_string.iter().map(|s| s.as_str()).collect();
    if parts.len() == 0 {
        return;
    }
    let cmd = parts[0];
    let args = &parts[1..];
    if cmd == "cd" {
        cd::builtin_cd(args);
        return;
    }
    if check_flag(cmd, args) {
        return;
    }

    match fork() {
        Ok(Fork::Parent(child)) => {
            match fork::waitpid(child) {
                Ok(_) => { /*println!("fffffffffff")*/ }
                Err(_) => eprintln!("Failted to wait on child"),
            }
        }
        Ok(Fork::Child) => {
            match cmd {
                "echo" => echo::builtin_echo(args),
                "ls" => ls::builtin_ls(args),    //
                "cat" => cat::builtin_cat(args), //
                "mkdir" => mkdir::builtin_mkdir(args),
                "rm" => rm::builtin_rm(args), //
                "cp" => cp::builtin_cp(args),
                "pwd" => pwd::builtin_pwd(args),
                "mv" => mv::builtin_mv(args),
                _ => println!("Command '{}' not found", cmd),
            }
            // println!("+++++++++++++");
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
