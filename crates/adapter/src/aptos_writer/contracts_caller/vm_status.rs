use std::fmt::{Display, Formatter};

use irelia_core::common::aptos_writer_error::AptosWriterError;
use regex::Regex;

#[derive(Debug)]
pub struct VmStatus {
    pub location: String,
    pub reason: String,
    pub code: u64,
    pub description: String,
}

impl Display for VmStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, " = {:#?}", self)
    }
}

impl TryInto<VmStatus> for &str {
    type Error = AptosWriterError;

    fn try_into(self) -> Result<VmStatus, Self::Error> {
        let re = Regex::new(
            r"Move abort in (?P<location>[^:]+::[^:]+): (?:(?P<reason>[^:]*)\()?(?P<code>0x[0-9a-fA-F]+)(?:: (?P<description>.*))?"
        ).expect("never fail");
        let caps = re
            .captures(self)
            .ok_or(AptosWriterError::ParseVmStatusError(self.to_string()))?;
        let location = caps.name("location").map_or("", |m| m.as_str()).to_string();
        let reason = caps.name("reason").map_or("", |m| m.as_str()).to_string();
        let code = caps.name("code").map_or("", |m| m.as_str());
        let description = caps
            .name("description")
            .map_or("", |m| m.as_str())
            .to_string();
        let code = u64::from_str_radix(code.trim_start_matches("0x"), 16)?;
        Ok(VmStatus {
            location,
            reason,
            code,
            description,
        })
    }
}
