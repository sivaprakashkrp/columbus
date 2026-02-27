use std::{fs, path::Path, vec::IntoIter};
use chrono::{DateTime, Local};
use hf::is_hidden;
use is_executable::IsExecutable;

use crate::{explorer::{EntryType, FileEntry}, file_size_deps::find_length};


fn get_files(path: &Path, directory_size: bool, byte_size: bool) -> Vec<FileEntry> {
    let mut data = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path) {
        let mut dir_index: usize = 0;
        for entry in read_dir {
            if let Ok(file) = entry {
                map_data(file, &mut data, &mut dir_index, directory_size, byte_size);
            }
        }
    }
    data
}

// To get the data about the files and directories in the given path
pub fn get_data(path: &Path, all:bool, hiddenonly: bool, directory_size: bool, byte_size: bool) -> Result<Vec<FileEntry>, String> {
    let mut get_files = get_files(path, directory_size, byte_size);
    if hiddenonly {
        let get_files_iter: IntoIter<FileEntry> = get_files.into_iter();
        get_files = only_hidden(get_files_iter);
    } else if !all {
        let get_files_iter: IntoIter<FileEntry> = get_files.into_iter();
        get_files = leave_hidden(get_files_iter);
    }
    if get_files.is_empty() {
        return Err(String::from("No Files or Directories found!"));
    }
    Ok(get_files)
}

// To collect data about directories and map them into a vector so that they can be displayed in table
fn map_dir_data(file: fs::DirEntry, data: &mut Vec<FileEntry>, dir_index: &mut usize, directory_size: bool, byte_size: bool) -> fs::DirEntry {
    if let Ok(meta) = fs::metadata(file.path()) {
        if meta.is_dir() {
            data.insert(*dir_index, FileEntry {
                name: file
                    .file_name()
                    .into_string()
                    .unwrap_or("unknown name".into()),
                e_type: EntryType::Dir,
                size: find_length(&file.path(), directory_size, byte_size),
                modified_at: if let Ok(mod_time) = meta.modified() {
                    let date: DateTime<Local> = mod_time.into();
                    format!("{}", date.format("%b %e %Y %H:%M"))
                } else {
                    String::default()
                },
                hidden: is_hidden(file.path()).unwrap_or(false),
                is_exec: file.path().is_executable(),
            });
            *dir_index += 1;
        }
    }
    file
}

// To collect data about files and map them into a vector so that they can be displayed in table
fn map_file_data(file: fs::DirEntry, data: &mut Vec<FileEntry>, byte_size: bool) {
    if let Ok(meta) = fs::metadata(file.path()) {
        if !meta.is_dir() {
            data.push(FileEntry {
                name: file
                    .file_name()
                    .into_string()
                    .unwrap_or("unknown name".into()),
                e_type: EntryType::File,
                size: find_length(&file.path(), false, byte_size),
                modified_at: if let Ok(mod_time) = meta.modified() {
                    let date: DateTime<Local> = mod_time.into();
                    format!("{}", date.format("%b %e %Y %H:%M"))
                } else {
                    String::default()
                },
                hidden: is_hidden(file.path()).unwrap_or(false),
                is_exec: file.path().is_executable(),
            });
        }
    }
}

// Calling map_dir_data and map_file_data
// This order of calling the methods is what displays Directories first, then the files in the table
fn map_data(file: fs::DirEntry, data: &mut Vec<FileEntry>, dir_index: &mut usize, directory_size: bool, byte_size: bool) {
    let re_arg = map_dir_data(file, data, dir_index, directory_size, byte_size);
    map_file_data(re_arg, data, byte_size);
}

// To omit hidden files from the Vector
fn leave_hidden<I>(data: I) -> Vec<FileEntry> where I: Iterator<Item = FileEntry> {
    let res: Vec<FileEntry> = data.filter(|x| !x.hidden).collect();
    res
}

// To have only the hidden files in the Vector
fn only_hidden<I>(data: I) -> Vec<FileEntry> where I: Iterator<Item = FileEntry> {
    let res: Vec<FileEntry> = data.filter(|x| x.hidden).collect();
    res
}