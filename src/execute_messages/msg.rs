use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::execute_messages::msg_admin::AdminExecuteMsg;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Subscribe { id_subscription: u32 },

    Admin(AdminExecuteMsg),
}
