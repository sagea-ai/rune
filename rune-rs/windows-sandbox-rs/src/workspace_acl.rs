use crate::acl::add_deny_write_ace;
use crate::path_normalization::canonicalize_path;
use anyhow::Result;
use std::ffi::c_void;
use std::path::Path;

pub fn is_command_cwd_root(root: &Path, canonical_command_cwd: &Path) -> bool {
    canonicalize_path(root) == canonical_command_cwd
}

/// # Safety
/// Caller must ensure `psid` is a valid SID pointer.
pub unsafe fn protect_workspace_rune_dir(cwd: &Path, psid: *mut c_void) -> Result<bool> {
    let cwd_rune = cwd.join(".rune");
    if cwd_rune.is_dir() {
        add_deny_write_ace(&cwd_rune, psid)
    } else {
        Ok(false)
    }
}
