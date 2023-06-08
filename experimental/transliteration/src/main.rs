use combine::error::{ParseError, StdParseResult};
use combine::parser::char::{char, letter, spaces, string};
use combine::stream::position;
use combine::stream::{Positioned, Stream};
use combine::{between, choice, many1, parser, sep_by, EasyParser, Parser, satisfy, many, optional, attempt, value};
use combine::parser::combinator::recognize;
use combine::parser::repeat::Many;
use combine::parser::token::Satisfy;

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
{a } a → b };

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
    let rules = RULES_EASY;
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
            println!("Parsed rules: {:?}", rules);
            println!("Remaining input: {:?}", rem);
        }
        Err(e) => {
            println!("Error: {}", e);
            println!("at position: {}", e.position().translate_position(rules));
            println!("so prefix: {}<-end", &rules[..e.position().translate_position(rules)])
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
    c != '{' && c != '}' && c != '[' && c != ']' && c != '→' && c != ';' && !c.is_whitespace()
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

    choice(
        (
            many1(satisfy(legal_top_level_char)),
            recognize((string("["), set, string("]")))
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
    many1(between(spaces(), spaces(), literal()).map(|l| UnicodeSetOrLiteral::Literal(l)))
        .map(|v| ContextLike(v))
}


fn half_rule<Input>() -> impl Parser<Input, Output = HalfRule>
where
    Input: Stream<Token = char>,
{
    let empty_ante = char('{').map(|_| None);
    let explicit_ante = (context_like(), spaces(), char('{'))
        .map(|(context, _, _)| Some(context));
    let ante = attempt(empty_ante.or(explicit_ante)).or(value(None));

    // let ante = attempt(optional(char('{').map(|_| None).or((context_like(), spaces(), char('{'))
    //     .map(|(optional_ante, _, _)| optional_ante))))
    //     .or(value(None));
    let key = literal().or(value(Literal("".to_string())));
    let empty_post = char('}').map(|_| None);
    let explicit_post = (char('}'), spaces(), context_like())
        .map(|(_, _, context)| Some(context));
    let post = attempt(empty_post.or(explicit_post)).or(value(None));
    // let post = attempt(optional((char('}'), spaces(), context_like())))
    //     .map(|optional_post| optional_post.map(|(_ , _, post)| post))
    //     .or(value(None));
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
    between(spaces(), spaces(), many(between(value(()), (spaces(), char(';'), spaces()), rule())))
    // rule().map(|r| vec![r])
}

parser! {
    fn parse_rules[Input]()(Input) -> Vec<Rule>
    where [Input: Stream<Token = char>]
    {
        parse_rules_()
    }
}
