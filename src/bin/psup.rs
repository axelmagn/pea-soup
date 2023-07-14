use clap::{Parser, Subcommand};
use std::process::{Command, Stdio};

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: CliCommands,
}

#[derive(Subcommand)]
enum CliCommands {
    Run {
        #[arg(last = true)]
        cmd_args: Vec<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        CliCommands::Run { cmd_args } => {
            assert!(cmd_args.len() > 0, "A command must be provided to run.");
            let cmd = &cmd_args[0];
            let cmd_args = &cmd_args[1..];

            let child = Command::new(cmd)
                .args(cmd_args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect(&format!(
                    "Failed to start child process: `{} {}`",
                    cmd,
                    cmd_args.join(" ")
                ));
            let output = child.wait_with_output().expect(&format!(
                "Failed to start child process: `{} {}`",
                cmd,
                cmd_args.join(" ")
            ));
            println!(
                "Output:\n{}",
                String::from_utf8(Vec::from(output.stdout.as_slice())).unwrap()
            );
        }
    }
}
