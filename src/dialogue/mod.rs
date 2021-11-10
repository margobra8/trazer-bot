mod states;

use crate::dialogue::states::{
    ReceiveDniState, ReceiveEmailState, ReceiveFullNameState, ReceivePhoneState, StartState,
};
use derive_more::From;

use teloxide::macros::Transition;

use serde::{Deserialize, Serialize};

#[derive(Transition, Clone, From, Serialize, Deserialize)]
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
