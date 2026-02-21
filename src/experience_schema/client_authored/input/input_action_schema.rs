use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct InputActionSchema {
    pub input_action: String,
    pub input_event: String,
}
