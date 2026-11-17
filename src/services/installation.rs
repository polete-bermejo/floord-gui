use std::env::consts::OS;
use std::path::PathBuf;
use std::process::Command;
use std::thread;

use anyhow::Result;

pub fn install_if_not() -> Result<()> {
    let bin_path = match OS {
        "linux" => dirs::executable_dir()
            .map(|f| f.join("floord"))
            .ok_or_else(|| panic!("Couldn't get executable path"))?,
        "windows" => dirs::data_local_dir()
            .map(|f| f.join("Floord/floord.exe"))
            .ok_or_else(|| panic!("Couldn't get 'config_logal_dir'"))?,
        _ => {
            panic!("Os not supported.")
        }
    };

    if !bin_path.exists() {
        install(bin_path)?;
    }

    Ok(())
}

#[cfg(target_os = "linux")]
fn install(bin_path: PathBuf) -> Result<()> {
    let link = "https://github.com/polete-bermejo/floord/releases/latest/download/floord-linux";
    thread::spawn(move || -> Result<()> {
        let status = Command::new("curl")
            .args(["-fL", link, "-o"])
            .arg(&bin_path)
            .status()?;

        if !status.success() {
            anyhow::bail!("Download failed");
        }

        let status = Command::new("chmod").arg("+x").arg(bin_path).status()?;

        if !status.success() {
            anyhow::bail!("Could not make binary executable");
        }

        Ok(())
    });

    Ok(())
}

#[cfg(target_os = "windows")]
fn instalal(bin_path: PathBuf) -> Result<()> {
    let link =
        "https://github.com/polete-bermejo/floord/releases/latest/download/floord-windows.exe";
    thread::spawn(move || -> Result<()> {
        let status = Command::new("curl.exe")
            .args(["-fL", link, "-o"])
            .arg(bin_path)
            .status()?;

        if !status.success() {
            anyhow::bail!("Download failed")
        }

        Ok(())
    });
    Ok(())
}
