use serenity::model::{
    channel::Message,
    id::{ChannelId, GuildId, MessageId},
};
use serenity::prelude::Context;

use crate::core::consts::{MAIN_COLOUR, MESSAGE_ID_RE};

fn message_url(guild_id: GuildId, channel_id: ChannelId, message_id: MessageId) -> String {
    format!(
        "[Jump!](https://discordapp.com/channels/{}/{}/{}/)",
        guild_id, channel_id, message_id
    )
}

pub fn message_id_monitor(context: &Context, message: &Message) {
    let guild_id = match message.guild_id {
        Some(id) => id,
        None => return,
    };

    for cap in MESSAGE_ID_RE.find_iter(message.content.as_str()) {
        let msg_id: u64 = match cap.as_str().parse::<u64>() {
            Ok(msg_id) => msg_id,
            Err(_) => return,
        };

        match message.channel_id.message(context, msg_id) {
            Ok(msg) => handle_message(context, guild_id, message.channel_id, &msg),
            Err(why) => {
                warn!("[MessageID Monitor] {}", why);
                let channels = match guild_id.channels(context) {
                    Ok(channels) => channels,
                    Err(_) => continue,
                };

                for (channel_id, _) in channels {
                    match channel_id.message(context, msg_id) {
                        Ok(msg) => {
                            handle_message(context, guild_id, message.channel_id, &msg);
                            break;
                        }
                        Err(why) => warn!("[MessageID Monitor] {}", why),
                    }
                }
            }
        }
    }
}

use serenity::builder::CreateEmbed;

fn handle_message(
    context: &Context,
    guild_id: GuildId,
    target_channel_id: ChannelId,
    message: &Message,
) {
    dbg!(&message.embeds);

    if !message.embeds.is_empty() {
        let _ = target_channel_id.send_message(context, |m| {
            if !message.content.is_empty() {
                m.content(message.content.clone());
            }

            let embed = message.embeds[0].clone();
            m.embed(|e| {
                e.clone_from(&CreateEmbed::from(embed));
                e
            });

            m
        });

        return;
    }

    if !message.content.is_empty() || !message.attachments.is_empty() {
        let url = message_url(guild_id, message.channel_id, message.id);

        let _ = target_channel_id.send_message(context, |m| {
            m.embed(|e| {
                e.author(|a| {
                    a.name(
                        message
                            .author
                            .nick_in(context, guild_id)
                            .unwrap_or_else(|| message.author.name.clone()),
                    )
                    .icon_url(
                        message
                            .author
                            .avatar_url()
                            .unwrap_or_else(|| message.author.default_avatar_url()),
                    )
                })
                .colour(MAIN_COLOUR)
                .description(message.content.clone())
                .field("Original", url, false);

                if !message.attachments.is_empty() {
                    let mut attachments = message.attachments.clone();
                    if let Some((index, image)) = attachments
                        .iter()
                        .enumerate()
                        .find(|(_, attachment)| attachment.dimensions().is_some())
                    {
                        dbg!(&image);
                        e.image(image.url.clone());
                        attachments.remove(index);
                    }

                    let files = attachments
                        .iter()
                        .map(|item| format!("[{}]({})", item.filename, item.url))
                        .collect::<Vec<String>>();

                    if !files.is_empty() {
                        e.field("Attachments", files.join("\n"), false);
                    }
                };

                e
            })
        });
    }
}
