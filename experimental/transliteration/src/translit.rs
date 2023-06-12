#[derive(Debug, Clone)]
pub enum PatternElement {
    Literal(String),
    UnicodeSet(String),
}

// invariant: consecutive literals must be concatenated
#[derive(Debug, Clone)]
pub struct Pattern(pub Vec<PatternElement>);

#[derive(Debug, Clone)]
pub struct Replacer {
    pub replacement: String,
    // offset from the end of the match, i.e., cursor = 0 is the default behavior
    pub cursor: i32,
}

// these types get serialized
#[derive(Debug, Clone)]
pub struct Rule {
    pub ante: Option<Pattern>,
    pub key: Pattern,
    pub post: Option<Pattern>,
    pub target: Replacer,
}

#[derive(Debug, Clone)]
pub struct Transliterator {
    pub rules: Vec<Rule>,
}

impl Transliterator {
    pub fn transliterate(&self, source: &str) -> String {
        /* Basic algorithm:
           Until the cursor is at the end of the string and there are no more rules to apply:
            * Find the first rule that matches the current cursor position
                * Find the rule by reverse-matching the ante context (i.e., s[0..cursor].rev())
                * Then the key and post-context from s[cursor..]
            * Apply the rule
                * Replace the matched part of the source string with the target
                * Move the cursor according to the offset
         */

        let mut chars: Vec<_> = source.chars().collect();
        let mut cursor = 0;

        while cursor < chars.len() {
            let mut rule_index = None;
            let mut rule_match_len = 0;
            let mut rule_match_cursor = 0;
            let mut rule_match_replacer = None;
            for (i, rule) in self.rules.iter().enumerate() {
                let ante = match &rule.ante {
                    Some(ante) => ante,
                    None => {
                        rule_index = Some(i);
                        rule_match_len = 0;
                        rule_match_cursor = 0;
                        rule_match_replacer = Some(&rule.target);
                        break;
                    }
                };
                let mut ante_cursor = cursor;
                let mut ante_match_len = 0;
                for element in ante.0.iter().rev() {
                    match element {
                        PatternElement::Literal(literal) => {
                            let literal_chars: Vec<_> = literal.chars().collect();
                            if ante_cursor < literal_chars.len() {
                                break;
                            }
                            let mut literal_cursor = 0;
                            while literal_cursor < literal_chars.len() {
                                if chars[ante_cursor] != literal_chars[literal_cursor] {
                                    break;
                                }
                                ante_cursor += 1;
                                literal_cursor += 1;
                            }
                            if literal_cursor == literal_chars.len() {
                                ante_match_len += literal_chars.len();
                            } else {
                                break;
                            }
                        },
                        PatternElement::UnicodeSet(set) => {
                            let mut set_cursor = 0;
                            while set_cursor < set.len() {
                                if chars[ante_cursor] != set.chars().nth(set_cursor).unwrap() {
                                    break;
                                }
                                ante_cursor += 1;
                                set_cursor += 1;
                            }
                            if set_cursor == set.len() {
                                ante_match_len += set.len();
                            } else {
                                break;
                            }
                        },
                    }
                }
                if ante_match_len > 0 {
                    let key = &rule.key;
                    let mut key_cursor = cursor + ante_match_len;
                    let mut key_match_len = 0;
                    for element in key.0.iter() {
                        match element {
                            PatternElement::Literal(literal) => {
                                let literal_chars: Vec<_> = literal.chars().collect();
                                if key_cursor < literal_chars.len() {
                                    break;
                                }
                                let mut literal_cursor = 0;
                                while literal_cursor < literal_chars.len() {
                                    if chars[key_cursor] != literal_chars[literal_cursor] {
                                        break;
                                    }
                                    key_cursor += 1;
                                    literal_cursor += 1;
                                }
                                if literal_cursor == literal_chars.len() {
                                    key_match_len += literal_chars.len();
                                } else {
                                    break;
                                }
                            },
                            PatternElement::UnicodeSet(set) => {
                                let mut set_cursor = 0;
                                while set_cursor < set.len() {
                                    if chars[key_cursor] != set.chars().nth(set_cursor).unwrap() {
                                        break;
                                    }
                                    key_cursor += 1;
                                    set_cursor += 1;
                                }
                                if set_cursor == set.len() {
                                    key_match_len += set.len();
                                } else {
                                    break;
                                }
                            },
                        }
                    }
                    if key_match_len > 0 {
                        let post = match &rule.post {
                            Some(post) => post,
                            None => {
                                rule_index = Some(i);
                                rule_match_len = ante_match_len + key_match_len;
                                rule_match_cursor = cursor + ante_match_len;
                                rule_match_replacer = Some(&rule.target);
                                break;
                            }
                        };
                        let mut post_cursor = key_cursor;
                        let mut post_match_len = 0;
                        for element in post.0.iter() {
                            match element {
                                PatternElement::Literal(literal) => {
                                    let literal_chars: Vec<_> = literal.chars().collect();
                                    if post_cursor < literal_chars.len() {
                                        break;
                                    }
                                    let mut literal_cursor = 0;
                                    while literal_cursor < literal_chars.len() {
                                        if chars[post_cursor] != literal_chars[literal_cursor] {
                                            break;
                                        }
                                        post_cursor += 1;
                                        literal_cursor += 1;
                                    }
                                    if literal_cursor == literal_chars.len() {
                                        post_match_len += literal_chars.len();
                                    } else {
                                        break;
                                    }
                                },
                                PatternElement::UnicodeSet(set) => {
                                    let mut set_cursor = 0;
                                    while set_cursor < set.len() {
                                        if chars[post_cursor] != set.chars().nth(set_cursor).unwrap() {
                                            break;
                                        }
                                        post_cursor += 1;
                                        set_cursor += 1;
                                    }
                                    if set_cursor == set.len() {
                                        post_match_len += set.len();
                                    } else {
                                        break;
                                    }
                                },
                            }
                        }
                        if post_match_len > 0 {
                            rule_index = Some(i);
                            rule_match_len = ante_match_len + key_match_len + post_match_len;
                            rule_match_cursor = cursor + ante_match_len;
                            rule_match_replacer = Some(&rule.target);
                            break;
                        }
                    }
                }
            }
            if let Some(rule_index) = rule_index {
                if let Some(rule_match_replacer) = rule_match_replacer {
                    let rule_match_cursor = rule_match_cursor;
                    let rule_match_len = rule_match_len;
                    let rule_match_replacer = rule_match_replacer;
                    let mut rule_match_replacer_chars: Vec<_> = rule_match_replacer.chars().collect();
                    let mut rule_match_replacer_cursor = 0;
                    while rule_match_replacer_cursor < rule_match_replacer_chars.len() {
                        chars.remove(rule_match_cursor);
                        chars.insert(rule_match_cursor, rule_match_replacer_chars[rule_match_replacer_cursor]);
                        rule_match_replacer_cursor += 1;
                        rule_match_cursor += 1;
                    }
                    cursor += rule_match_len;
                } else {
                    cursor += 1;
                }
            } else {
                cursor += 1;
            }
        }
        let result: String = chars.into_iter().collect();

        result
    }
}