use crate::commands::*;
// use fork::waitpid;
use shell::*;
use fork::{fork, Fork};
// handle 3la 7sap chno dkhl user lina fe input 
pub fn dispatch(input: &str) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let cmd = parts[0];
    let args = &parts[1..];
    if cmd == "cd" {
        cd::builtin_cd(args);
        return;
    }


match fork() {
   Ok(Fork::Parent(child)) => {
        match fork::waitpid(child) {
            Ok(_) => {/*println!("fffffffffff")*/},
            Err(_) => eprintln!("Failted to wait on child"),
        }
   }
   Ok(Fork::Child) => {
      match cmd {
        "echo" => echo::builtin_echo(args),
        "ls" => ls::builtin_ls(args),
        "cat" => cat::builtin_cat(args),
        "mkdir" => mkdir::builtin_mkdir(args),
        "rm" => rm::builtin_rm(args),
        "cp"=>cp::builtin_cp(args),
        "pwd"=>pwd::builtin_pwd(args),
        "mv"=>mv::builtin_mv(args),
        _=> println!("Command '{}' not found", cmd),
    }
    // println!("+++++++++++++");
    std::process::exit(0);
},
   Err(_) => println!("Fork failed"),
}
}
