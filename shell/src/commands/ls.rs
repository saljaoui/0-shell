
pub fn builtin_ls(args: &[&str]){
    let mut show_hidden = false;
    let mut long_format = false;
    let mut is_dir = false;
    let mut paths = Vec::new();
    for arg in args{
        match *arg{
            "-l"=> show_hidden = true,
            "-a"=> long_format = true,
            "-F"=> is_dir = true,
            "--help"=> println!("Usage: ls [OPTION]...\nList information about the FILEs (the current directory by default).\n\nOptions:\n  -l      use a long listing format\n  -a      do not ignore entries starting with .\n  -F      append indicator (one of */=>@|) to entries\n  --help  display this help and exit"),
            _=> {
                if arg.starts_with('-'){
                    for a in arg.chars(){
                        match a {
                            'l'=> show_hidden = true,
                            'a'=> long_format = true,
                            'F'=> is_dir = true,
                            '-'=>{},
                            _=>{
                                println!("ls: invalid option -- '{}'\nTry 'ls --help' for more information.",a)
                            }
                        }
                    }
                }else {
                    paths.push(*arg)
                }
            }
        }
    }
    if paths.is_empty(){
        paths.push(".")
    }
    for path in paths {
        list_directory(path, show_hidden, long_format, is_dir);
    }

}

fn list_directory(path: Vec<&str>, show_hidden: bool, long_format: bool, is_dir: bool){

}