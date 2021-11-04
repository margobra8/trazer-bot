mod receive_dni;
mod receive_email;
mod receive_full_name;
mod receive_phone;
mod start;

pub use receive_dni::ReceiveDniState;
pub use receive_email::ReceiveEmailState;
pub use receive_full_name::ReceiveFullNameState;
pub use receive_phone::ReceivePhoneState;
pub use start::StartState;
