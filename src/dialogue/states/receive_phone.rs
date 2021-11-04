use crate::dialogue::{states::ReceiveDniState, Dialogue};
use teloxide::adaptors::DefaultParseMode;
use teloxide::prelude::*;
use teloxide::utils::markdown::code_inline;

use regex::Regex;

#[derive(Clone, Generic)]
pub struct ReceivePhoneState {
    pub full_name: String,
    pub email: String,
}

lazy_static::lazy_static! {
    static ref PHONERE: Regex = Regex::new(r"^[67]\d{8}$").unwrap();
}

#[teloxide(subtransition)]
async fn receive_phone_state(
    state: ReceivePhoneState,
    cx: TransitionIn<AutoSend<DefaultParseMode<Bot>>>,
    ans: String,
) -> TransitionOut<Dialogue> {
    match PHONERE.is_match(ans.as_str()) {
        true => {
            cx.answer(format!(
                "¿Cuál es tu DNI\\? Escríbelo con 8 dígitos y letra mayúscula de la forma {}",
                code_inline("01234567Z")
            ))
            .await?;
            next(ReceiveDniState::up(state, ans))
        }
        false => {
            cx.answer(
                "Prueba de nuevo a enviarme el número de teléfono con un formato correcto\\.",
            )
            .await?;
            next(state)
        }
    }
}
