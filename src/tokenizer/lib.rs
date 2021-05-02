
struct MultiLineTokenizer<'a> {
    multiline_data: String,
    newline: &'a str,
    timeout: time::Duration,
    created: Option<time::Instant>,
    inner: Arc<MultiLineTokenizer<'a>>,
}


impl<'a> MultiLineTokenizer<'a> {
    async fn new(newline: &'a str) -> Self {
        let entity = MultiLineTokenizer {
            multiline_data: "".to_string(),
            newline,
            timeout: time::Duration::new(3, 0),
            created: None,
            inner: Arc::new(Self),
        };
        entity.watchdog_start();
        entity
    }

    fn dump_data(&mut self) {
        DummyPrintStorage::store(
            LogEvent {
                created: self.created.unwrap_or(time::Instant::now()),
                enity: "".to_string(),
                data: self.multiline_data.clone(),
            });

        self.created = None;
        self.multiline_data = "".to_string();
    }

    pub fn append(&mut self, line: &str) {
        if line.starts_with(self.newline) {
            self.dump_data();
            self.created = Some(time::Instant::now());
            self.multiline_data = line.to_string();
        }
        self.multiline_data = format!("{}\n{}", self.multiline_data, line);
    }

    async fn watchdog_task(&mut self) {
        loop {
            sleep(Duration::from_secs(1)).await;
            match self.created {
                Some(c) => if c.elapsed() > self.timeout {
                    self.dump_data()
                }
                _ => continue
            }
        }
    }
}
