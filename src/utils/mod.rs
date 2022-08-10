pub mod packages;

use std::{fs, io, path::Path};

/// This has not been supported directory symlink in windows environment yet.
pub fn create_symlink(in_dir: &Path, out_dir: Option<&Path>) -> io::Result<()> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows;
        for entry in in_dir.read_dir().expect("It must be set a directory.") {
            if let Ok(entry) = entry {
                let entry_absolute = fs::canonicalize(entry.path()).unwrap();
                windows::fs::symlink_file(
                    &entry_absolute,
                    out_dir
                        .unwrap_or(Path::new("~/"))
                        .join(format!(".{}", &entry.file_name().to_str().unwrap())),
                ).unwrap_or(());;
            }
        }
        Ok(())
    }
    #[cfg(not(target_os = "windows"))]
    {
        use std::os::unix;
        for entry in in_dir.read_dir().expect("It must be set a directory.") {
            if let Ok(entry) = entry {
                let entry_absolute = fs::canonicalize(entry.path()).unwrap();
                unix::fs::symlink(
                    &entry_absolute,
                    out_dir
                        .unwrap_or(Path::new("~/"))
                        .join(format!(".{}", &entry.file_name().to_str().unwrap())),
                ).unwrap_or(());
            }
        }
        Ok(())
    }
}
