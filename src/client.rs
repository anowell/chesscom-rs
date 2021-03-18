use crate::api;
use chesscom_openapi::apis::configuration::Configuration;
use chesscom_openapi::apis::default_api::*;

use crate::error::Result;

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

/// API client for chess.com
pub struct ChessApi {
    config: Configuration,
}

impl ChessApi {
    pub fn new() -> ChessApi {
        let mut config = Configuration::new();
        config.user_agent = Some(format!("{} chesscom-rs", USER_AGENT));

        ChessApi { config }
    }

    /// Get profile details about a user
    pub async fn get_profile(&self, username: &str) -> Result<api::Profile> {
        get_player_profile(&self.config, username).await
    }

    /// Check if the user has been online in the last 5 minutes
    pub async fn is_player_online(&self, username: &str) -> Result<bool> {
        get_player_online_status(&self.config, username)
            .await
            .map(|resp| resp.online)
    }

    /// Get a list of titled-player users
    pub async fn get_titled_players(&self, title: api::Title) -> Result<Vec<String>> {
        get_titled_players(&self.config, title).await
    }
    
    /// Get ratings, win/loss, and other stats about a player's game play, tactics, lessons and Puzzle Rush score.
    pub async fn get_player_stats(&self, username: &str) -> Result<api::PlayerStats> {
        get_player_stats(&self.config, username).await
    }

    /// Get list of a player's current daily games
    pub async fn get_daily_chess_games(&self, username: &str) -> Result<Vec<api::DailyGame>> {
        get_daily_chess_games(&self.config, username)
            .await
            .map(|resp| resp.games)
    }

    /// Get a list of games that are waiting on a specific player to move
    pub async fn get_waiting_chess_games(&self, username: &str) -> Result<Vec<api::ToMoveGame>> {
        get_daily_chess_games_to_move(&self.config, username)
            .await
            .map(|resp| resp.games)
    }


}
