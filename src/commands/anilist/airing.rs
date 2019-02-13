use chrono::{Local, Weekday};
use std::ops::Add;
use time::Duration;

use serenity::framework::standard::{Args, Command, CommandError};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::commands::anilist::client;
use crate::models::anilist::airing_schedule::AiringSchedule;
use crate::utils::*;


pub struct AiringCommand;

impl Command for AiringCommand {
    fn execute(&self, _context: &mut Context, message: &Message, args: Args) -> Result<(), CommandError> {
        let (start, day) = if args.full().len() <= 0 {
            (to_midnight(Local::now()), "Today".to_owned())
        } else {
            let day = args.full();
            match day.parse::<Weekday>() {
                Ok(day) => {
                    (to_midnight(next_day(day)), weekday_to_string(day))
                }
                Err(_) => (to_midnight(Local::now()), "Today".to_owned())
            }
        };

        let results: Vec<AiringSchedule> = client::search_airing_schedule(start.timestamp(), start.add(Duration::days(1)).timestamp());

        if results.len() > 0 {
            let mut airing = vec![];

            for item in results {
                airing.push(item.to_url());
            }

            let _ = message.channel_id.send_message(|m| m
                .embed(|e| e
                    .color(3447003)
                    .title(format!("Airing Schedule for {}", day))
                    .description(airing.join("\n"))
                    .footer(|f| f
                        .icon_url("https://anilist.co/img/icons/favicon-32x32.png")
                        .text("Powered by AniList"))
                )
            );
        } else {
            let _ = message.channel_id.say(format!("Error checking the airing schedule"));
        }

        Ok(())
    }
}