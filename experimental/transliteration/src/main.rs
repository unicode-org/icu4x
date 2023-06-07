use combine::parser::char::{char, letter, spaces};
use combine::{between, choice, many1, parser, sep_by, Parser, EasyParser};
use combine::error::{ParseError, StdParseResult};
use combine::stream::{Stream, Positioned};
use combine::stream::position;

const RULES: &str = r#" 
$AE = [Ä {A \u0308}];
$OE = [Ö {O \u0308}];
$UE = [Ü {U \u0308}];

[ä {a \u0308}] → ae;
[ö {o \u0308}] → oe;
[ü {u \u0308}] → ue;

{$AE} [:Lowercase:] → Ae;
{$OE} [:Lowercase:] → Oe;
{$UE} [:Lowercase:] → Ue;

$AE → AE;
$OE → OE;
$UE → UE;
"#;

struct Match {
    start: u32,
    end: u32,
}

struct Matcher {}

// impl Matcher {
//     pub fn matches(&self, &StringSegment) -> Option<Match> {}
// }

struct Replacer {}

struct TransliterationRule {
    ante_context: Option<Matcher>,
    post_context: Option<Matcher>,
    key: Matcher,
    output: Replacer,    
} 

fn main() {
    println!("{RULES}\nHello, world!");
    println!("{:?}", parse_rules().parse(RULES));
}

struct UnicodeSet(String);
struct Literal(String);

enum UnicodeSetOrLiteral {
    UnicodeSet(UnicodeSet),
    Literal(Literal),
}

struct KeyLike(Vec<Literal>);

struct ContextLike(Vec<UnicodeSetOrLiteral>);

struct HalfRule {
    ante: Option<ContextLike>,
    key: KeyLike,
    post: Option<ContextLike>,
}

struct Rule {
    source: HalfRule,
    target: HalfRule,
}

/*
Parsing:
first just handle rules:
 * root: half direction half ;
 * half: (context {)? key (} context)?
 * context: (UnicodeSet|literal)*
 * UnicodeSet: \[:ident:\] | ...
 * key: (UnicodeSet|literal)*
 * literal: quoted_string | unquoted_string_wo_illegal_chars | unicode_escape | hex_escape

 */

fn parse_rules_<Input>() -> impl Parser<Input, Output = Vec<()>> 
where Input: Stream<Token = char>,
// Necessary due to rust-lang/rust#24159
Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many1(combine::parser::token::any().map(|_| ()))
}

parser!{
    fn parse_rules[Input]()(Input) -> Vec<()>
    where [Input: Stream<Token = char>]
    {
        parse_rules_()
    }
}
