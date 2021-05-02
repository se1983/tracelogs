use log::info;
use std::fs;
use std::os::linux::fs::MetadataExt;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone)]
struct Process {
    pid: usize,
    std_out_path: PathBuf,
    exe: String,
}

impl Process {
    fn new(pid: usize) -> Self {
        let std_out_path = Path::new("/proc").join(format!("{}", &pid)).join("fd/1");

        fn get_binary_link(pid: &usize) -> String {
            let bin_link = Path::new("/proc").join(format!("{}", &pid)).join("exe");
            match fs::read_link(bin_link) {
                Err(_) => String::from("None"),
                Ok(x) => x.into_os_string().into_string().unwrap(),
            }
        }

        let exe: String = get_binary_link(&pid);

        Process {
            pid,
            std_out_path,
            exe,
        }
    }

    async fn follow_logs(&self) {
        let mut pos: usize = 0;
        while self.std_out_path.exists() {
            sleep(Duration::from_millis(300)).await;
            let mut contents = String::new();
            let mut file = match File::open(&self.std_out_path).await {
                Ok(f) => f,
                _ => continue,
            };
            match file.read_to_string(&mut contents).await {
                Ok(_) => {}
                _ => continue,
            };
            for (i, line) in contents.lines().enumerate() {
                if i <= pos {
                    continue;
                }

                info!("[{}] {}: {}", self.pid, self.exe, line);
                pos += 1;
            }
        }
    }
}

async fn extract_processes(known: Vec<usize>) -> Vec<Process> {
    let mut procs = Vec::new();
    let mut dir = tokio::fs::read_dir("/proc").await.unwrap();
    let uid = fs::metadata("/proc/self").unwrap().st_uid();

    while let Some(proc_dir) = dir.next_entry().await.unwrap() {
        let pid = proc_dir.file_name().into_string().unwrap();
        let pid = match pid.parse::<usize>() {
            Ok(pid) => pid,
            _ => continue,
        };
        if known.contains(&pid) {
            continue;
        }
        match fs::metadata(proc_dir.path()) {
            Ok(m) if m.st_uid() == uid => m,
            _ => continue,
        };
        procs.push(Process::new(pid))
    }
    procs
}

pub async fn watch() {
    let mut hist: Vec<usize> = Vec::new();
    loop {
        for new_process in extract_processes(hist.clone()).await {
            let pid = new_process.pid.clone();
            tokio::spawn(async move {
                new_process.follow_logs().await;
            });
            hist.push(pid);
        }
        sleep(Duration::from_millis(5000)).await;
    }
}
