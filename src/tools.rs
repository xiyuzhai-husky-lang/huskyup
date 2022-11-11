use anyhow::{anyhow, Result};

// A list of all binaries which Rustup will proxy.
static TOOLS: &[&str] = &["huskyc", "huskydoc", "corgi"];

// Tools which are commonly installed by Cargo as well as rustup. We take a bit
// more care with these to ensure we don't overwrite the user's previous
// installation.
static DUP_TOOLS: &[&str] = &["husky-analyzer", "huskyfmt", "corgi-fmt"];

// If the given name is one of the tools we proxy.
pub fn is_proxyable_tools(tool: &str) -> Result<()> {
    if TOOLS
        .iter()
        .chain(DUP_TOOLS.iter())
        .any(|&name| name == tool)
    {
        Ok(())
    } else {
        Err(anyhow!(format!(
            "unknown proxy name: '{}'; valid proxy names are {}",
            tool,
            TOOLS
                .iter()
                .chain(DUP_TOOLS.iter())
                .map(|s| format!("'{}'", s))
                .collect::<Vec<_>>()
                .join(", ")
        )))
    }
}
