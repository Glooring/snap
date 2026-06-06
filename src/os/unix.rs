use anyhow::Result;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use walkdir::DirEntry;

/// Check if a directory entry is hidden (name starts with '.').
pub fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}

/// Check if a std::fs::Metadata indicates a read-only file (no owner write bit).
pub fn is_readonly(metadata: &fs::Metadata) -> bool {
    metadata.permissions().readonly()
}

/// No-op on Linux — the "hidden" attribute is a naming convention, not a
/// filesystem attribute.  Renaming files would break Git tracking.
pub fn set_hidden(_path: &Path, _hidden: bool) -> Result<()> {
    // Nothing to do on Unix; hidden is determined by the filename prefix.
    Ok(())
}

/// Set or clear the read-only permission on a path using POSIX mode bits.
pub fn set_readonly(path: &Path, readonly: bool) -> Result<()> {
    let metadata = fs::metadata(path)?;
    let mut perms = metadata.permissions();
    if readonly {
        // Remove write bits: keep current mode but clear u+w, g+w, o+w
        let mode = perms.mode() & !0o222;
        perms.set_mode(mode);
    } else {
        // Add owner write bit
        let mode = perms.mode() | 0o200;
        perms.set_mode(mode);
    }
    fs::set_permissions(path, perms)?;
    Ok(())
}
