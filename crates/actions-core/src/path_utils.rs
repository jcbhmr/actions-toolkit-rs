pub fn to_posix_path(path: &str) -> String {
    path.replace("\\", "/")
}

pub fn to_win32_path(path: &str) -> String {
    path.replace("/", "\\")
}

pub fn to_platform_path(path: &str) -> String {
    #[cfg(target_os = "windows")]
    return to_win32_path(path);
    #[cfg(not(target_os = "windows"))]
    return to_posix_path(path);
}
