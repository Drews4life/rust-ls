use std::error::Error;
use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt;
use std::fs;
use std::time::SystemTime;
use chrono::{DateTime, Local};
use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
use std::fs::DirEntry;

pub struct Ls {}

impl Ls {
    pub fn run (&self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        if path.is_dir() {
            //Reading the directory and getting
            for entry in fs::read_dir(path)? {
                self.print_entry(entry?);
            }
        }
        Ok(())
    }

    fn print_entry(&self, entry: DirEntry) -> Result<(), Box<dyn Error>>{
        //Getting metadata, may return an error
        let metadata = entry.metadata()?;
        let file_name = entry
            .file_name()
            .into_string()
            .or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;

        let file_size = metadata.len();
        let mode = metadata.permissions().mode();

        println!(
            "{} {:>5} {} {}",
            self.get_file_permissions(mode as u32),
            file_size,
            self.format_time(metadata.modified()?),
            file_name
        );

        Ok(())
    }

    fn format_time(&self, modified_time: SystemTime) -> String {
        let modification_date: DateTime<Local> = DateTime::from(modified_time);
        modification_date.format("%_d %b %H:%M").to_string()
    }

    fn get_file_permissions(&self, mode: u32) -> String {
        let user = self.triplet(mode, S_IRUSR, S_IWUSR, S_IXUSR);
        let group = self.triplet(mode, S_IRGRP, S_IWGRP, S_IXGRP);
        let other = self.triplet(mode, S_IROTH, S_IWOTH, S_IXOTH);
        [user, group, other].join("")
    }

    fn triplet(&self, mode: u32, read: u32, write: u32, execute: u32) -> String {
        match (mode & read, mode & write, mode & execute) {
            (0, 0, 0) => "---",
            (_, 0, 0) => "r--",
            (0, _, 0) => "-w-",
            (0, 0, _) => "--x",
            (_, 0, _) => "r-x",
            (_, _, 0) => "rw-",
            (0, _, _) => "-wx",
            (_, _, _) => "rwx",
        }.to_string()
    }
}