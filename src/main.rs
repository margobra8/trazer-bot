#![allow(clippy::trivial_regex)]
#![allow(dead_code)]

#[macro_use]
extern crate frunk;
extern crate lazy_static;

mod dialogue;

use crate::dialogue::Dialogue;
use teloxide::adaptors::DefaultParseMode;
use teloxide::prelude::*;
use teloxide::requests::RequesterExt;
use teloxide::types::ParseMode::MarkdownV2;

use std::env;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting dialogue_bot...");

    let bot: AutoSend<DefaultParseMode<Bot>> =
        Bot::new(env::var("BOT_TOKEN").expect(
            "Proporciona el token del bot de Telegram en la variable de entorno `BOT_TOKEN`",
        ))
        .parse_mode(MarkdownV2)
        .auto_send();

    teloxide::dialogues_repl(bot, |message, dialogue| async move {
        handle_message(message, dialogue)
            .await
            .expect("Something wrong with the bot!")
    })
    .await;
}

async fn handle_message(
    cx: UpdateWithCx<AutoSend<DefaultParseMode<Bot>>, Message>,
    dialogue: Dialogue,
) -> TransitionOut<Dialogue> {
    match cx.update.text().map(ToOwned::to_owned) {
        None => {
            cx.answer("¡Buenas! Soy Covisito, el bot asistente para mantener la trazabilidad COVID del acceso al local del IEEE").await?;
            next(dialogue)
        }

        Some(ans) => {
            let ci: UpdateWithCx<AutoSend<DefaultParseMode<Bot>>, Message> = cx;
            dialogue.react(ci, ans).await
        }
    }
}
