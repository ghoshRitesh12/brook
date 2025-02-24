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

    // sync ytdlp version and start the subprocess
    YTDLP.sync_version().start();

    YTDLP.exec(vec![
        "https://www.youtube.com/watch?v=mrV3lSBiGAs",
        "--get-url",
        "-f",
        "bestaudio",
    ]);

    // println!("__, YTDLP_TEMP_PATH: -> {:#?}", YTDLP);
}
