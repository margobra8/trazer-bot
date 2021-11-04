mod states;

use crate::dialogue::states::{
    ReceiveDniState, ReceiveEmailState, ReceiveFullNameState, ReceivePhoneState, StartState,
};
use derive_more::From;
use teloxide::macros::Transition;

#[derive(Transition, Clone, From)]
pub enum Dialogue {
    Start(StartState),
    ReceiveFullName(ReceiveFullNameState),
    ReceiveEmail(ReceiveEmailState),
    ReceivePhone(ReceivePhoneState),
    ReceiveDni(ReceiveDniState),
}

impl Default for Dialogue {
    fn default() -> Self {
        Self::Start(StartState)
    }
}
