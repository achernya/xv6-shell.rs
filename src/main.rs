// Author: Alex Chernyakhovsky (achernya@mit.edu)

// TODO(achernya): Per the documentation on doc.rust-lang.org, std::io
// is not yet ready, so use old_io until 1.0-final.
#![feature(old_io)]

// std::env contains a lot of nice functions that otherwise would
// require std::os to use; std::os has lots of deprecated functions.
#![feature(env)]

// TODO(achernya): Remove this feature when std::env moves over to
// std::path.
#![feature(old_path)]

use std::env;
use std::old_io as io;

// println_stderr is like println, but to stderr.
macro_rules! println_stderr(
    ($($arg:tt)*) => (
        match writeln!(&mut io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);

// ShellCommand is a trait that defines a runnable POSIX shell
// command. An implementation is an abstract representation of shell
// commands such as simple invocations, invocations with redirection,
// and even shell pipelines.
trait ShellCommand {
    fn run(&self);
}

fn shell_loop() {
    let mut stdin = io::stdin();
    loop {
        print!("$ ");
        let line = stdin.read_line();
        match line {
            Ok(expr) => handle_command(&expr),
            Err(_) => break,
        }
    }
}

fn handle_command(user_expr: &str) {
    // Clean up the string by removing the newline at the end
    let expr = user_expr.trim_matches('\n');
    let components: Vec<&str> = expr.split(' ').collect();
    if builtins(&components) {
        return;
    }
}

fn builtins(command: &Vec<&str>) -> bool {
    match command[0] {
        "cd" => cd(command),
        "pwd" => pwd(),
        _ => return false,
    }
    true
}

fn cd(command: &Vec<&str>) {
    // cd is the "change directory" command. It can take either 0 or 1
    // arguments. If given no arguments, then the $HOME directory is
    // chosen.
    let dir: Option<Path> = match command.len() {
        0 => panic!("invalid cd invocation"),
        1 => env::home_dir(),
        _ => Some(Path::new(command[1]))
    };
    if dir.is_none() {
        println_stderr!("cd: no directory to change to");
        return;
    }
    let directory = dir.unwrap();
    let result = env::set_current_dir(&directory);
    match result {
        Err(err) => {
            println_stderr!("cd: {}: {}", directory.display(), err);
        },
        _ => {},
    }
}

fn pwd() {
    let p = env::current_dir().unwrap_or(Path::new("/"));
    println!("{}", p.display());
}

fn main() {
    // TODO(achernya): is there any initialization we want to do
    // before we enter the shell loop?
    shell_loop();
}
