pub mod spotify;
pub mod task;
pub mod weather;

pub use spotify::Spotify;
pub use weather::Weather;
pub use task::Task;

pub enum Module {
    Weather,
    Spotify,
    Task,
}