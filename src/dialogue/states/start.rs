use crate::dialogue::{states::ReceiveFullNameState, Dialogue};

use serde::{Deserialize, Serialize};

use teloxide::adaptors::DefaultParseMode;
use teloxide::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct StartState;

#[teloxide(subtransition)]
async fn start(
    _state: StartState,
    cx: TransitionIn<AutoSend<DefaultParseMode<Bot>>>,
    _ans: String,
) -> TransitionOut<Dialogue> {
    cx.answer("¡Buenas\\! Veo que no te has registrado antes en el sistema de trazabilidad\\.\\.\\.\nComencemos, ¿cuál es tu nombre y apellidos\\?").await?;
    next(ReceiveFullNameState)
}
