use std::time::Instant;

/// A input file taken from `veryl`. Its size is 157.200 Bytes.
const LEXER_INPUT: &str = include_str!("./input.vl");

// The regex generated by `parol` for `verly` grammar
const REGEX: &str = r#"(?P<G1>\r\n|\r|\n)|(?P<G2>[\s--\r\n]+)|(?P<G5>(?:(?:(?://.*(?:\r\n|\r|\n|$))|(?:(?ms)/\u{2a}.*?\u{2a}/))\s*)+)|(?P<G6>[0-9]+(?:_[0-9]+)*\.[0-9]+(?:_[0-9]+)*[eE][+-]?[0-9]+(?:_[0-9]+)*)|(?P<G7>[0-9]+(?:_[0-9]+)*\.[0-9]+(?:_[0-9]+)*)|(?P<G8>[0-9]+(?:_[0-9]+)*'[bodh][0-9a-fA-FxzXZ]+(?:_[0-9a-fA-FxzXZ]+)*)|(?P<G9>[0-9]+(?:_[0-9]+)*)|(?P<G10>'[01xzXZ])|(?P<G11>\-:)|(?P<G12>\->)|(?P<G13>\+:)|(?P<G14>\+=|-=|\*=|/=|%=|&=|\|=|\^=|<<=|>>=|<<<=|>>>=)|(?P<G15>\*\*)|(?P<G16>/|%)|(?P<G17>\+|-)|(?P<G18><<<|>>>|<<|>>)|(?P<G19><=|>=|<|>)|(?P<G20>===|==\?|!==|!=\?|==|!=)|(?P<G21>&&)|(?P<G22>\|\|)|(?P<G23>&)|(?P<G24>\^~|\^|~\^)|(?P<G25>\|)|(?P<G26>~&|~\||!|~)|(?P<G27>::)|(?P<G28>:)|(?P<G29>,)|(?P<G30>\$)|(?P<G31>\.\.)|(?P<G32>\.)|(?P<G33>=)|(?P<G34>\#)|(?P<G35>\{)|(?P<G36>\[)|(?P<G37>\()|(?P<G38>\})|(?P<G39>\])|(?P<G40>\))|(?P<G41>;)|(?P<G42>\*)|(?P<G43>(?-u:\b)always_comb(?-u:\b))|(?P<G44>(?-u:\b)always_ff(?-u:\b))|(?P<G45>(?-u:\b)assign(?-u:\b))|(?P<G46>(?-u:\b)async_high(?-u:\b))|(?P<G47>(?-u:\b)async_low(?-u:\b))|(?P<G48>(?-u:\b)as(?-u:\b))|(?P<G49>(?-u:\b)bit(?-u:\b))|(?P<G50>(?-u:\b)case(?-u:\b))|(?P<G51>(?-u:\b)default(?-u:\b))|(?P<G52>(?-u:\b)else(?-u:\b))|(?P<G53>(?-u:\b)enum(?-u:\b))|(?P<G54>(?-u:\b)export(?-u:\b))|(?P<G55>(?-u:\b)f32(?-u:\b))|(?P<G56>(?-u:\b)f64(?-u:\b))|(?P<G57>(?-u:\b)for(?-u:\b))|(?P<G58>(?-u:\b)function(?-u:\b))|(?P<G59>(?-u:\b)i32(?-u:\b))|(?P<G60>(?-u:\b)i64(?-u:\b))|(?P<G61>(?-u:\b)if_reset(?-u:\b))|(?P<G62>(?-u:\b)if(?-u:\b))|(?P<G63>(?-u:\b)import(?-u:\b))|(?P<G64>(?-u:\b)inout(?-u:\b))|(?P<G65>(?-u:\b)input(?-u:\b))|(?P<G66>(?-u:\b)inst(?-u:\b))|(?P<G67>(?-u:\b)interface(?-u:\b))|(?P<G68>(?-u:\b)in(?-u:\b))|(?P<G69>(?-u:\b)localparam(?-u:\b))|(?P<G70>(?-u:\b)logic(?-u:\b))|(?P<G71>(?-u:\b)modport(?-u:\b))|(?P<G72>(?-u:\b)module(?-u:\b))|(?P<G73>(?-u:\b)negedge(?-u:\b))|(?P<G74>(?-u:\b)output(?-u:\b))|(?P<G75>(?-u:\b)package(?-u:\b))|(?P<G76>(?-u:\b)parameter(?-u:\b))|(?P<G77>(?-u:\b)posedge(?-u:\b))|(?P<G78>(?-u:\b)ref(?-u:\b))|(?P<G79>(?-u:\b)repeat(?-u:\b))|(?P<G80>(?-u:\b)return(?-u:\b))|(?P<G81>(?-u:\b)step(?-u:\b))|(?P<G82>(?-u:\b)struct(?-u:\b))|(?P<G83>(?-u:\b)sync_high(?-u:\b))|(?P<G84>(?-u:\b)sync_low(?-u:\b))|(?P<G85>(?-u:\b)tri(?-u:\b))|(?P<G86>(?-u:\b)u32(?-u:\b))|(?P<G87>(?-u:\b)u64(?-u:\b))|(?P<G88>(?-u:\b)var(?-u:\b))|(?P<G89>[a-zA-Z_][0-9a-zA-Z_]*)|(?P<G90>.)"#;

const PATTERNS: &[&str] = &[
    r"\r\n|\r|\n",
    r"[\s--\r\n]+",
    r"\w(?-u:\b)\w",
    r"\w(?-u:\b)\w",
    r"(?:(?:(?://.*(?:\r\n|\r|\n|$))|(?:(?ms)/\u{2a}.*?\u{2a}/))\s*)+",
    r"[0-9]+(?:_[0-9]+)*\.[0-9]+(?:_[0-9]+)*[eE][+-]?[0-9]+(?:_[0-9]+)*",
    r"[0-9]+(?:_[0-9]+)*\.[0-9]+(?:_[0-9]+)*",
    r"[0-9]+(?:_[0-9]+)*'[bodh][0-9a-fA-FxzXZ]+(?:_[0-9a-fA-FxzXZ]+)*",
    r"[0-9]+(?:_[0-9]+)*",
    r"'[01xzXZ]",
    r"\-:",
    r"\->",
    r"\+:",
    r"\+=|-=|\*=|/=|%=|&=|\|=|\^=|<<=|>>=|<<<=|>>>=",
    r"\*\*",
    r"/|%",
    r"\+|-",
    r"<<<|>>>|<<|>>",
    r"<=|>=|<|>",
    r"===|==\?|!==|!=\?|==|!=",
    r"&&",
    r"\|\|",
    r"&",
    r"\^~|\^|~\^",
    r"\|",
    r"~&|~\||!|~",
    r"::",
    r":",
    r",",
    r"\$",
    r"\.\.",
    r"\.",
    r"=",
    r"\#",
    r"\{",
    r"\[",
    r"\(",
    r"\}",
    r"\]",
    r"\)",
    r";",
    r"\*",
    r"(?-u:\b)always_comb(?-u:\b)",
    r"(?-u:\b)always_ff(?-u:\b)",
    r"(?-u:\b)assign(?-u:\b)",
    r"(?-u:\b)async_high(?-u:\b)",
    r"(?-u:\b)async_low(?-u:\b)",
    r"(?-u:\b)as(?-u:\b)",
    r"(?-u:\b)bit(?-u:\b)",
    r"(?-u:\b)case(?-u:\b)",
    r"(?-u:\b)default(?-u:\b)",
    r"(?-u:\b)else(?-u:\b)",
    r"(?-u:\b)enum(?-u:\b)",
    r"(?-u:\b)export(?-u:\b)",
    r"(?-u:\b)f32(?-u:\b)",
    r"(?-u:\b)f64(?-u:\b)",
    r"(?-u:\b)for(?-u:\b)",
    r"(?-u:\b)function(?-u:\b)",
    r"(?-u:\b)i32(?-u:\b)",
    r"(?-u:\b)i64(?-u:\b)",
    r"(?-u:\b)if_reset(?-u:\b)",
    r"(?-u:\b)if(?-u:\b)",
    r"(?-u:\b)import(?-u:\b)",
    r"(?-u:\b)inout(?-u:\b)",
    r"(?-u:\b)input(?-u:\b)",
    r"(?-u:\b)inst(?-u:\b)",
    r"(?-u:\b)interface(?-u:\b)",
    r"(?-u:\b)in(?-u:\b)",
    r"(?-u:\b)localparam(?-u:\b)",
    r"(?-u:\b)logic(?-u:\b)",
    r"(?-u:\b)modport(?-u:\b)",
    r"(?-u:\b)module(?-u:\b)",
    r"(?-u:\b)negedge(?-u:\b)",
    r"(?-u:\b)output(?-u:\b)",
    r"(?-u:\b)package(?-u:\b)",
    r"(?-u:\b)parameter(?-u:\b)",
    r"(?-u:\b)posedge(?-u:\b)",
    r"(?-u:\b)ref(?-u:\b)",
    r"(?-u:\b)repeat(?-u:\b)",
    r"(?-u:\b)return(?-u:\b)",
    r"(?-u:\b)step(?-u:\b)",
    r"(?-u:\b)struct(?-u:\b)",
    r"(?-u:\b)sync_high(?-u:\b)",
    r"(?-u:\b)sync_low(?-u:\b)",
    r"(?-u:\b)tri(?-u:\b)",
    r"(?-u:\b)u32(?-u:\b)",
    r"(?-u:\b)u64(?-u:\b)",
    r"(?-u:\b)var(?-u:\b)",
    r"[a-zA-Z_][0-9a-zA-Z_]*",
    r".",
];

fn test_regex() {
    let regex = regex::Regex::new(REGEX).unwrap();
    let capture_iter = regex.captures_iter(LEXER_INPUT);
    let mut token_count = 0usize;
    let now = Instant::now();
    for _ in capture_iter {
        token_count += 1;
    }
    let elapsed_time = now.elapsed();
    println!(
        "Tokenizating with regex::Regex:\n{} tokens took {} milliseconds.\n",
        token_count,
        elapsed_time.as_millis()
    );
}

fn test_regex_automata() {
    // We don't disable Unicode and UTF-8 globally but for ceratin constructs in our regex patterns,
    // e.g. we use `(?-u:\b)` instedd of `\b`
    let regex = regex_automata::dfa::regex::Regex::builder()
        // .syntax(regex_automata::SyntaxConfig::new().unicode(false).utf8(false))
        .build_many(PATTERNS)
        .unwrap();
    let find_iter = regex.find_leftmost_iter(LEXER_INPUT.as_bytes());
    let mut token_count = 0usize;
    let now = Instant::now();
    for _ in find_iter {
        token_count += 1;
    }
    let elapsed_time = now.elapsed();
    println!(
        "Tokenizating with regex_automata::dfa::regex::Regex:\n{} tokens took {} milliseconds.\n",
        token_count,
        elapsed_time.as_millis()
    );
}

fn main() {
    test_regex();
    test_regex_automata();
}
