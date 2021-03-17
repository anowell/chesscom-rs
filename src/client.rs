use crate::api;
use chesscom_openapi::apis::configuration::Configuration;
use chesscom_openapi::apis::default_api::{
    pub_player_username_get, pub_player_username_is_online_get, pub_player_username_stats_get,
    pub_titled_title_get,
};

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
        pub_player_username_get(&self.config, username).await
    }

    /// Check if the user has been online in the last 5 minutes
    pub async fn is_user_online(&self, username: &str) -> Result<bool> {
        pub_player_username_is_online_get(&self.config, username)
            .await
            .map(|resp| resp.online)
    }

    /// Get a list of titled-player users
    pub async fn get_titled_profiles(&self, title: api::Title) -> Result<Vec<String>> {
        pub_titled_title_get(&self.config, title).await
    }
    
    /// Get ratings, win/loss, and other stats about a player's game play, tactics, lessons and Puzzle Rush score.
    pub async fn get_user_stats(&self, username: &str) -> Result<api::PlayerStats> {
        pub_player_username_stats_get(&self.config, username).await
    }
}
