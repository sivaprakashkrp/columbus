use std::{fs::read_dir, path::Path};
use std::{cmp, io};

// from the crate fs_extra
fn get_size<P>(path: P) -> io::Result<u64>
where
    P: AsRef<Path>,
{
    // Using `fs::symlink_metadata` since we don't want to follow symlinks,
    // as we're calculating the exact size of the requested path itself.
    let path_metadata = path.as_ref().symlink_metadata()?;

    let mut size_in_bytes: u64 = 0;

    if path_metadata.is_dir() {
        for entry in read_dir(&path)? {
            let entry = entry?;
            let entry_metadata = entry.metadata()?;

            if entry_metadata.is_dir() {
                // The size of the directory entry itself will be counted inside the `get_size()` call,
                // so we intentionally don't also add `entry_metadata.len()` to the total here.
                size_in_bytes += get_size(entry.path())?;
            } else {
                size_in_bytes += entry_metadata.len();
            }
        }
    } else {
        size_in_bytes = path_metadata.len();
    }

    Ok(size_in_bytes)
}

// To convert the length of the files from Byte information to respective file length unit
pub fn convert(num: f64) -> String {
  let negative = if num.is_sign_positive() { "" } else { "-" };
  let num = num.abs();
  let units = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
  if num < 1_f64 {
      return format!("{}{} {}", negative, num, "B");
  }
  let delimiter = 1024_f64;
  let exponent = cmp::min((num.ln() / delimiter.ln()).floor() as i32, (units.len() - 1) as i32);
  let pretty_bytes = format!("{:.2}", num / delimiter.powi(exponent)).parse::<f64>().unwrap_or(-1.0) * 1_f64;
  let unit = units[exponent as usize];
  format!("{}{} {}", negative, pretty_bytes, unit)
}

pub fn find_length(path: &Path, directory_size: bool, byte_size: bool) -> String {
    if let Ok(metadata) = path.symlink_metadata() {
        let mut bytes: u64 = metadata.len();
        if directory_size && metadata.is_dir() {
            bytes = get_size(path).unwrap_or(0_u64);
        } else if !directory_size && metadata.is_dir() {
            return String::from("...");
        }
        if byte_size {
            return bytes.to_string();
        }
        convert(bytes as f64)
    } else {
        String::from("Nan B")
    }
}