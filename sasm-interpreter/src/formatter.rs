use crate::{error::RuntimeError, varstorage::VariableStorage};
use regex::Regex;
use sasm_parse::{expression::Expression, ident::Identifier};
use std::sync::OnceLock;

static PATTERN: OnceLock<Regex> = OnceLock::new();

pub fn format(fmt: &str, vars: &VariableStorage) -> Result<String, RuntimeError> {
    let mut buffer = fmt.to_string();
    // TODO: Allow escaping
    let re = PATTERN.get_or_init(|| Regex::new(r"\{[a-zA-Z]\w*\}").unwrap());

    while let Some(cap) = re.find(&buffer) {
        let var_name = &cap.as_str()[1..(cap.len() - 1)];
        let ident = Identifier::try_from(var_name).unwrap();

        let var_value = vars.get_nonnull(&ident)?;
        let repr = match var_value {
            Expression::Number(n) => n.to_string(),
            Expression::String(string) => string.clone(),
            Expression::Identifier(..) => unreachable!(),
        };

        buffer.replace_range(cap.range(), &repr);
    }

    Ok(buffer)
}
