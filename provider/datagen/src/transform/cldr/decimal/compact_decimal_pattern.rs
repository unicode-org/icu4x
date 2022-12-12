use std::{borrow::Cow, str::FromStr};

#[derive(PartialEq, Clone)]
pub struct ParsedPlaceholder {
    pub index: usize,
    pub number_of_0s: i8,
}

#[derive(PartialEq, Clone)]
pub struct ParsedPattern {
    pub literal_text: Cow<'static, str>,
    pub placeholder: Option<ParsedPlaceholder>,
}

pub fn parse(pattern: &str) -> Result<Option<ParsedPattern>, Cow<'static, str>> {
    if pattern == "0" {
        return Ok(None);
    } else {
        let mut placeholder: Option<ParsedPlaceholder> = None;
        let mut literal_text = String::with_capacity(pattern.len());
        for (i, chunk) in pattern.split('\'').enumerate() {
            let escaped = i % 2 == 1;
            if escaped {
                if chunk.is_empty() {
                    // '' means '.
                    literal_text.push('\'');
                } else {
                    // Anything else wrapped in apostrophes is literal text.
                    literal_text.push_str(chunk);
                }
            } else {
                // We are in unquoted text, so we need to check for the
                // symbols defined in https://www.unicode.org/reports/tr35/tr35-numbers.html#Number_Pattern_Character_Definitions.
                if chunk
                    .chars()
                    .any(|c| ('1'..'9').contains(&c) || "@#.-,E+%‰,¤*'".contains(c))
                {
                    return Err(format!(
                        "Unsupported symbol in compact decimal pattern {}",
                        pattern
                    )
                    .into());
                }
                if let Some((prefix, additional_0s_and_suffix)) = chunk.split_once('0') {
                    if placeholder.is_some() {
                        return Err(format!(
                            "Multiple placeholders in compact decimal pattern {})",
                            pattern
                        )
                        .into());
                    }
                    literal_text.push_str(prefix);
                    if let Some((middle_0s, suffix)) = additional_0s_and_suffix.rsplit_once('0') {
                        if !middle_0s.chars().all(|c| c == '0') {
                            return Err(format!(
                                "Multiple placeholders in compact decimal pattern {}",
                                pattern
                            )
                            .into());
                        }
                        placeholder = Some(ParsedPlaceholder {
                            index: literal_text.len(),
                            number_of_0s: i8::try_from(middle_0s.len() + 2)
                                .map_err(|_| format!("Too many 0s in pattern {}", pattern))?,
                        });
                        literal_text.push_str(suffix);
                    } else {
                        placeholder = Some(ParsedPlaceholder {
                            index: literal_text.len(),
                            number_of_0s: 1,
                        });
                        literal_text.push_str(additional_0s_and_suffix);
                    }
                } else {
                    // No symbols, all literal text.
                    literal_text.push_str(chunk);
                }
            }
        }
        Ok(Some(ParsedPattern {
            literal_text: Cow::Owned(literal_text),
            placeholder: placeholder,
        }))
    }
}
