use std::fmt::{Display, Formatter};
use std::thread::sleep;

/*
Differences to parsed types:
 * Rules go only in one direction
 * No contexts on the target
 * Target is a literal
 * All variables get replaced with their definition
 * Adjacent literals get concatenated
 * Cursor is only allowed to exist on the target, and is implemented using a signed offset
*/

/*
TODO: * zero-copify types
      * switch to proper UnicodeSet
*/

#[derive(Debug, Clone)]
pub enum PatternElement {
    Literal(String),
    UnicodeSet(String),
}

impl PatternElement {
    fn matches(&self, source: &[char], dir: MatchDirection) -> Option<u32> {
        match self {
            PatternElement::Literal(s) => {
                dbg!(dir);
                dbg!(s);
                dbg!(source.iter().collect::<String>());
                let s_chars = s.chars().collect::<Vec<_>>();

                match dir {
                    MatchDirection::Forward => {
                        if source.starts_with(&s_chars[..]) {
                            Some(s_chars.len() as u32)
                        } else {
                            None
                        }
                    }
                    MatchDirection::Backward => {
                        if source.ends_with(&s_chars[..]) {
                            Some(s_chars.len() as u32)
                        } else {
                            None
                        }
                    }
                }
            }
            PatternElement::UnicodeSet(_s) => {
                // TODO: implement
                Some(1)
            }
        }
    }
}

// invariant: consecutive literals must be concatenated
#[derive(Debug, Clone)]
pub struct Pattern(pub Vec<PatternElement>);

impl Pattern {
    // returns the length of the match if there was one
    fn matches(&self, source: &[char], dir: MatchDirection) -> Option<u32> {
        match dir {
            MatchDirection::Forward => self.matches_forward(source),
            MatchDirection::Backward => self.matches_backward(source),
        }
    }

    fn matches_forward(&self, source: &[char]) -> Option<u32> {
        let mut i = 0;

        for element in &self.0 {
            if i >= source.len() {
                return None;
            }
            if let Some(len) = element.matches(&source[i..], MatchDirection::Forward) {
                i += len as usize;
            } else {
                return None;
            }
        }

        Some(i as u32)
    }

    fn matches_backward(&self, source: &[char]) -> Option<u32> {
        let mut i = source.len();

        for element in self.0.iter().rev() {
            if i == 0 {
                return None;
            }
            if let Some(len) = element.matches(&source[..i], MatchDirection::Backward) {
                i -= len as usize;
            } else {
                return None;
            }
        }

        Some((source.len() - i) as u32)
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for element in &self.0 {
            match element {
                PatternElement::Literal(l) => {
                    // todo: refactor this function into one pass
                    // check if any char needs escaping
                    let res = if l.chars().any(|c| c == '\'') {
                        // cannot escape in quoted literals, escape using \
                        let mut res = String::new();
                        for c in l.chars() {
                            if !super::dfaparsing::legal_top_level_char(c) {
                                res.push('\\');
                            }
                            res.push(c);
                        }
                        res
                    } else if !l.chars().all(super::dfaparsing::legal_top_level_char) {
                        // needs escaping, can be escaped with quotes
                        format!("'{}'", l)
                    } else {
                        // no escaping needed
                        l.clone()
                    };
                    write!(f, "{}", res)?
                },
                PatternElement::UnicodeSet(s) => write!(f, "[{}]", s)?,
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Replacer {
    pub replacement: String,
    // offset from the end of the match, i.e., cursor = 0 is the default behavior,
    // -replacement.len() would be restarting from the beginning
    pub cursor: i32,
}

impl Display for Replacer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // TODO: need to figure out a solution for cases where the replacement contains chars that need escaping
        if self.cursor == 0 {
            return write!(f, "{}", self.replacement);
        }
        let mut replacement_chars: Vec<_> = self.replacement.chars().collect();
        let replacement_len = replacement_chars.len();
        let cursor_beginning = self.cursor + replacement_chars.len() as i32;
        // pad the replacement_chars with @ if cursor is negative or greater than len
        if cursor_beginning < 0 {
            replacement_chars = vec!['@'; -cursor_beginning as usize]
                .into_iter()
                .chain(replacement_chars.into_iter())
                .collect();
            write!(f, "|{}", replacement_chars.into_iter().collect::<String>())
        } else if cursor_beginning > replacement_len as i32 {
            replacement_chars = replacement_chars
                .into_iter()
                .chain(vec!['@'; cursor_beginning as usize - replacement_len].into_iter())
                .collect();
            write!(f, "{}|", replacement_chars.into_iter().collect::<String>())
        } else {
            replacement_chars.insert(cursor_beginning as usize, '|');
            write!(f, "{}", replacement_chars.into_iter().collect::<String>())
        }
    }
}

// these types get serialized
#[derive(Debug, Clone)]
pub struct Rule {
    pub ante: Option<Pattern>,
    pub key: Pattern,
    pub post: Option<Pattern>,
    pub target: Replacer,
}

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(ante) = &self.ante {
            write!(f, "{} {{ ", ante)?;
        }
        write!(f, "{}", self.key)?;
        if let Some(post) = &self.post {
            write!(f, " }} {}", post)?;
        }
        write!(f, " → {}", self.target)
    }
}

#[derive(Debug, Clone, Copy)]
enum MatchDirection {
    Forward,
    Backward,
}

#[derive(Debug, Clone)]
pub struct Transliterator {
    pub rules: Vec<Rule>,
}

impl Display for Transliterator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[\n")?;
        for rule in &self.rules {
            write!(f, "  {}\n", rule)?;
        }
        write!(f, "]")
    }
}

impl Transliterator {
    pub fn transliterate(&self, source: &str) -> String {
        /* Basic algorithm:
           Until the cursor is at the end of the string and there are no more rules to apply:
            1. Find the first rule that matches the current cursor position
                a) Find the rule by reverse-matching the ante context (i.e., s[0..cursor].rev())
                b) Then the key and post-context from s[cursor..]
            2. Apply the rule
                a) Replace the matched part of the source string with the target
                b) Move the cursor according to the offset
         */

        let mut chars: Vec<_> = source.chars().collect();
        let mut cursor = 0;

        'outer: while cursor < chars.len() {
            dbg!(cursor);
            dbg!(chars.iter().copied().collect::<String>());
            // sleep(std::time::Duration::from_millis(1000));


            // step 1.
            for (i, rule) in self.rules.iter().enumerate() {
                // dbg!("checking rule {:?}", rule);
                // TODO: factor out for loop body into Rule::matches

                let ante = rule.ante.as_ref();
                let post = rule.post.as_ref();
                let key = &rule.key;

                // step 1a)
                if let Some(ante) = ante {
                    if ante.matches(&chars[0..cursor], MatchDirection::Backward).is_none() {
                        // ante doesn't match, skip this rule
                        continue;
                    }
                }

                let mut post_start = cursor;

                // step 1b)
                if let Some(key_match_len) = key.matches(&chars[cursor..], MatchDirection::Forward) {
                    let key_match_len = key_match_len as usize;
                    post_start += key_match_len;
                } else {
                    // key doesn't match, skip this rule
                    continue;
                }
                if let Some(post) = post {
                    if post.matches(&chars[post_start..], MatchDirection::Forward).is_none() {
                        // post doesn't match, skip this rule
                        continue;
                    }
                }
                // matched rule
                dbg!(rule);
                // step 2a)
                let mut replacement: Vec<_> = rule.target.replacement.chars().collect();
                let mut new_chars = replacement.len();
                chars.splice(cursor..post_start as usize, replacement.drain(..));
                // step 2b)
                let new_cursor = cursor as i64 + new_chars as i64 + rule.target.cursor as i64;
                cursor = new_cursor.clamp(0, chars.len() as i64) as usize;

                continue 'outer;
            }

            // no match: continue
            cursor += 1;
        }
        let result: String = chars.into_iter().collect();

        result
    }
}