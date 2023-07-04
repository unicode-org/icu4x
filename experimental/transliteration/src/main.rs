extern crate core;

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
}
