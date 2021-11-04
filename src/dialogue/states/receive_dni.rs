use crate::dialogue::Dialogue;

use teloxide::adaptors::DefaultParseMode;
use teloxide::utils::markdown::escape;

use teloxide::prelude::*;
use teloxide::utils::markdown::{bold, code_inline};

use regex::Regex;

#[derive(Clone, Generic)]
pub struct ReceiveDniState {
    pub full_name: String,
    pub email: String,
    pub phone: String,
}

// Static ref, compile time of Phone Regex
lazy_static::lazy_static! {
    static ref DNIRE: Regex = Regex::new(r"^\d{8}[A-Z]$").unwrap();
}


#[teloxide(subtransition)]
async fn receive_location(
    state: ReceiveDniState,
    cx: TransitionIn<AutoSend<DefaultParseMode<Bot>>>,
    ans: String,
) -> TransitionOut<Dialogue> {
    match DNIRE.is_match(ans.as_str()) {
        true => {
            cx.answer(format!(
                "{}\nNombre y apellidos: {}\nEmail: {}\nTeléfono: {}\nDNI: {}",
                bold("Se han registrado los siguientes datos"),
                state.full_name,
                escape(state.email.as_ref()),
                state.phone,
                ans
            ))
            .await?;

            let current_user = cx.update.from().expect("Error obteniendo el usuario actual del chat");
            let user_id = current_user.id;
            let username = match &current_user.username {
                Some(u) => u,
                _ => "-",
            };

            cx.answer(escape(format!("Tu ID único de usuario es {} asociado al usuario @{}", user_id, username).as_ref())).await?;
            exit()
        }

        false => {
            cx.answer(format!(
                "Prueba de nuevo a proporcionarme un DNI válido con el formato {}",
                code_inline("01234567Z")
            ))
            .await?;
            next(state)
        }
    }
}
