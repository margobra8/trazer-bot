use crate::dialogue::{states::receive_phone::ReceivePhoneState, Dialogue};
use teloxide::adaptors::DefaultParseMode;
use teloxide::prelude::*;
use teloxide::utils::markdown::code_inline;

use regex::Regex;

#[derive(Clone, Generic)]
pub struct ReceiveEmailState {
    pub full_name: String,
}

// Static ref, compile time of Phone Regex
lazy_static::lazy_static! {
    static ref EMAILRE: Regex = Regex::new(r"^.+@(alumnos\.)?upm\.es$").unwrap();
}

#[teloxide(subtransition)]
async fn receive_email_state(
    state: ReceiveEmailState,
    cx: TransitionIn<AutoSend<DefaultParseMode<Bot>>>,
    ans: String,
) -> TransitionOut<Dialogue> {
    match EMAILRE.is_match(ans.as_str()) {
        true => {
            cx.answer(format!("Genial\\. Dime, ¿cuál es tu número de teléfono móvil\\? Introdúcelo sin prefijos ni espacios: de la forma {}", code_inline("612345678"))).await?;
            next(ReceivePhoneState::up(state, ans))
        }
        false => {
            cx.answer(format!("Prueba de nuevo a enviarme en un solo mensaje un email acabado en {} o {}\\. Ejemplo: peter\\.parker@alumnos\\.upm\\.es", code_inline("@upm.es"), code_inline("@alumnos.upm.es"))).await?;
            next(state)
        }
    }
}
