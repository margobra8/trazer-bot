use crate::dialogue::{states::receive_email::ReceiveEmailState, Dialogue};
use teloxide::adaptors::DefaultParseMode;
use teloxide::prelude::*;
use teloxide::utils::markdown::code_inline;

#[derive(Clone, Generic)]
pub struct ReceiveFullNameState;

#[teloxide(subtransition)]
async fn receive_full_name(
    state: ReceiveFullNameState,
    cx: TransitionIn<AutoSend<DefaultParseMode<Bot>>>,
    ans: String,
) -> TransitionOut<Dialogue> {
    cx.answer(format!(
        "Gracias, ¿Cuál es tu correo {} o {}\\?",
        code_inline("@upm.es"),
        code_inline("@alumnos.upm.es")
    ))
    .await?;
    next(ReceiveEmailState::up(state, ans))
}
