pub struct Spotify {
    pub command: Option<String>,
}

impl Spotify {
    pub fn new(command: Option<String>) -> Self {
        Self { command }
    }

    pub fn play(&self, song: String) {
        println!("Playing {} on spotify...", song);
    }

    pub fn next(&self) {
        println!("Skipping to next song...");
    }

    pub fn previous(&self) {
        println!("Going back to previous song...");
    }

    pub fn help(&self) {
        println!("===== Command Lists =====");
        println!("- play: spotify.play <SONG>");
        println!("- next: spotify.next");
        println!("- previous: spotify.previous");
        println!("=========================");
    }
}