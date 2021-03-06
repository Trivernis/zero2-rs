use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::prelude::{Message, UserId};
use serenity::prelude::Context;
use std::collections::HashSet;

use super::consts::{BOT_ID, PREFIX, PREFIXES};
use super::utils;
use crate::commands::{self, anilist, fun, meta, nekoslife, system, urban};
use crate::monitors;

pub struct Zero2Framework;

impl Zero2Framework {
    pub fn with_owners(owners: HashSet<UserId>) -> StandardFramework {
        StandardFramework::new()
            .configure(|c| {
                c.with_whitespace(true)
                    .allow_dm(true)
                    .on_mention(Some(UserId(BOT_ID)))
                    .ignore_bots(true)
                    .case_insensitivity(true)
                    .delimiters(vec![",", " "])
                    .owners(owners)
                    .prefix(PREFIX.as_str())
                    .prefixes(PREFIXES.to_vec())
            })
            .before(before)
            .after(after)
            .normal_message(|ctx, msg| {
                monitors::message_monitors(ctx, msg);
            })
            .bucket("stats_limit", |b| {
                b.delay(6 * 3600).time_span(24 * 3600).limit(4)
            })
            .help(&commands::ZERO2_HELP)
            .group(&anilist::ANILIST_GROUP)
            .group(&urban::KNOWLEDGE_GROUP)
            .group(&fun::FUN_GROUP)
            .group(&meta::META_GROUP)
            .group(&nekoslife::NEKOSLIFE_GROUP)
            .group(&system::SYSTEM_GROUP)
            .group(&commands::NOCATEGORY_GROUP)
            .on_dispatch_error(|_ctx, _msg, _err| {
                // USed for errors that happen before the command is ran
            })
    }
}

fn before(ctx: &mut Context, msg: &Message, cmd: &str) -> bool {
    if cmd != "shutdown" {
        let _ = msg.channel_id.broadcast_typing(&ctx.http);
    }

    if (cmd == "fortune" || cmd == "cookie") && is_trolling() {
        let _ = msg.channel_id.say(&ctx.http, "Quack! 🦆");
        return false;
    }

    utils::log_command(ctx, msg, cmd);

    true
}

fn after(context: &mut Context, message: &Message, _cmd: &str, error: CommandResult) {
    if let Err(why) = error {
        let error_msg = why.0;
        let _ = message.channel_id.say(&context, error_msg);
    }
}

fn is_trolling() -> bool {
    use rand::distributions::WeightedIndex;
    use rand::prelude::*;

    let choices = [true, false];
    let weights = [1, 9];
    let dist = WeightedIndex::new(&weights).unwrap();

    let mut rng = thread_rng();
    let result = dist.sample(&mut rng);

    choices[result]
}
