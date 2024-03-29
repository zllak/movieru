use eyre::bail;
use std::process::Command;

fn main() -> eyre::Result<()> {
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

    let status = Command::new("ffprobe")
        .args(["-version"])
        .status()
        .expect("unable to run process");

    if !status.success() {
        bail!("FFProbe does not seems to be installed");
    }

    Ok(())
}
