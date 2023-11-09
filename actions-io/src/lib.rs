use std::fs::{self, File, OpenOptions};
use std::path::{Path, PathBuf};
use std::os::windows::prelude::*;
use std::os::windows::fs::{OpenOptionsExt, MetadataExt};
use std::os::unix::fs::MetadataExt as UnixMetadataExt;
use std::io;
use std::os::windows::io::AsRawHandle;
use std::os::unix::io::AsRawFd;
use std::os::raw::c_int;

#[cfg(windows)]
use winapi::um::fileapi::*;
#[cfg(windows)]
use winapi::um::handleapi::*;
#[cfg(windows)]
use winapi::um::errhandlingapi::*;
#[cfg(windows)]
use winapi::shared::minwindef::DWORD;

#[cfg(unix)]
use libc::*;

pub const IS_WINDOWS: bool = cfg!(windows);

#[cfg(windows)]
pub const UV_FS_O_EXLOCK: DWORD = 0x10000000;

#[cfg(unix)]
pub const UV_FS_O_EXLOCK: c_int = 0x10000000;

pub async fn chmod<P: AsRef<Path>>(path: P, mode: u32) -> io::Result<()> {
    #[cfg(windows)]
    {
        unimplemented!();
    }
    #[cfg(unix)]
    {
        fs::set_permissions(path, fs::Permissions::from_mode(mode)).await
    }
}

pub async fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<u64> {
    fs::copy(from, to).await
}

pub async fn lstat<P: AsRef<Path>>(path: P) -> io::Result<fs::Metadata> {
    fs::symlink_metadata(path).await
}

pub async fn mkdir<P: AsRef<Path>>(path: P, mode: u32) -> io::Result<()> {
    #[cfg(windows)]
    {
        unimplemented!();
    }
    #[cfg(unix)]
    {
        fs::create_dir(path).await
    }
}

pub async fn open<P: AsRef<Path>>(path: P, options: OpenOptions) -> io::Result<File> {
    options.open(path)
}

pub async fn readdir<P: AsRef<Path>>(path: P) -> io::Result<Vec<PathBuf>> {
    let entries: Vec<io::Result<PathBuf>> = fs::read_dir(path)
        .await?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect();

    let entries = entries.into_iter().collect::<io::Result<Vec<PathBuf>>>()?;
    Ok(entries)
}

pub async fn readlink<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    #[cfg(windows)]
    {
        unimplemented!();
    }
    #[cfg(unix)]
    {
        fs::read_link(path).await
    }
}

pub async fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<()> {
    fs::rename(from, to).await
}

pub async fn remove_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fs::remove_file(path).await
}

pub async fn remove_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fs::remove_dir(path).await
}

pub async fn stat<P: AsRef<Path>>(path: P) -> io::Result<fs::Metadata> {
    fs::metadata(path).await
}

pub async fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    #[cfg(windows)]
    {
        unimplemented!();
    }
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(src, dst)
    }
}

pub async fn unlink<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fs::remove_file(path).await
}

pub async fn exists<P: AsRef<Path>>(path: P) -> bool {
    stat(path).await.is_ok()
}

pub async fn is_directory<P: AsRef<Path>>(path: P) -> io::Result<bool> {
    let metadata = lstat(path).await?;
    Ok(metadata.is_dir())
}

pub fn is_rooted(p: &str) -> bool {
    let p = normalize_separators(p);
    if p.is_empty() {
        panic!("is_rooted() parameter \"p\" cannot be empty");
    }

    if IS_WINDOWS {
        p.starts_with('\\') || p.chars().nth(1) == Some(':')
    } else {
        p.starts_with('/')
    }
}

pub async fn try_get_executable_path<P: AsRef<Path>>(
    file_path: P,
    extensions: &[&str],
) -> io::Result<PathBuf> {
    let original_file_path = file_path.as_ref();

    if stat(original_file_path).await.is_ok() {
        return Ok(original_file_path.to_path_buf());
    }

    for extension in extensions {
        let mut file_path_with_extension = original_file_path.to_path_buf();
        file_path_with_extension.push(extension);

        if stat(&file_path_with_extension).await.is_ok() {
            return Ok(file_path_with_extension);
        }
    }

    Err(io::Error::new(io::ErrorKind::NotFound, "Executable not found"))
}

fn normalize_separators(p: &str) -> String {
    let p = p.to_string();
    if IS_WINDOWS {
        p.replace("/", "\\").replace("\\\\+", "\\")
    } else {
        p.replace("//+", "/")
    }
}

#[cfg(unix)]
fn is_unix_executable(metadata: fs::Metadata) -> bool {
    let mode = metadata.mode();
    (mode & 0o111) > 0
        || (mode & 0o1000) > 0 && unsafe { getgid() } == metadata.gid()
        || (mode & 0o100) > 0 && unsafe { getuid() } == metadata.uid()
}

#[cfg(windows)]
fn is_unix_executable(_metadata: fs::Metadata) -> bool {
    false
}

pub fn get_cmd_path() -> &'static str {
    std::env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".to_string())
}
