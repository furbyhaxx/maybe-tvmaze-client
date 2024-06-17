mod show_main_info;
mod season;
mod episode;
mod show;

/// Id to be used on show lookup
pub enum ShowId {
    Imdb(String),
    Tvdb(i64),
    TvMaze(i64),
}

pub use show_main_info::*;
pub use show::Show;
pub use season::Season;
pub use episode::Episode;