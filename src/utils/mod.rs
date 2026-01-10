pub mod packages;

use std::{fs, io, path::Path};

/// This has not been supported directory symlink in windows environment yet.
pub fn create_symlink(in_dir: &Path, out_dir: Option<&Path>, force: bool) -> io::Result<()> {
    let out_base = out_dir.unwrap_or(Path::new("~/"));
    create_symlink_recursive(in_dir, in_dir, out_base, force)
}

fn create_symlink_recursive(
    root_in_dir: &Path,
    current_dir: &Path,
    out_base: &Path,
    force: bool,
) -> io::Result<()> {
    for entry in current_dir.read_dir().expect("It must be set a directory.") {
        let entry = entry?;
        let entry_path = entry.path();
        let rel_path = entry_path.strip_prefix(root_in_dir).unwrap();
        let out_path = out_base.join(format!(".{}", rel_path.to_string_lossy()));

        if entry_path.is_dir() {
            if let Ok(meta) = fs::symlink_metadata(&out_path) {
                if !meta.is_dir() {
                    if force {
                        fs::remove_file(&out_path)?;
                    } else {
                        return Err(io::Error::new(
                            io::ErrorKind::AlreadyExists,
                            format!(
                                "Destination exists and is not a directory: {}",
                                out_path.to_string_lossy()
                            ),
                        ));
                    }
                }
            }
            fs::create_dir_all(&out_path)?;
            create_symlink_recursive(root_in_dir, &entry_path, out_base, force)?;
        } else {
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent)?;
            }
            if let Ok(meta) = fs::symlink_metadata(&out_path) {
                if meta.is_dir() {
                    if force {
                        fs::remove_dir_all(&out_path)?;
                    } else {
                        return Err(io::Error::new(
                            io::ErrorKind::AlreadyExists,
                            format!(
                                "Destination exists and is a directory: {}",
                                out_path.to_string_lossy()
                            ),
                        ));
                    }
                } else {
                    if force {
                        fs::remove_file(&out_path)?;
                    } else {
                        return Err(io::Error::new(
                            io::ErrorKind::AlreadyExists,
                            format!("Destination already exists: {}", out_path.to_string_lossy()),
                        ));
                    }
                }
            }
            let entry_absolute = fs::canonicalize(&entry_path).unwrap();
            #[cfg(target_os = "windows")]
            {
                use std::os::windows;
                windows::fs::symlink_file(&entry_absolute, &out_path).unwrap_or(());
            }
            #[cfg(not(target_os = "windows"))]
            {
                use std::os::unix;
                unix::fs::symlink(&entry_absolute, &out_path).unwrap_or(());
            }
        }
    }
    Ok(())
}
