use regex_syntax::ParserBuilder;

#[derive(Clone, Copy, Debug)]
pub struct SyntaxConfig {
    case_insensitive: bool,
    multi_line: bool,
    dot_matches_new_line: bool,
    swap_greed: bool,
    ignore_whitespace: bool,
    unicode: bool,
    utf8: bool,
    nest_limit: u32,
    octal: bool,
}

impl SyntaxConfig {
        pub fn new() -> SyntaxConfig {
        // These defaults match the ones used in regex-syntax.
        SyntaxConfig {
            case_insensitive: false,
            multi_line: false,
            dot_matches_new_line: false,
            swap_greed: false,
            ignore_whitespace: false,
            unicode: true,
            utf8: true,
            nest_limit: 250,
            octal: false,
        }
    }

                                    pub fn case_insensitive(mut self, yes: bool) -> SyntaxConfig {
        self.case_insensitive = yes;
        self
    }

                                            pub fn multi_line(mut self, yes: bool) -> SyntaxConfig {
        self.multi_line = yes;
        self
    }

                                                                    pub fn dot_matches_new_line(mut self, yes: bool) -> SyntaxConfig {
        self.dot_matches_new_line = yes;
        self
    }

                                pub fn swap_greed(mut self, yes: bool) -> SyntaxConfig {
        self.swap_greed = yes;
        self
    }

                                    pub fn ignore_whitespace(mut self, yes: bool) -> SyntaxConfig {
        self.ignore_whitespace = yes;
        self
    }

                                                            pub fn unicode(mut self, yes: bool) -> SyntaxConfig {
        self.unicode = yes;
        self
    }

                                                            pub fn utf8(mut self, yes: bool) -> SyntaxConfig {
        self.utf8 = yes;
        self
    }

                                                                                                        pub fn nest_limit(mut self, limit: u32) -> SyntaxConfig {
        self.nest_limit = limit;
        self
    }

                                                                pub fn octal(mut self, yes: bool) -> SyntaxConfig {
        self.octal = yes;
        self
    }

        pub fn get_unicode(&self) -> bool {
        self.unicode
    }

        pub fn get_case_insensitive(&self) -> bool {
        self.case_insensitive
    }

        pub fn get_multi_line(&self) -> bool {
        self.multi_line
    }

        pub fn get_dot_matches_new_line(&self) -> bool {
        self.dot_matches_new_line
    }

        pub fn get_swap_greed(&self) -> bool {
        self.swap_greed
    }

        pub fn get_ignore_whitespace(&self) -> bool {
        self.ignore_whitespace
    }

        pub fn get_utf8(&self) -> bool {
        self.utf8
    }

        pub fn get_nest_limit(&self) -> u32 {
        self.nest_limit
    }

        pub fn get_octal(&self) -> bool {
        self.octal
    }

        pub(crate) fn apply(&self, builder: &mut ParserBuilder) {
        builder
            .unicode(self.unicode)
            .case_insensitive(self.case_insensitive)
            .multi_line(self.multi_line)
            .dot_matches_new_line(self.dot_matches_new_line)
            .swap_greed(self.swap_greed)
            .ignore_whitespace(self.ignore_whitespace)
            .allow_invalid_utf8(!self.utf8)
            .nest_limit(self.nest_limit)
            .octal(self.octal);
    }
}

impl Default for SyntaxConfig {
    fn default() -> SyntaxConfig {
        SyntaxConfig::new()
    }
}
