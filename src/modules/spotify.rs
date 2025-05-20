pub struct Spotify {
    pub song: Option<String>,
}

impl Spotify {
    pub fn new(song: Option<String>) -> Self {
        Self { song }
    }

    pub fn play(&self) {
        match &self.song {
            Some(song) => println!("Playing {} on spotify...", song),
            None => eprintln!("Song can't be empty!"),
        }
    }

    pub fn next(&self) {
        println!("Skipping to next song...");
    }

    pub fn previous(&self) {
        println!("Going back to previous song...");
    }

    pub fn help(&self) {
        println!("===== Command Lists =====");
        println!("- play: spotify-play <SONG>");
        println!("- next: spotify-next");
        println!("- previous: spotify-previous");
        println!("=========================");
    }
}