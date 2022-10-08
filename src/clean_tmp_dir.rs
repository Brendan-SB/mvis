use std::{fs, path::Path};

pub fn clean_tmp_dir<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if entry.file_type()?.is_dir() {
            let result = clean_tmp_dir(&path);

            fs::remove_dir(path)?;

            return result;
        } else {
            fs::remove_file(path)?;
        }
    }

    Ok(())
}
