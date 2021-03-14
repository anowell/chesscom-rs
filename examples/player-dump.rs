use chesscom::{ChessApi, api::ChessStats};
use chrono::{Duration, Utc};
use std::{env, process};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn exit(msg: &str) -> ! {
    eprintln!("{}", msg);
    process::exit(1);
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = env::args();
    args.next(); // discard args[0]
    let username = match args.next() {
        Some(arg) => arg,
        None => exit("USAGE: profile <username>"),
    };

    let chess_api = ChessApi::new();
    let profile = chess_api.get_profile(&username).await?;
    println!("Username: {}", profile.username);
    println!("Status: {}", profile.status);
    println!("Name: {}", profile.name.as_deref().unwrap_or("<Unknown>"));
    println!("Joined: {}", profile.joined.date().format("%Y-%m-%d"));

    let time_since_online = Utc::today() - profile.last_online.date();

    if time_since_online < Duration::hours(1) {
        println!("Last Online: {} mins ago", time_since_online.num_minutes());
    } else if time_since_online < Duration::days(1) {
        println!("Last Online: {} hours ago", time_since_online.num_hours());
    } else {
        println!("Last Online: {} days ago", time_since_online.num_days());
    }

    let stats = chess_api.get_user_stats(&username).await?;
    if let Some(stats) = stats.chess_daily {
        print_stats("Daily", &stats);
    }
    if let Some(stats) = stats.chess_rapid {
        print_stats("Rapid", &stats);
    }
    if let Some(stats) = stats.chess_blitz {
        print_stats("Blitz", &stats);
    }
    if let Some(stats) = stats.chess_bullet {
        print_stats("Bullet", &stats);
    }

    Ok(())
}

fn print_stats(label: &str, stats: &ChessStats) {
    println!(
        "{}: {} rating (rd={}) with {}-{}-{} record",
        label,
        stats.last.rating,
        stats.last.rd,
        stats.record.win,
        stats.record.loss,
        stats.record.draw
    );
}
