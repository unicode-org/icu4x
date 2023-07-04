extern crate core;

use combine::error::{ParseError, StdParseResult};
use combine::parser::char::{char, letter, spaces, string};
use combine::parser::combinator::recognize;
use combine::parser::repeat::Many;
use combine::parser::token::Satisfy;
use combine::stream::position;
use combine::stream::{Positioned, Stream};
use combine::{
    attempt, between, choice, many, many1, optional, parser, satisfy, sep_by, value, EasyParser,
    Parser,
};
use std::fmt::Display;
use std::io::Write;

mod parse;
mod compile;
mod translit;

const RULES: &str = r#"
$AE = [Ä {A \u0308}];
$OE = [Ö {O \u0308}];
$UE = [Ü {U \u0308}];

[ä {a \u0308}] → ae;
[ö {o \u0308}] → oe;
[ü {u \u0308}] → ue;

{$AE} [:Lowercase:] → Ae;
{$AE}[:Lowercase:]→Ae;
{[]}[:Lowercase:]→Ae;
{a}[:Lowercase:]→Ae;
a {$AE} [:Lowercase:] → Ae;
a{$AE}[:Lowercase:]→Ae;
 a{[]}[:Lowercase:]→Ae;
 a {a}[:Lowercase:]→Ae;
 $AE} [:Lowercase:] → Ae;
$AE}[:Lowercase:]→Ae;
[]}[:Lowercase:]→Ae;
a}[:Lowercase:]→Ae;
{$OE} [:Lowercase:] → Oe;
{$UE} [:Lowercase:] → Ue;

$AE → AE;
$OE → OE;
$UE → UE;

'hello ' { world } > | @@@@@@ welt ;
{ 'hello welt' } > 'hallo welt' ;

'' > '"' ;

\>a\<''\' > ;

"#;

const RULES_ORIGINAL: &str = r#"

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

# ::Any-ASCII;


"#;

const RULES_NO_UNICODE_SET: &str = r#"

ä → ae;
ö → oe;
ü → ue;

{Ä} → Ae;
{Ö} → Oe;
{Ü} b → Ue;

Ä → AE;
Ö → OE;
Ü → UE;

'hello ' { world } > | @@@@@@ welt ;
{ 'hello welt'   > 'hallo welt' ;

# ::Any-ASCII;


"#;

const RULES_PUBLISHING: &str = r#"
# Variables
$single = \' ;
$space = ' ' ;
$double = \";
$back = \` ;
$tab = \u0008 ;
$makeRight = [[:Z:][:Ps:][:Pi:]$] ;

# fix UNIX quotes
$back $back → “ ;
$back → ‘ ;

# fix typewriter quotes, by context
$makeRight {$double} ↔ “ ;
$double ↔ ” ;
$makeRight {$single} ↔ ‘ ;
$single ↔ ’;

$space {$space} → ;
'<--->' ↔ '⟷';
'<---' ↔ '⟵';
'--->' ↔ '⟶';
'<-->' ↔ '↔';
'<--' ↔ '←';
'-->' ↔ '→';
'<-/->' ↔ '↮';
'<-/-' ↔ '↚';
'-/->' ↔ '↛';
'<===>' ↔ '⟺';
'<===' ↔ '⟸';
'===>' ↔ '⟹';
'<==>' ↔ '⇔';
'<==' ↔ '⇐';
'==>' ↔ '⇒';
'!=' ↔ ≠;
'<=' ↔ ≤;
'>=' ↔ ≥;
'+-' ↔ ±;
'-+' ↔ ∓;
'~=' ↔ ≅;
'--' ↔ —;
'...' ↔ …;

\(C\) ↔ ©;
\(c\) → ©;
\(R\) ↔ ®;
\(r\) → ®;
\(TM\) ↔ ™;
\(tm\) → ™;
{c\/o} ↔ ℅;

[^0-9] {1\/2} [^0-9] ↔ ½;
[^0-9] {1\/3} [^0-9] ↔ ⅓;
[^0-9] {2\/3} [^0-9] ↔ ⅔;
[^0-9] {1\/4} [^0-9] ↔ ¼;
[^0-9] {3\/4} [^0-9] ↔ ¾;
[^0-9] {1\/5} [^0-9] ↔ ⅕;
[^0-9] {2\/5} [^0-9] ↔ ⅖;
[^0-9] {3\/5} [^0-9] ↔ ⅗;
[^0-9] {4\/5} [^0-9] ↔ ⅘;
[^0-9] {1\/6} [^0-9] ↔ ⅙;
[^0-9] {5\/6} [^0-9] ↔ ⅚;
[^0-9] {1\/8} [^0-9] ↔ ⅛;
[^0-9] {3\/8} [^0-9] ↔ ⅜;
[^0-9] {5\/8} [^0-9] ↔ ⅝;
[^0-9] {7\/8} [^0-9] ↔ ⅞;
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

fn pretty_print_rules(rules: &[Rule]) {
    fn string_from_many_literals(v: &[UnicodeSetOrLiteral]) -> String {
        v.into_iter()
            .map(|s| format!("{s}"))
            .collect::<Vec<_>>()
            .join("")
    }

    println!("[");
    for rule in rules {
        let source_ante = rule
            .source
            .ante
            .as_ref()
            .map(|s| string_from_many_literals(&s.0))
            .unwrap_or("".into());
        let source_post = rule
            .source
            .post
            .as_ref()
            .map(|s| string_from_many_literals(&s.0))
            .unwrap_or("".into());
        let target_ante = rule
            .target
            .ante
            .as_ref()
            .map(|s| string_from_many_literals(&s.0))
            .unwrap_or("".into());
        let target_post = rule
            .target
            .post
            .as_ref()
            .map(|s| string_from_many_literals(&s.0))
            .unwrap_or("".into());

        let source_key = rule
            .source
            .key
            .0
            .iter()
            .map(|Literal(s)| format!("{s}"))
            .collect::<Vec<_>>()
            .join("");
        let target_key = rule
            .target
            .key
            .0
            .iter()
            .map(|Literal(s)| format!("{s}"))
            .collect::<Vec<_>>()
            .join("");
        println!("  {source_ante} {{ {source_key} }} {source_post} → {target_ante} {{ {target_key} }} {target_post}", );
    }
    println!("]");
}

// TODO: add below to gdoc
// TODO: escaped() for \u, \{ etc in literals
// TODO: string literals with '
// TODO: check if there's some unicode set parsing already
// TODO: add "compile" function that: 1. compiles UnicodeSets (converts them into CodePointInversionLists) and 2. compiles variable references.

fn main() {
    let source = "

    J'ai écouté.

    Überlegen ist ÜBERTRIEBEN gut für die Gesundheit. # demonstrates post context

    string replacement: \u{61}\u{0308} Au0308

    hello world :) # demonstrates ante context and cursor

    ";

    // let source = "hello world";


    // let rules = RULES_NO_UNICODE_SET;
    let rules = RULES_ORIGINAL;
    //let rules = RULES_PUBLISHING;
    let mut it = rules.chars().peekable();
    let rules = parse::parse_rules(&mut it);
    eprintln!("Remaining input: {:?}", it.collect::<String>());
    eprintln!("debug: {:?}", rules);
    let rules = match rules {
        Ok(rules) => {
            eprintln!("Parsed rules:");
            parse::pretty_print_rules(&rules);
            rules
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            return;
        }
    };

    let mut compiler = compile::Compiler::new();
    let (fwd_translit, bwd_translit) = compiler.compile(&rules).unwrap();

    eprintln!("Compiled forward transliterator:");
    eprintln!("{}", fwd_translit);

    let json = serde_json::ser::to_string_pretty(&fwd_translit).unwrap();
    eprintln!("JSON-serialized forward transliterator:\n{json}\n--- end json forward ---");

    eprintln!("Forward transliteration:");
    eprintln!("{}", fwd_translit.transliterate(&source));
    // eprintln!("Backward translit:");
    // eprintln!("{}", bwd_translit.transliterate(&source));

    return;

    let rules = RULES;
    println!("{rules}\nHello, world!");
    // let parse_res = half_rule().easy_parse(rules);
    // let parse_res = between(spaces(), spaces(), half_rule()).easy_parse(rules);
    // let parse_res = between(spaces(), spaces(), context_like()).easy_parse(rules);
    // // let parse_res = between(spaces(), spaces(), between(char('['), char(']'), many(satisfy(|c| c != ']')))).easy_parse(rules);
    // let parse_res = between(spaces(), spaces(), literal()).easy_parse(rules);

    // let parse_res = attempt(optional(string("helloo"))).easy_parse("hellox");

    let parse_res = parse_rules().easy_parse(rules);
    match parse_res {
        Ok((rules, rem)) => {
            println!("Parsed rules:");
            pretty_print_rules(&rules);
            println!("Remaining input: {:?}", rem);
        }
        Err(e) => {
            println!("Error: {}", e);
            println!("at position: {}", e.position().translate_position(rules));
            println!(
                "so prefix: {}<-end",
                &rules[..e.position().translate_position(rules)]
            )
        }
    }
}

#[derive(Debug, Clone)]
struct UnicodeSet(String);
#[derive(Debug, Clone)]
struct Literal(String);

#[derive(Debug, Clone)]
enum UnicodeSetOrLiteral {
    UnicodeSet(UnicodeSet),
    Literal(Literal),
}

impl Display for UnicodeSetOrLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnicodeSetOrLiteral::UnicodeSet(s) => write!(f, "{}", s.0),
            UnicodeSetOrLiteral::Literal(s) => write!(f, "{}", s.0),
        }
    }
}

// actually, source keys can be UnicodeSets, but we'll just use Literals for now
#[derive(Debug, Clone)]
struct KeyLike(Vec<Literal>);

#[derive(Debug, Clone)]
struct ContextLike(Vec<UnicodeSetOrLiteral>);

#[derive(Debug)]
struct HalfRule {
    ante: Option<ContextLike>,
    key: KeyLike,
    post: Option<ContextLike>,
}

#[derive(Debug)]
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
 * direction: just → for now

 */

fn legal_top_level_char(c: char) -> bool {
    // c != '{' && c != '}' && c != '[' && c != ']' && c != '→' && c != ';' && !c.is_whitespace()
    // 'a' <= c && c <= 'z' || 'A' <= c && c <= 'Z' || '0' <= c && c <= '9'
    (c.is_ascii() && c.is_ascii_alphanumeric())
        || (!c.is_ascii() && c != '→' && c != '←' && c != '↔')
}

fn legal_in_set_char(c: char) -> bool {
    c != ']'
}

fn legal_in_single_quote_char(c: char) -> bool {
    c != '\''
}

// parses literals and also [..anything..]
fn literal<Input>() -> impl Parser<Input, Output = Literal>
where
    Input: Stream<Token = char>,
{
    let set = many(satisfy(legal_in_set_char)).map(|s: String| s);

    choice((
        many1(satisfy(legal_top_level_char)),
        recognize((string("["), set, string("]"))),
    ))
    .map(|s| Literal(s))
}

fn context_like<Input>() -> impl Parser<Input, Output = ContextLike>
where
    Input: Stream<Token = char>,
{
    // for now just a string without \{ \}
    // let literal = many1(satisfy(|c| {
    //     c != '{' && c != '}' /*&& c != '[' && c != ']'*/ && c != '→'
    // })).map(|s: String| UnicodeSetOrLiteral::Literal(Literal(s)));
    many1(between(spaces(), spaces(), literal()).map(|l| UnicodeSetOrLiteral::Literal(l)))
        .map(|v| ContextLike(v))
}

fn half_rule<Input>() -> impl Parser<Input, Output = HalfRule>
where
    Input: Stream<Token = char>,
{
    let empty_ante = char('{').map(|_| None);
    let explicit_ante = (context_like(), spaces(), char('{')).map(|(context, _, _)| Some(context));
    let ante = attempt(empty_ante.or(explicit_ante)).or(value(None));

    let key = between(
        spaces(),
        spaces(),
        many1(between(spaces(), spaces(), literal())).or(value(vec![Literal("".to_string())])),
    );

    let empty_post = char('}').map(|_| None);
    let explicit_post = (char('}'), spaces(), context_like()).map(|(_, _, context)| Some(context));
    let post = attempt(attempt(explicit_post).or(empty_post)).or(value(None));

    (ante, key, post).map(|(ante, key, post)| HalfRule {
        ante,
        key: KeyLike(key),
        post,
    })
}

fn rule<Input>() -> impl Parser<Input, Output = Rule>
where
    Input: Stream<Token = char>,
{
    let half_left = half_rule();
    let half_right = half_rule();
    let direction = between(spaces(), spaces(), char('→'));
    between(spaces(), spaces(), (half_left, direction, half_right))
        .map(|(source, _, target)| Rule { source, target })
}

fn parse_rules_<Input>() -> impl Parser<Input, Output = Vec<Rule>>
where
    Input: Stream<Token = char>,
{
    between(
        spaces(),
        spaces(),
        many(between(value(()), (spaces(), char(';'), spaces()), rule())),
    )
    // rule().map(|r| vec![r])
}

parser! {
    fn parse_rules[Input]()(Input) -> Vec<Rule>
    where [Input: Stream<Token = char>]
    {
        parse_rules_()
    }
}
