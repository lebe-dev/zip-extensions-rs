use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;

use zip::{CompressionMethod, write, ZipWriter};
use zip::result::ZipResult;
use zip::write::FileOptions;

use crate::file_utils::make_relative_path;

/// Creates a zip archive that contains the files and directories from the specified directory.
pub fn zip_create_from_directory(archive_file: &PathBuf, directory: &PathBuf, include_dir_in_path: bool) -> ZipResult<()> {
    let options = write::FileOptions::default().compression_method(CompressionMethod::Stored);
    zip_create_from_directory_with_options(archive_file, directory, options, include_dir_in_path)
}

/// Creates a zip archive that contains the files and directories from the specified directory, uses the specified compression level.
pub fn zip_create_from_directory_with_options(
    archive_file: &PathBuf,
    directory: &PathBuf,
    options: FileOptions,
    include_dir_in_path: bool
) -> ZipResult<()> {
    let file = File::create(archive_file)?;
    let mut zip_writer = zip::ZipWriter::new(file);
    zip_writer.create_from_directory_with_options(directory, options, include_dir_in_path)
}

pub trait ZipWriterExtensions {
    /// Creates a zip archive that contains the files and directories from the specified directory.
    ///
    /// `include_dir_in_path` - add target dir into destination hierarchy.
    /// I.e. `/media/cd1` -> `cd1/...`
    fn create_from_directory(&mut self, directory: &PathBuf,
                             include_dir_in_path: bool) -> ZipResult<()>;

    /// Creates a zip archive that contains the files and directories from the specified directory,
    /// uses the specified compression level.
    ///
    /// `include_dir_in_path` - add target dir into destination hierarchy.
    /// I.e. `/media/cd1` -> `cd1/...`
    fn create_from_directory_with_options(
        &mut self,
        directory: &PathBuf,
        options: FileOptions,
        include_dir_in_path: bool
    ) -> ZipResult<()>;
}

impl<W: Write + io::Seek> ZipWriterExtensions for ZipWriter<W> {
    fn create_from_directory(&mut self, directory: &PathBuf,
                             include_dir_in_path: bool) -> ZipResult<()> {
        let options = write::FileOptions::default()
            .compression_method(CompressionMethod::Stored);
        self.create_from_directory_with_options(directory, options, include_dir_in_path)
    }

    fn create_from_directory_with_options(
        &mut self,
        directory: &PathBuf,
        options: FileOptions,
        include_dir_in_path: bool
    ) -> ZipResult<()> {
        let mut paths_queue: Vec<PathBuf> = vec![];
        paths_queue.push(directory.clone());

        let mut buffer = Vec::new();

        while let Some(next) = paths_queue.pop() {
            let directory_entry_iterator = std::fs::read_dir(next)?;

            for entry in directory_entry_iterator {
                let entry_path = entry?.path();
                let entry_metadata = std::fs::metadata(entry_path.clone())?;
                if entry_metadata.is_file() {
                    let mut f = File::open(&entry_path)?;
                    f.read_to_end(&mut buffer)?;
                    let relative_path = make_relative_path(&directory, &entry_path, include_dir_in_path);
                    self.start_file_from_path(&relative_path, options)?;
                    self.write_all(buffer.as_ref())?;
                    buffer.clear();
                } else if entry_metadata.is_dir() {
                    let relative_path = make_relative_path(&directory, &entry_path, include_dir_in_path);
                    self.add_directory_from_path(&relative_path, options)?;
                    paths_queue.push(entry_path.clone());
                }
            }
        }

        self.finish()?;
        Ok(())
    }
}
