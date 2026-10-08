mod todo;

use crate::todo::{Args, Cmd, add_todo, done_todo, list_todo, remove_todo};
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = Args::parse();
    match cli.cmd {
        Cmd::List => list_todo()?,
        Cmd::Add { txt } => add_todo(&txt)?,
        Cmd::Remove { index } => remove_todo(index)?,
        Cmd::Done { index } => done_todo(index)?,
    }
    Ok(())
}
