use std::{fmt::format, io, path::Path};

pub fn create_symlink(in_dir: &Path, out_dir: Option<&Path>) -> io::Result<()> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows;
        for entry in in_dir.read_dir().expect("It must be set a directory.") {
            if let Ok(entry) = entry {
                windows::fs::symlink_file(
                    entry.path(),
                    out_dir
                        .unwrap_or(Path::new("~/"))
                        .join(format!(".{:?}", &entry.file_name())),
                )?;
            }
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        use std::os::unix;
        for entry in in_dir.read_dir().expect("It must be set a directory.") {
            if let Ok(entry) = entry {
                unix::fs::symlink(
                    entry.path(),
                    out_dir
                        .unwrap_or(Path::new("~/"))
                        .join(format!(".{:?}", &entry.file_name())),
                )?;
            }
        }
        Ok(())
    }
}
