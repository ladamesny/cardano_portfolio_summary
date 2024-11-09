use std::time::Duration;
use tokio::time::sleep;

pub struct Spinner {
    frames: Vec<&'static str>,
    current: usize,
}

impl Spinner {
    pub fn new() -> Self {
        Self {
            frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            current: 0,
        }
    }

    pub fn next(&mut self) -> &str {
        let frame = self.frames[self.current];
        self.current = (self.current + 1) % self.frames.len();
        frame
    }

    pub async fn spin_while<F, T>(message: &str, future: F) -> T 
    where
        F: std::future::Future<Output = T>,
    {
        let mut spinner = Spinner::new();
        let message_len = message.len();
        let message = message.to_string();
        let spinner_handle = tokio::spawn(async move {
            loop {
                print!("\r{} {}", spinner.next(), message);
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                sleep(Duration::from_millis(100)).await;
            }
        });

        let result = future.await;
        spinner_handle.abort();
        print!("\r{}\r", " ".repeat(message_len + 2));
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        
        result
    }

    pub async fn spin_with_nav<F, T>(future: F) -> T 
    where
        F: std::future::Future<Output = T>,
    {
        // Save cursor position and clear navigation line
        print!("\x1B[s\x1B[{};0H\x1B[K", termion::terminal_size().unwrap().1);
        
        let result = Self::spin_while("Loading", future).await;
        
        // Restore cursor position
        print!("\x1B[u");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        
        result
    }
} 