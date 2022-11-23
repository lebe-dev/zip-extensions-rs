use std::fs::File;
use std::io;
use std::io::{Error, ErrorKind, Write};
use std::path::{Component, PathBuf};

/// Writes all bytes to a file.
pub fn file_write_all_bytes(path: PathBuf, bytes: &[u8], overwrite: bool) -> io::Result<usize> {
    if path.exists() && !overwrite {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            "The specified file already exists.",
        ));
    }
    let mut file = File::create(path)?;
    file.set_len(0)?;
    file.write(bytes)
}

/// Returns a relative path from one path to another.
pub(crate) fn make_relative_path(root: &PathBuf, current: &PathBuf,
                                 include_dir_in_path: bool) -> PathBuf {
    let mut result = PathBuf::new();

    let mut root_components = root.components().collect::<Vec<Component>>();

    if include_dir_in_path {
        root_components.pop();
    }

    let current_components = current.components().collect::<Vec<_>>();

    for i in 0..current_components.len() {
        let current_path_component: Component = current_components[i];
        if i < root_components.len() {
            let other: Component = root_components[i];
            if other != current_path_component {
                break;
            }
        } else {
            result.push(current_path_component)
        }
    }
    result
}

#[cfg(test)]
mod make_relative_path_tests {
    use std::path::{Path, PathBuf};
    use std::str::FromStr;

    use crate::file_utils::make_relative_path;

    #[test]
    fn include_target_dir_test() {
        let root_path = Path::new("a").join("b");
        let current = PathBuf::new().join("a").join("b").join("c").join("d.jpg");
        let result = make_relative_path(&root_path, &current, true);

        let expected_path = Path::new("b").join("c").join("d.jpg");

        assert_eq!(expected_path, result);
    }
}
