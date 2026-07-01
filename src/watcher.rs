use std::fs::File;
use std::io::{self, BufRead as _, BufReader, Read as _, Seek as _, SeekFrom};
use std::path::Path;
use std::thread;
use std::time::Duration;

pub struct Watcher<PreHook> {
    reader: BufReader<File>,
    interval: Duration,
    attempts: u8,
    pre_hook: Option<PreHook>,
}

pub enum Status {
    PreHook,
    Success,
    Failure,
    TimeOut,
}

impl Watcher<fn() -> anyhow::Result<bool>> {
    pub fn open<P>(path: P) -> io::Result<Self>
    where
        P: AsRef<Path>,
    {
        File::open(path).map(BufReader::new).map(|reader| Self {
            reader,
            interval: Duration::from_secs(1),
            attempts: 60,
            pre_hook: None,
        })
    }
}

#[expect(dead_code)]
impl<PreHook> Watcher<PreHook> {
    pub fn with_interval(self, interval: Duration) -> Self {
        Self { interval, ..self }
    }

    pub fn with_attempts(self, attempts: u8) -> Self {
        Self { attempts, ..self }
    }
}

impl<PreHook> Watcher<PreHook>
where
    PreHook: FnOnce() -> anyhow::Result<bool> + Copy,
{
    pub fn poll<F>(&mut self, handler: F) -> anyhow::Result<Status>
    where
        F: Fn(&str) -> Option<Status>,
    {
        let mut position = 0;

        for _ in 0..self.attempts {
            if let Some(pre_hook) = &self.pre_hook
                && pre_hook()?
            {
                return Ok(Status::PreHook);
            }

            self.reader.seek(SeekFrom::Start(position))?;

            let lines = self
                .reader
                .by_ref()
                .lines()
                .collect::<io::Result<Vec<String>>>()?;

            for line in &lines {
                if let Some(status) = handler(line) {
                    return Ok(status);
                }

                position = self.reader.stream_position()?;
            }

            thread::sleep(self.interval);
        }

        Ok(Status::TimeOut)
    }
}

impl<OldHook> Watcher<OldHook> {
    pub fn with_pre_hook<PreHook>(self, pre_hook: PreHook) -> Watcher<PreHook>
    where
        PreHook: FnOnce() -> anyhow::Result<bool>,
    {
        Watcher {
            reader: self.reader,
            interval: self.interval,
            attempts: self.attempts,
            pre_hook: Some(pre_hook),
        }
    }
}
