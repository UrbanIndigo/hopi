use anyhow::Result;
use std::process::Command;

pub fn open_place(place_id: u64, universe_id: u64) -> Result<()> {
    let uri = format!(
        "roblox-studio:1+launchmode:edit+task:EditPlace+placeId:{place_id}+universeId:{universe_id}"
    );
    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg(&uri).status()?;
    }
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd").args(["/C", "start", "", &uri]).status()?;
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        Command::new("xdg-open").arg(&uri).status()?;
    }
    Ok(())
}
