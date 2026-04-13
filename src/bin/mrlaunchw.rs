#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use std::env;
use std::ffi::OsString;
use std::process::{Command, Stdio};

use anyhow::{Context, Result, anyhow};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

fn main() {
    if let Err(err) = run() {
        eprintln!("mrlaunchw error: {err:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let current_exe = env::current_exe().context("failed to resolve current executable")?;
    let exe_dir = current_exe
        .parent()
        .ok_or_else(|| anyhow!("failed to resolve executable directory"))?;

    #[cfg(target_os = "windows")]
    let target_name: OsString = OsString::from("mrlaunch.exe");

    #[cfg(not(target_os = "windows"))]
    let target_name: OsString = OsString::from("mrlaunch");

    let target = exe_dir.join(target_name);
    if !target.is_file() {
        return Err(anyhow!(
            "could not find sibling launcher binary: {}",
            target.display()
        ));
    }

    let args: Vec<OsString> = env::args_os().skip(1).collect();

    let mut cmd = Command::new(&target);
    cmd.args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    cmd.spawn()
        .with_context(|| format!("failed to launch {}", target.display()))?;

    Ok(())
}
