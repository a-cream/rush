use crate::run::error::ShellError;
use crate::run::bic;
use crate::run::node;
use std::fs::{File, OpenOptions};
use std::io;
use std::process::{Child, Command, ExitStatus, Stdio};

pub fn execute(ast: node::AST) -> Result<(), ShellError> {
    match ast {
        node::AST::Command(args, output) => {
            if args[0] == "cd" {
                let arg = if args.len() > 1 { &args[1] } else { "~" };
                bic::cd(arg).map_err(|e| ShellError::BicError(e))?;
                return Ok(());
            } else if args[0] == "exit" {
                let code = if args.len() > 1 {
                    args[1].parse::<i32>().map_err(|_| {
                        ShellError::InvalidArgument("exit code must be a number".to_string())
                    })?
                } else {
                    0
                };
                bic::exit(code);
            }

            let mut cmd = Command::new(&args[0]);
            if args.len() > 1 {
                cmd.args(&args[1..]);
            }
            cmd.stdin(Stdio::inherit());

            if let Some((file, append)) = output {
                let file = if append {
                    OpenOptions::new().append(true).create(true).open(file)
                } else {
                    File::create(file)
                }
                .map_err(|e| ShellError::from(e))?;
                cmd.stdout(Stdio::from(file));
            } else {
                cmd.stdout(Stdio::inherit());
            }

            let mut child = cmd.spawn().map_err(|e| {
                if e.kind() == io::ErrorKind::NotFound {
                    ShellError::CommandNotFound(args[0].clone())
                } else {
                    ShellError::IoError(e)
                }
            })?;

            child.wait().map_err(|e| ShellError::from(e))?;

            Ok(())
        }
        node::AST::Pipeline(commands, output) => {
            let mut previous_stdout = None;
            let mut children: Vec<Child> = Vec::new();

            for (i, command) in commands.iter().enumerate() {
                if command[0] == "cd" {
                    let arg = if command.len() > 1 { &command[1] } else { "~" };
                    bic::cd(arg).map_err(|e| ShellError::InvalidArgument(e))?;
                    continue;
                } else if command[0] == "exit" {
                    let code = if command.len() > 1 {
                        command[1].parse::<i32>().unwrap_or(0)
                    } else {
                        0
                    };
                    bic::exit(code);
                }

                let mut cmd = Command::new(&command[0]);
                if command.len() > 1 {
                    cmd.args(&command[1..]);
                }

                if let Some(stdout) = previous_stdout {
                    cmd.stdin(stdout);
                } else {
                    cmd.stdin(Stdio::inherit());
                }

                if i == commands.len() - 1 {
                    if let Some((file, append)) = &output {
                        let file = if *append {
                            OpenOptions::new().append(true).create(true).open(file)
                        } else {
                            File::create(file)
                        }
                        .map_err(|e| ShellError::from(e))?;
                        cmd.stdout(Stdio::from(file));
                    } else {
                        cmd.stdout(Stdio::inherit());
                    }
                } else {
                    cmd.stdout(Stdio::piped());
                }

                let mut child = cmd.spawn().map_err(|e| ShellError::from(e))?;
                previous_stdout = child.stdout.take();
                children.push(child);
            }

            for mut child in children {
                child.wait().map_err(|e| ShellError::from(e))?;
            }

            Ok(())
        }
        node::AST::AndList(commands) => {
            for command in commands {
                let status = execute_status(command)?;
                if !status.success() {
                    break;
                }
            }
            Ok(())
        }
    }
}

fn execute_status(ast: node::AST) -> Result<ExitStatus, ShellError> {
    match ast {
        node::AST::Command(args, output) => {
            let mut cmd = Command::new(&args[0]);
            if args.len() > 1 {
                cmd.args(&args[1..]);
            }
            let stdout = if let Some((file, append)) = output {
                let file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(append)
                    .open(file)
                    .map_err(|e| ShellError::from(e))?;
                Stdio::from(file)
            } else {
                Stdio::inherit()
            };

            let mut child = cmd
                .stdin(Stdio::inherit())
                .stdout(stdout)
                .spawn()
                .map_err(|e| ShellError::from(e))?;
            child.wait().map_err(|e| ShellError::from(e))
        }
        _ => {
            // For non-command AST nodes, we call execute recursively.
            execute(ast)?;
            Ok(ExitStatus::default())
        }
    }
}
