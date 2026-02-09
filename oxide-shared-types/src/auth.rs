use std::fmt::{Formatter, Debug};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JwtToken {
    pub token: String,
}
impl Debug for JwtToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JwtToken(***)")
    }
}