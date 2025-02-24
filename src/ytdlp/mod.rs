use crate::BROOK_DIR_NAME;
use glob::glob;
use once_cell::sync::Lazy;
use reqwest::{
    blocking::{self, ClientBuilder},
    header::{HeaderValue, ACCEPT, USER_AGENT},
};
use scraper::{Html, Selector};
use std::{env, ffi::OsStr, fs, io::Write, path::PathBuf, sync::Once};

mod cmd;

static SYNC_VERSION: Once = Once::new();
pub const EXECUTABLE_NAME_PREFIX: &str = "_ytdlp_";

pub const fn init() -> Lazy<YTDlp> {
    return Lazy::new(|| YTDlp::new());
}

/// Interface to interact with ytdlp binary
#[derive(Debug)]
pub struct YTDlp {
    exec_name: String,
    latest_version: String,
    exec_path: String,
    brook_dir: PathBuf,
}

impl YTDlp {
    fn new() -> Self {
        let latest_version = Self::get_latest_release_tag();
        let exec_name = format!(
            "{EXECUTABLE_NAME_PREFIX}{}",
            latest_version.replace(".", "_")
        );
        let (brook_dir, exec_path) = Self::get_paths(&exec_name);

        return Self {
            exec_name,
            latest_version,
            exec_path,
            brook_dir,
        };
    }

    /// Syncs the latest ytdlp version \
    /// this method should only be called **once** per process
    pub fn sync_version(&self) -> &Self {
        SYNC_VERSION.call_once(|| self._sync_version());
        return self;
    }

    fn _sync_version(&self) {
        const YTDLP_DOWNLOAD_BASE_URL: &str = "https://github.com/yt-dlp/yt-dlp/releases/download";
        let mut latest_version_exists = false;
        let exec_filename = OsStr::new(&self.exec_name);

        println!("Syncing latest `ytdlp` version {}...", self.latest_version);

        if !self.brook_dir.exists() {
            fs::create_dir(&self.brook_dir)
                .expect("YTDlpSyncError: Failed to create brook directory");
        }

        if self.brook_dir.join(&self.exec_name).exists() {
            println!("Latest `ytdlp` version already exists");
            latest_version_exists = true;
        }

        let ytdlp_glob = glob(&format!(
            "{}",
            self.brook_dir
                .join(format!("{}*", EXECUTABLE_NAME_PREFIX))
                .to_str()
                .unwrap_or_default()
        ))
        .expect("YTDlpSyncError: Unable to remove stale ytdlp version(s)");

        // delete stale versions
        println!("Removing stale `ytdlp` versions");
        for entry in ytdlp_glob {
            if let Ok(path) = entry {
                let file = path.file_name().unwrap_or_default();
                if file == exec_filename {
                    continue;
                }

                fs::remove_file(path)
                    .expect("YTDlpSyncError: Unable to remove stale ytdlp version");
            }
        }

        self.add_ytdlp_to_path();

        if latest_version_exists {
            return;
        }

        let ytdlp_binary_name = if cfg!(target_os = "windows") {
            "yt-dlp.exe"
        } else if cfg!(target_os = "linux") {
            "yt-dlp"
        } else if cfg!(target_os = "macos") {
            "yt-dlp_macos"
        } else {
            panic!("YTDlpSyncError: Unsupported OS")
        };
        let ytdlp_binary_url = format!(
            "{}/{}/{ytdlp_binary_name}",
            YTDLP_DOWNLOAD_BASE_URL, self.latest_version
        );

        println!("Downloading latest `ytdlp` version, please wait...");

        let resp = blocking::get(ytdlp_binary_url)
            .expect("YTDlpSyncError: Failed to download latest ytdlp binary");
        let ytdlp_binary = resp
            .bytes()
            .expect("YTDlpSyncError: Failed to read ytdlp bytes");

        let mut file = fs::File::create(&self.exec_path)
            .expect("YTDlpSyncError: Failed to create ytdlp executable file");
        file.write_all(&ytdlp_binary)
            .expect("YTDlpSyncError: Failed to write ytdlp bytes to executable file");
        // file.flush().expect("Failed to flush file");

        // Make executable on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = file
                .metadata()
                .expect("YTDlpSyncError: Failed to get ytdlp file metadata")
                .permissions();
            perms.set_mode(0o755);

            file.set_permissions(perms)
                .expect("YTDlpSyncError: Failed to set ytdlp file permissions");
        }

        println!("Synced latest `ytdlp` version");
    }

    fn add_ytdlp_to_path(&self) {
        let mut path = env::var("PATH").unwrap_or_default();
        if path.contains(&self.exec_path) {
            return;
        }

        #[cfg(target_os = "windows")]
        {
            path.push(';');
        }
        #[cfg(unix)]
        {
            path.push(':');
        }

        path.push_str(&self.exec_path);
        env::set_var("PATH", path);
    }

    /// returns (brook_dir, exec_path)
    fn get_paths(exec_name: &str) -> (PathBuf, String) {
        let brook_dir = match home::home_dir() {
            Some(path) => {
                if path.as_os_str().is_empty() {
                    panic!("YTDlpSyncError: Unable to locate your home directory");
                }
                path.join(BROOK_DIR_NAME)
            }
            _ => panic!("YTDlpSyncError: Unable to find your home directory"),
        };

        let exec_path: String = brook_dir.join(exec_name).to_string_lossy().into();
        if exec_path.is_empty() {
            panic!("YTDlpSyncError: Unable to locate brook directory");
        }

        return (brook_dir, exec_path);
    }

    fn get_latest_release_tag() -> String {
        const URL: &str = "https://github.com/yt-dlp/yt-dlp/releases/latest";

        const ACCEPT_HEADER: &str = "text/html";
        const USER_AGENT_HEADER: &str =
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:133.0) Gecko/20100101 Firefox/133.0";

        let client = ClientBuilder::new()
            .default_headers(
                [
                    (USER_AGENT, HeaderValue::from_static(USER_AGENT_HEADER)),
                    (ACCEPT, HeaderValue::from_static(ACCEPT_HEADER)),
                ]
                .into_iter()
                .collect(),
            )
            .build()
            .expect("CientBuildError: Could'nt build client to get latest ytdlp release");

        let release_selector = &Selector::parse(
            "#repo-content-turbo-frame div nav ol li.breadcrumb-item.breadcrumb-item-selected a",
        )
        .expect("ParsingError: Failed to parse selector");

        let page = client
            .get(URL)
            .send()
            .expect("FetchError: Failed to get latest ytdlp release")
            .text()
            .expect("ParsingError: Failed to get latest ytdlp release");
        if page.is_empty() {
            panic!("ParsingError: Invalid ytdlp release information");
        }

        let doc = Html::parse_document(&page);
        let release_tag = doc
            .select(release_selector)
            .next()
            .and_then(|el| el.text().next())
            .map(|s| s.to_string())
            .unwrap_or_default();

        if release_tag.is_empty() {
            panic!("ParsingError: Invalid ytdlp release information");
        }

        return release_tag;
    }
}
