use anyhow::{Context, Result};

pub fn load() -> Result<String> {
    rbx_cookie::get_value().context(
        "could not find .ROBLOSECURITY — log into Roblox Studio once, or set ROBLOSECURITY env var",
    )
}
