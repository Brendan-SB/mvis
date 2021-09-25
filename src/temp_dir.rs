use home::home_dir;
use std::{fs, path::Path};

pub fn try_create_temp_dir() {
    fs::create_dir_all(home_dir().unwrap().join(".local/mvis/tmp")).unwrap();
}

pub fn clean_temp_dir() -> Result<(), std::io::Error> {
    fn clean_temp_dir_inner<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if entry.file_type()?.is_dir() {
                let result = clean_temp_dir_inner(&path);

                fs::remove_dir(path)?;

                return result;
            } else {
                fs::remove_file(path)?;
            }
        }

        Ok(())
    }

    clean_temp_dir_inner(home_dir().unwrap().join(".local/mvis/tmp"))
}
