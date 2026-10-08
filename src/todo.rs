use std::io;
use std::num::NonZeroU8;
use std::path::PathBuf;
use std::{
    fs::{self, OpenOptions},
    io::{BufRead, BufReader},
};

use anyhow::{self, Context, Result, ensure};
use clap::{Parser, Subcommand};
use dirs::config_dir;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    /// Add/create a task to the list
    Add { txt: String },
    /// List the tasks
    List,
    /// Remove/delete a task by it`s position line. Must be a value greater than 0. > 0
    Remove { index: NonZeroU8 },
    /// Mark a task as done with "[x]". Must be a value greater than 0. > 0
    Done { index: NonZeroU8 },
}

pub fn file_path() -> Result<PathBuf> {
    let path = config_dir()
        .context("Erro ao tentar acessar/encontrar o `config_dir` do sistema!")?
        .join("oxi-todo")
        .join("todo.txt");
    Ok(path)
}

pub fn open_read_only_path() -> Result<(Vec<String>, PathBuf)> {
    let path = file_path()?;
    if !path.exists() {
        return Ok((Vec::new(), path));
    }
    let file = OpenOptions::new().read(true).open(&path)?;
    let lines: io::Result<Vec<String>> = BufReader::new(file).lines().collect();
    let lines = lines?;
    Ok((lines, path))
}

pub fn open_file() -> Result<(Vec<String>, PathBuf)> {
    let path = file_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(&path)?;
    let lines: io::Result<Vec<String>> = BufReader::new(file).lines().collect();
    let lines = lines?;
    Ok((lines, path))
}

pub fn list_todo() -> Result<()> {
    let (lines, _path) = open_read_only_path()?;
    ensure!(!lines.is_empty(), "Todo list is empty!");
    for l in lines {
        println!("{}", l);
    }
    Ok(())
}

pub fn add_todo(txt: &str) -> Result<()> {
    let (mut lines, path) = open_file()?;
    lines.push(format!("[] {txt}"));
    fs::write(&path, lines.join("\n") + "\n")?;
    Ok(())
}

pub fn remove_todo(index: NonZeroU8) -> Result<()> {
    let (mut lines, path) = open_file()?;
    ensure!(!lines.is_empty(), "Todo list is empty!");
    let index = index.get() as usize - 1;
    ensure!(
        lines.get(index).is_some(),
        "There's no task at that position!"
    );
    lines.remove(index);
    if lines.is_empty() {
        fs::write(&path, "")?;
    } else {
        fs::write(&path, lines.join("\n") + "\n")?;
    }
    Ok(())
}

pub fn done_todo(index: NonZeroU8) -> Result<()> {
    let (mut lines, path) = open_file()?;
    ensure!(!lines.is_empty(), "Todo list is empty!");
    let index = index.get() as usize - 1;
    let line = lines
        .get_mut(index)
        .context("There's no task at that position!")?;
    if let Some(pos) = line.find("[]") {
        line.replace_range(pos..pos + 2, "[x]");
    }
    fs::write(&path, lines.join("\n") + "\n")?;
    Ok(())
}
