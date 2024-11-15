use std::env;

#[cfg(target_os = "windows")]
const ALIASES: [&str; 1] = ["win"];
#[cfg(target_os = "macos")]
const ALIASES: [&str; 3] = ["osx", "mac", "darwin"];
#[cfg(target_os = "linux")]
const ALIASES: [&str; 1] = ["linux"];

pub fn get_compatible_file_index(files: &[String]) -> Option<usize> {
    let arch = env::consts::ARCH;

    let specific_match = files.iter().enumerate().find_map(|(idx, file)| {
        if ALIASES.iter().any(|alias| file.contains(alias)) {
            if (arch == "x86_64" && file.contains("x64"))
                || (arch == "aarch64" && file.contains("arm64"))
                || (arch == "x86" && file.contains("x86"))
            {
                return Some(idx);
            }
        }
        None
    });

    specific_match.or_else(|| files.iter().position(|file| file.contains("generic")))
}
