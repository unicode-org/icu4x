use combine::error::{ParseError, StdParseResult};
use combine::parser::char::{char, letter, spaces};
use combine::stream::position;
use combine::stream::{Positioned, Stream};
use combine::{between, choice, many1, parser, sep_by, EasyParser, Parser, satisfy, many, optional, attempt};

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

const RULES_EASY: &str = r#"

[ä {a \u0308}] → ae;


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
    let rules = RULES_EASY;
    let parse_res = parse_rules().easy_parse(rules);
    match parse_res {
        Ok((rules, rem)) => {
            println!("Parsed rules: {:?}", rules);
            println!("Remaining input: {:?}", rem);
        }
        Err(e) => {
            println!("Error: {}", e);
            println!("at position: {}", e.position().translate_position(rules));
        }
    }
}

#[derive(Debug)]
struct UnicodeSet(String);
#[derive(Debug)]
struct Literal(String);

#[derive(Debug)]
enum UnicodeSetOrLiteral {
    UnicodeSet(UnicodeSet),
    Literal(Literal),
}

// actually, source keys can be UnicodeSets, but we'll just use Literals for now
#[derive(Debug)]
struct KeyLike(Vec<Literal>);

#[derive(Debug)]
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

// parses literals and also [..anything..]
fn literal<Input>() -> impl Parser<Input, Output = Literal>
    where
        Input: Stream<Token = char>,
{
    choice(
        (
            many1(satisfy(|c| c != '[' && c != ']' && c != '{' && c != '}' && c != ' ')),
            between(char('['), char(']'), many(satisfy(|c| c != ']'))),
        )
    ).map(|s| Literal(s))

}

fn context_like<Input>() -> impl Parser<Input, Output = ContextLike>
where
    Input: Stream<Token = char>,
{
    // for now just a string without \{ \}
    // let literal = many1(satisfy(|c| {
    //     c != '{' && c != '}' /*&& c != '[' && c != ']'*/ && c != '→'
    // })).map(|s: String| UnicodeSetOrLiteral::Literal(Literal(s)));
    many(literal().map(|l| UnicodeSetOrLiteral::Literal(l)))
        .map(|v| ContextLike(v))
}


fn half_rule<Input>() -> impl Parser<Input, Output = HalfRule>
where
    Input: Stream<Token = char>,
{
    let ante = attempt(optional((context_like(), spaces(), char('{'))))
        .map(|optional_ante| optional_ante.map(|(ante, _, _)| ante));
    let post = attempt(optional((char('}'), spaces(), context_like())))
        .map(|optional_post| optional_post.map(|(_ , _, post)| post));
    let key = literal();
    (ante, key, post).map(|(ante, key, post)| HalfRule {
        ante,
        key: KeyLike(vec![key]),
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
    between(spaces(), spaces(), (half_left, direction, half_right)).map(|(source, _, target)| Rule {
        source,
        target,
    })
}

fn parse_rules_<Input>() -> impl Parser<Input, Output = Vec<Rule>>
where
    Input: Stream<Token = char>,
{
    // many(between(spaces(), (spaces(), char(';')), rule()))
    rule().map(|r| vec![r])
}

parser! {
    fn parse_rules[Input]()(Input) -> Vec<Rule>
    where [Input: Stream<Token = char>]
    {
        parse_rules_()
    }
}
