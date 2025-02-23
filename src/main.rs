use once_cell::sync::Lazy;
use ytdlp::YTDlp;

mod ytdlp;

const BROOK_DIR_NAME: &str = ".brook";
const ASCII_ART: &str = r#"
 _                     _
| |                   | |
| |__  ____ ___   ___ | | __
|  _ \|  __/ _ \ / _ \| |/ /
| |_) | | | (_) | (_) |   <
|____/|_|  \___/ \___/|_|\_\
"#;

pub static YTDLP: Lazy<YTDlp> = ytdlp::init();

fn main() {
    println!("{ASCII_ART}");

    // regardless of how many times this is called,
    // it will only be executed once
    YTDLP.sync_version();
    YTDLP.sync_version();

    // println!("__, YTDLP_TEMP_PATH: -> {:#?}", YTDLP);
}
