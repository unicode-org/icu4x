// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashMap;

use formatted_string_builder::SimpleFormattedStringBuilder;
use regex::Regex;

use crate::list_formatter::{Error, FieldType, ListFormatter, Pattern, Type, Width};

impl Pattern for &str {
    fn append_element_to_string(&self, list: String, element: &str) -> String {
        list + self + element
    }

    fn append_element_to_sfsb(
        &self,
        mut list: SimpleFormattedStringBuilder<FieldType>,
        element: &str,
    ) -> SimpleFormattedStringBuilder<FieldType> {
        list.append(self, FieldType::Literal);
        list.append(element, FieldType::Element);
        list
    }
}

// Allows the pattern to depend on the element that's being added.
struct ConditionalPattern {
    condition: Regex,
    standard: &'static dyn Pattern,
    special: &'static dyn Pattern,
}

impl ConditionalPattern {
    fn relevant_pattern(&self, element: &str) -> &'static dyn Pattern {
        if self.condition.is_match(element) {
            self.special
        } else {
            self.standard
        }
    }
}

impl Pattern for ConditionalPattern {
    fn append_element_to_string(&self, list: String, element: &str) -> String {
        self.relevant_pattern(element)
            .append_element_to_string(list, element)
    }

    fn append_element_to_sfsb(
        &self,
        list: SimpleFormattedStringBuilder<FieldType>,
        element: &str,
    ) -> SimpleFormattedStringBuilder<FieldType> {
        self.relevant_pattern(element)
            .append_element_to_sfsb(list, element)
    }
}

lazy_static! {
    static ref Y_OR_E: ConditionalPattern = {
        ConditionalPattern {
            // Starts with "hi" or "i" but not with "hie" nor "hia"
            condition: Regex::new("^(?i)i.*|hi|hi[^ae].*$").expect("valid"),
            standard: &" y ",
            special: &" e ",
        }
    };

    static ref O_OR_U: ConditionalPattern = {
        ConditionalPattern {
            // Starts with "o", "ho", and "8". Also "11" by itself.
            condition: Regex::new("^(?i)(o|ho|8).*|11$").expect("valid"),
            standard: &" o ",
            special: &" u ",
        }
    };

    // This static map should be a compact representation of the CLDR data. Each locale entry is
    // a 3 x 3 array (type x width) of ListFormatter objects, which contain references to static
    // strings. As lots of patterns are repeated many times, this should be more efficient than
    // having strings owned by the ListFormatters.
    static ref LIST_FORMATTERS: HashMap<&'static str, [[ListFormatter<'static>;3];3]> = {
        let mut m = HashMap::new();

        // Convenience constructor
        fn lf(first: &'static dyn Pattern, pair: &'static dyn Pattern, middle: &'static dyn Pattern, last: &'static dyn Pattern)
            -> ListFormatter<'static> { ListFormatter { first, pair, middle, last } }
        
        // m.insert(locale, [
        //     [standard, standard-narrow, standard-short],
        //     [or, or-narrow, or-short],
        //     [unit, unit-narrow, unit-short]);
        
        // ****************************************************************************************************
        // Generated code
        // ****************************************************************************************************
        m.insert("en", [
            [lf(&", ", &" and ", &", ", &", and "), lf(&", ", &" and ", &", ", &", "), lf(&", ", &" & ", &", ", &", & ")],
            [lf(&", ", &" or ", &", ", &", or "), lf(&", ", &" or ", &", ", &", or "), lf(&", ", &" or ", &", ", &", or ")],
            [lf(&", ", &", ", &", ", &", "), lf(&" ", &" ", &" ", &" "), lf(&", ", &", ", &", ", &", ")],
        ]);
        m.insert("es", [
            [lf(&", ", &*Y_OR_E, &", ", &*Y_OR_E), lf(&", ", &*Y_OR_E, &", ", &*Y_OR_E), lf(&", ", &*Y_OR_E, &", ", &*Y_OR_E)],
            [lf(&", ", &*O_OR_U, &", ", &*O_OR_U), lf(&", ", &*O_OR_U, &", ", &*O_OR_U), lf(&", ", &*O_OR_U, &", ", &*O_OR_U)],
            [lf(&", ", &*Y_OR_E, &", ", &*Y_OR_E), lf(&" ", &" ", &" ", &" "), lf(&", ", &*Y_OR_E, &", ", &", ")],
        ]);
        // ****************************************************************************************************
        // End generated code
        // ****************************************************************************************************
        m
    };
}

impl<'a> ListFormatter<'a> {
    pub fn new(locale: &str, type_: Type, width: Width) -> Result<&'a Self, Error> {
        match LIST_FORMATTERS.get(locale) {
            Some(array) => Ok(&array[match type_ {
                Type::Standard => 0,
                Type::Or => 1,
                Type::Unit => 2,
            }][match width {
                Width::Standard => 0,
                Width::Narrow => 1,
                Width::Short => 2,
            }]),
            None => Err(Error::UnknownLocale),
        }
    }
}
