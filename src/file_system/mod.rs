use std::{fs, io, path::Path, process};

pub fn create_empty_directory<At: AsRef<Path>>(path: At) -> io::Result<()> {
    fs::create_dir(path)?;
    Ok(())
}

pub fn copy_fs_objects<From: AsRef<Path>, To: AsRef<Path>>(
    from: From,
    to: To,
    exclude: &Vec<String>,
) -> io::Result<()> {
    let source_type = fs::metadata(&from)?;
    let dest_meta = fs::metadata(&to);

    if dest_meta.is_err() {
        fs::create_dir_all(&to)?;
    }

    let dest_type = fs::metadata(&to)?;

    // Check if the provided source is a directory whilst the destinations is a file or symlink
    if source_type.is_dir() && (dest_type.is_file()) {
        // Print error & exit the process
        eprintln!("The destination cannot be a file whilst the source is a directory.");
        process::exit(1);
    }

    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let object_type = entry.file_type()?;
        let object_path = entry.path();
        if exclude.contains(&object_path.to_str().unwrap().to_string()) {
            continue;
        };
        if object_type.is_dir() {
            copy_fs_objects(object_path, to.as_ref().join(entry.file_name()), &exclude)?;
        } else {
            fs::copy(object_path, to.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}
