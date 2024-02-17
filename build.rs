use anyhow::bail;
use std::process::Command;

fn main() -> anyhow::Result<()> {
    if cfg!(target_os = "windows") {
        bail!("does not work on Windows yet");
    }
    let status = Command::new("ffmpeg")
        .args(["-version"])
        .status()
        .expect("unable to run process");

    if !status.success() {
        bail!("FFMpeg does not seems to be installed");
    }

    Ok(())
}
