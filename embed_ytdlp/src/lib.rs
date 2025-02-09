extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use reqwest::blocking;
use std::{env, fs::File, io::Write};

#[proc_macro]
pub fn embed_ytdlp(_: TokenStream) -> TokenStream {
    const BASE_URL: &str = "https://github.com/yt-dlp/yt-dlp/releases/latest/download";
    const TEMP_BIN_NAME: &str = "____ytdlp_";
    const YTDLP_TEMP_PATH: &str = "YTDLP_TEMP_PATH";

    let binary_filename = if cfg!(target_os = "windows") {
        "yt-dlp.exe"
    } else if cfg!(target_os = "linux") {
        "yt-dlp"
    } else if cfg!(target_os = "macos") {
        "yt-dlp_macos"
    } else {
        panic!("Unsupported OS")
    };
    let binary_url = format!("{}/{}", BASE_URL, binary_filename);

    let temp_path = env::temp_dir().join(TEMP_BIN_NAME);
    if temp_path.to_str().is_none() {
        panic!("Failed to convert path to string");
    }

    env::set_var(YTDLP_TEMP_PATH, temp_path.to_str().unwrap_or_default());

    print!(
        "{:#?}, YTDLP_TEMP_PATH:{:#?}",
        temp_path.to_str(),
        std::env::var("YTDLP_TEMP_PATH").unwrap_or_default()
    );

    if !temp_path.exists() {
        let resp = blocking::get(&binary_url).expect("Failed to download ytdlp binary");
        let bytes = resp.bytes().expect("Failed to read response bytes");

        let mut file = File::create(&temp_path).expect("Failed to create file");
        file.write_all(&bytes).expect("Failed to write to file");
        // file.flush().expect("Failed to flush file");

        // Make executable on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = file
                .metadata()
                .expect("Failed to get metadata")
                .permissions();
            perms.set_mode(0o755);

            file.set_permissions(perms)
                .expect("Failed to set permissions");
        }
    }

    let temp_path_str = temp_path
        .to_str()
        .expect("Failed to convert path to string");

    let output = quote! {
        const EMBEDDED_YTDLP: &[u8] = include_bytes!(#temp_path_str);
    };

    output.into()
}
