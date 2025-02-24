use crate::ytdlp::YTDlp;
use std::sync::Once;
use tokio::process::Command;

static INIT: Once = Once::new();

impl YTDlp {
    /// Starts the ytdlp subprocess \
    /// this method should only be called **once** per process
    pub fn start(&self) {
        INIT.call_once(|| self.init_subprocess());
    }

    fn init_subprocess(&self) {
        println!("Hello from init subprocess");
    }

    pub fn exec(&self, args: Vec<&str>) {
        println!("Hello from exec");

        let mut child = Command::new(&self.exec_path)
            .args(args)
            .spawn()
            .expect("Failed to execute command");

        child.try_wait().expect("Failed to wait on child");
    }

    fn _read_output(&self) {
        println!("Hello from read output");
        todo!();
    }
}
