#![allow(clippy::trivial_regex)]
#![allow(dead_code)]

#[macro_use]
extern crate frunk;
extern crate lazy_static;

mod contexts;
mod dialogue;

use crate::contexts::connections::ConnectionContext;
use crate::dialogue::Dialogue;

use std::{env};

use redis::{Client};

use teloxide::{
    adaptors::DefaultParseMode,
    dispatching::dialogue::{serializer::Bincode, RedisStorage, Storage},
    prelude::*,
    requests::RequesterExt,
    types::ParseMode::MarkdownV2,
    RequestError,
};

use thiserror::Error;

type StorageError = <RedisStorage<Bincode> as Storage<Dialogue>>::Error;
type In = DialogueWithCx<AutoSend<DefaultParseMode<Bot>>, Message, Dialogue, StorageError>;

#[derive(Debug, Error)]
enum Error {
    #[error("error from Telegram: {0}")]
    TelegramError(#[from] RequestError),
    #[error("error from storage: {0}")]
    StorageError(#[from] StorageError),
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!(target: "teloxide_backend", "Iniciando Trazer Bot...");

    let redis_hostname: String = env::var("REDIS_HOSTNAME").unwrap_or("localhost".to_string());
    let redis_port: u16 = env::var("REDIS_PORT")
        .unwrap_or("6379".to_string())
        .parse()
        .unwrap();

    let redis_connection = Client::open(format!("redis://{}:{}", &redis_hostname, &redis_port))
        .unwrap()
        .get_tokio_connection()
        .await
        .unwrap();

    log::info!(target: "redis_backend", "Conexión OK a Redis en {}:{}", &redis_hostname, &redis_port);

    let context_c = ConnectionContext::new(redis_connection);

    log::info!(target: "connection_context", "ConnectionContext creado con Redis y Selenium");

    let bot =
        Bot::new(env::var("BOT_TOKEN").expect(
            "Proporciona el token del bot de Telegram en la variable de entorno `BOT_TOKEN`",
        ))
        .parse_mode(MarkdownV2)
        .auto_send();

    Dispatcher::new(bot)
        .messages_handler(DialogueDispatcher::with_storage(
            move |DialogueWithCx { cx, dialogue }: In| {
                let conn = context_c.clone();
                async move {
                    let dialogue = dialogue.expect("std::convert::Infallible");

                    handle_message(cx, dialogue, conn)
                        .await
                        .expect("Algo malo ha pasado con el bot!")
                }
            },
            RedisStorage::open(
                format!("redis://{}:{}", &redis_hostname, &redis_port),
                Bincode,
            )
            .await
            .unwrap(),
        ))
        .dispatch()
        .await;
}

async fn handle_message(
    cx: UpdateWithCx<AutoSend<DefaultParseMode<Bot>>, Message>,
    dialogue: Dialogue,
    connu: ConnectionContext,
) -> TransitionOut<Dialogue> {
    match cx.update.text().map(ToOwned::to_owned) {
        None => {
            cx.answer("¡Buenas! Soy Trazer, el bot asistente para mantener la trazabilidad COVID del acceso al local del IEEE").await?;
            next(dialogue)
        }

        Some(ans) => dialogue.react(cx, ans).await,
    }
}
