use anyhow::Result;
use std::fs;
use std::os::windows::fs::MetadataExt;
use std::path::Path;
use walkdir::DirEntry;

use crate::utils::run_command;

const FILE_ATTRIBUTE_READONLY: u32 = 0x1;
const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;

/// Check if a directory entry has the Windows Hidden attribute set.
pub fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .metadata()
        .map(|m| (m.file_attributes() & FILE_ATTRIBUTE_HIDDEN) != 0)
        .unwrap_or(false)
}

/// Check if a std::fs::Metadata has the Windows Read-Only attribute set.
pub fn is_readonly(metadata: &fs::Metadata) -> bool {
    (metadata.file_attributes() & FILE_ATTRIBUTE_READONLY) != 0
}

/// Set or clear the Windows Hidden attribute on a path using `attrib`.
pub fn set_hidden(path: &Path, hidden: bool) -> Result<()> {
    let flag = if hidden { "+H" } else { "-H" };
    run_command(
        &format!("attrib {} \"{}\"", flag, path.to_string_lossy()),
        None,
    )?;
    Ok(())
}

/// Set or clear the read-only permission on a path.
pub fn set_readonly(path: &Path, readonly: bool) -> Result<()> {
    let metadata = fs::metadata(path)?;
    let mut perms = metadata.permissions();
    perms.set_readonly(readonly);
    fs::set_permissions(path, perms)?;
    Ok(())
}
