// @generated
#![cfg(feature = "icu_datetime")]
type DataStruct = < :: icu_datetime :: provider :: calendar :: TimeSymbolsV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[
        ("ar", AR_AR_EG),
        ("ar-EG", AR_AR_EG),
        ("bn", BN_CCP_UND),
        ("ccp", BN_CCP_UND),
        ("en", EN),
        ("en-001", EN_001_EN_ZA),
        ("en-ZA", EN_001_EN_ZA),
        ("es", ES),
        ("es-AR", ES_AR),
        ("fil", FIL),
        ("fr", FR),
        ("ja", JA),
        ("ru", RU),
        ("sr", SR_SR_CYRL),
        ("sr-Cyrl", SR_SR_CYRL),
        ("sr-Latn", SR_LATN),
        ("th", TH),
        ("tr", TR),
        ("und", BN_CCP_UND),
    ]);
static AR_AR_EG: &DataStruct = &::icu_datetime::provider::calendar::TimeSymbolsV1 {
    day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
        format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("ص"),
                pm: alloc::borrow::Cow::Borrowed("م"),
                noon: None,
                midnight: None,
            },
            narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("ص"),
                pm: alloc::borrow::Cow::Borrowed("م"),
                noon: None,
                midnight: None,
            },
            short: None,
            wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("ص"),
                pm: alloc::borrow::Cow::Borrowed("م"),
                noon: None,
                midnight: None,
            },
        },
        stand_alone: Some(
            ::icu_datetime::provider::calendar::day_periods::StandAloneWidthsV1 {
                abbreviated: None,
                narrow: None,
                short: None,
                wide: Some(::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: alloc::borrow::Cow::Borrowed("صباح\u{64b}ا"),
                    pm: alloc::borrow::Cow::Borrowed("مساء\u{64b}"),
                    noon: None,
                    midnight: None,
                }),
            },
        ),
    },
};
static BN_CCP_UND: &DataStruct = &::icu_datetime::provider::calendar::TimeSymbolsV1 {
    day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
        format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("AM"),
                pm: alloc::borrow::Cow::Borrowed("PM"),
                noon: None,
                midnight: None,
            },
            narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("AM"),
                pm: alloc::borrow::Cow::Borrowed("PM"),
                noon: None,
                midnight: None,
            },
            short: None,
            wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("AM"),
                pm: alloc::borrow::Cow::Borrowed("PM"),
                noon: None,
                midnight: None,
            },
        },
        stand_alone: None,
    },
};
static EN: &DataStruct = &::icu_datetime::provider::calendar::TimeSymbolsV1 {
    day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
        format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("AM"),
                pm: alloc::borrow::Cow::Borrowed("PM"),
                noon: Some(alloc::borrow::Cow::Borrowed("noon")),
                midnight: Some(alloc::borrow::Cow::Borrowed("midnight")),
            },
            narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("a"),
                pm: alloc::borrow::Cow::Borrowed("p"),
                noon: Some(alloc::borrow::Cow::Borrowed("n")),
                midnight: Some(alloc::borrow::Cow::Borrowed("mi")),
            },
            short: None,
            wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("AM"),
                pm: alloc::borrow::Cow::Borrowed("PM"),
                noon: Some(alloc::borrow::Cow::Borrowed("noon")),
                midnight: Some(alloc::borrow::Cow::Borrowed("midnight")),
            },
        },
        stand_alone: Some(
            ::icu_datetime::provider::calendar::day_periods::StandAloneWidthsV1 {
                abbreviated: None,
                narrow: Some(::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: alloc::borrow::Cow::Borrowed("AM"),
                    pm: alloc::borrow::Cow::Borrowed("PM"),
                    noon: Some(alloc::borrow::Cow::Borrowed("noon")),
                    midnight: Some(alloc::borrow::Cow::Borrowed("midnight")),
                }),
                short: None,
                wide: None,
            },
        ),
    },
};
static EN_001_EN_ZA: &DataStruct = &::icu_datetime::provider::calendar::TimeSymbolsV1 {
    day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
        format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("am"),
                pm: alloc::borrow::Cow::Borrowed("pm"),
                noon: Some(alloc::borrow::Cow::Borrowed("noon")),
                midnight: Some(alloc::borrow::Cow::Borrowed("midnight")),
            },
            narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("a"),
                pm: alloc::borrow::Cow::Borrowed("p"),
                noon: Some(alloc::borrow::Cow::Borrowed("n")),
                midnight: Some(alloc::borrow::Cow::Borrowed("mi")),
            },
            short: None,
            wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("am"),
                pm: alloc::borrow::Cow::Borrowed("pm"),
                noon: Some(alloc::borrow::Cow::Borrowed("noon")),
                midnight: Some(alloc::borrow::Cow::Borrowed("midnight")),
            },
        },
        stand_alone: Some(
            ::icu_datetime::provider::calendar::day_periods::StandAloneWidthsV1 {
                abbreviated: None,
                narrow: Some(::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: alloc::borrow::Cow::Borrowed("am"),
                    pm: alloc::borrow::Cow::Borrowed("pm"),
                    noon: Some(alloc::borrow::Cow::Borrowed("noon")),
                    midnight: Some(alloc::borrow::Cow::Borrowed("midnight")),
                }),
                short: None,
                wide: None,
            },
        ),
    },
};
static ES: &DataStruct = &::icu_datetime::provider::calendar::TimeSymbolsV1 {
    day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
        format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("a.\u{202f}m."),
                pm: alloc::borrow::Cow::Borrowed("p.\u{202f}m."),
                noon: Some(alloc::borrow::Cow::Borrowed("del mediodía")),
                midnight: None,
            },
            narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("a.\u{202f}m."),
                pm: alloc::borrow::Cow::Borrowed("p.\u{202f}m."),
                noon: Some(alloc::borrow::Cow::Borrowed("del mediodía")),
                midnight: None,
            },
            short: None,
            wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("a.\u{a0}m."),
                pm: alloc::borrow::Cow::Borrowed("p.\u{a0}m."),
                noon: Some(alloc::borrow::Cow::Borrowed("del mediodía")),
                midnight: None,
            },
        },
        stand_alone: Some(
            ::icu_datetime::provider::calendar::day_periods::StandAloneWidthsV1 {
                abbreviated: Some(::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: alloc::borrow::Cow::Borrowed("a.\u{202f}m."),
                    pm: alloc::borrow::Cow::Borrowed("p.\u{202f}m."),
                    noon: Some(alloc::borrow::Cow::Borrowed("mediodía")),
                    midnight: None,
                }),
                narrow: Some(::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: alloc::borrow::Cow::Borrowed("a.\u{202f}m."),
                    pm: alloc::borrow::Cow::Borrowed("p.\u{202f}m."),
                    noon: Some(alloc::borrow::Cow::Borrowed("mediodía")),
                    midnight: None,
                }),
                short: None,
                wide: Some(::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: alloc::borrow::Cow::Borrowed("a.\u{a0}m."),
                    pm: alloc::borrow::Cow::Borrowed("p.\u{a0}m."),
                    noon: Some(alloc::borrow::Cow::Borrowed("mediodía")),
                    midnight: None,
                }),
            },
        ),
    },
};
static ES_AR: &DataStruct = &::icu_datetime::provider::calendar::TimeSymbolsV1 {
    day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
        format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("a.\u{202f}m."),
                pm: alloc::borrow::Cow::Borrowed("p.\u{202f}m."),
                noon: Some(alloc::borrow::Cow::Borrowed("mediodía")),
                midnight: None,
            },
            narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("a.\u{202f}m."),
                pm: alloc::borrow::Cow::Borrowed("p.\u{202f}m."),
                noon: Some(alloc::borrow::Cow::Borrowed("del mediodía")),
                midnight: None,
            },
            short: None,
            wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("a.\u{a0}m."),
                pm: alloc::borrow::Cow::Borrowed("p.\u{a0}m."),
                noon: Some(alloc::borrow::Cow::Borrowed("mediodía")),
                midnight: None,
            },
        },
        stand_alone: Some(
            ::icu_datetime::provider::calendar::day_periods::StandAloneWidthsV1 {
                abbreviated: None,
                narrow: Some(::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: alloc::borrow::Cow::Borrowed("a.\u{202f}m."),
                    pm: alloc::borrow::Cow::Borrowed("p.\u{202f}m."),
                    noon: Some(alloc::borrow::Cow::Borrowed("m.")),
                    midnight: None,
                }),
                short: None,
                wide: None,
            },
        ),
    },
};
static FIL: &DataStruct = &::icu_datetime::provider::calendar::TimeSymbolsV1 {
    day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
        format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("AM"),
                pm: alloc::borrow::Cow::Borrowed("PM"),
                noon: Some(alloc::borrow::Cow::Borrowed("tanghaling-tapat")),
                midnight: Some(alloc::borrow::Cow::Borrowed("hatinggabi")),
            },
            narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("am"),
                pm: alloc::borrow::Cow::Borrowed("pm"),
                noon: Some(alloc::borrow::Cow::Borrowed("tanghaling-tapat")),
                midnight: Some(alloc::borrow::Cow::Borrowed("hatinggabi")),
            },
            short: None,
            wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("AM"),
                pm: alloc::borrow::Cow::Borrowed("PM"),
                noon: Some(alloc::borrow::Cow::Borrowed("tanghaling-tapat")),
                midnight: Some(alloc::borrow::Cow::Borrowed("hatinggabi")),
            },
        },
        stand_alone: Some(
            ::icu_datetime::provider::calendar::day_periods::StandAloneWidthsV1 {
                abbreviated: None,
                narrow: Some(::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: alloc::borrow::Cow::Borrowed("AM"),
                    pm: alloc::borrow::Cow::Borrowed("PM"),
                    noon: Some(alloc::borrow::Cow::Borrowed("tanghaling-tapat")),
                    midnight: Some(alloc::borrow::Cow::Borrowed("hatinggabi")),
                }),
                short: None,
                wide: None,
            },
        ),
    },
};
static FR: &DataStruct = &::icu_datetime::provider::calendar::TimeSymbolsV1 {
    day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
        format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("AM"),
                pm: alloc::borrow::Cow::Borrowed("PM"),
                noon: Some(alloc::borrow::Cow::Borrowed("midi")),
                midnight: Some(alloc::borrow::Cow::Borrowed("minuit")),
            },
            narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("AM"),
                pm: alloc::borrow::Cow::Borrowed("PM"),
                noon: Some(alloc::borrow::Cow::Borrowed("midi")),
                midnight: Some(alloc::borrow::Cow::Borrowed("minuit")),
            },
            short: None,
            wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("AM"),
                pm: alloc::borrow::Cow::Borrowed("PM"),
                noon: Some(alloc::borrow::Cow::Borrowed("midi")),
                midnight: Some(alloc::borrow::Cow::Borrowed("minuit")),
            },
        },
        stand_alone: None,
    },
};
static JA: &DataStruct = &::icu_datetime::provider::calendar::TimeSymbolsV1 {
    day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
        format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("午前"),
                pm: alloc::borrow::Cow::Borrowed("午後"),
                noon: Some(alloc::borrow::Cow::Borrowed("正午")),
                midnight: Some(alloc::borrow::Cow::Borrowed("真夜中")),
            },
            narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("午前"),
                pm: alloc::borrow::Cow::Borrowed("午後"),
                noon: Some(alloc::borrow::Cow::Borrowed("正午")),
                midnight: Some(alloc::borrow::Cow::Borrowed("真夜中")),
            },
            short: None,
            wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("午前"),
                pm: alloc::borrow::Cow::Borrowed("午後"),
                noon: Some(alloc::borrow::Cow::Borrowed("正午")),
                midnight: Some(alloc::borrow::Cow::Borrowed("真夜中")),
            },
        },
        stand_alone: None,
    },
};
static RU: &DataStruct = &::icu_datetime::provider::calendar::TimeSymbolsV1 {
    day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
        format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("AM"),
                pm: alloc::borrow::Cow::Borrowed("PM"),
                noon: Some(alloc::borrow::Cow::Borrowed("полд.")),
                midnight: Some(alloc::borrow::Cow::Borrowed("полн.")),
            },
            narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("AM"),
                pm: alloc::borrow::Cow::Borrowed("PM"),
                noon: Some(alloc::borrow::Cow::Borrowed("полд.")),
                midnight: Some(alloc::borrow::Cow::Borrowed("полн.")),
            },
            short: None,
            wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("AM"),
                pm: alloc::borrow::Cow::Borrowed("PM"),
                noon: Some(alloc::borrow::Cow::Borrowed("полдень")),
                midnight: Some(alloc::borrow::Cow::Borrowed("полночь")),
            },
        },
        stand_alone: None,
    },
};
static SR_LATN: &DataStruct = &::icu_datetime::provider::calendar::TimeSymbolsV1 {
    day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
        format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("AM"),
                pm: alloc::borrow::Cow::Borrowed("PM"),
                noon: Some(alloc::borrow::Cow::Borrowed("podne")),
                midnight: Some(alloc::borrow::Cow::Borrowed("ponoć")),
            },
            narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("AM"),
                pm: alloc::borrow::Cow::Borrowed("PM"),
                noon: Some(alloc::borrow::Cow::Borrowed("podne")),
                midnight: Some(alloc::borrow::Cow::Borrowed("ponoć")),
            },
            short: None,
            wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("AM"),
                pm: alloc::borrow::Cow::Borrowed("PM"),
                noon: Some(alloc::borrow::Cow::Borrowed("podne")),
                midnight: Some(alloc::borrow::Cow::Borrowed("ponoć")),
            },
        },
        stand_alone: Some(
            ::icu_datetime::provider::calendar::day_periods::StandAloneWidthsV1 {
                abbreviated: None,
                narrow: Some(::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: alloc::borrow::Cow::Borrowed("pre\u{202f}podne"),
                    pm: alloc::borrow::Cow::Borrowed("po\u{202f}podne"),
                    noon: Some(alloc::borrow::Cow::Borrowed("podne")),
                    midnight: Some(alloc::borrow::Cow::Borrowed("ponoć")),
                }),
                short: None,
                wide: None,
            },
        ),
    },
};
static SR_SR_CYRL: &DataStruct = &::icu_datetime::provider::calendar::TimeSymbolsV1 {
    day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
        format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("AM"),
                pm: alloc::borrow::Cow::Borrowed("PM"),
                noon: Some(alloc::borrow::Cow::Borrowed("подне")),
                midnight: Some(alloc::borrow::Cow::Borrowed("поноћ")),
            },
            narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("AM"),
                pm: alloc::borrow::Cow::Borrowed("PM"),
                noon: Some(alloc::borrow::Cow::Borrowed("подне")),
                midnight: Some(alloc::borrow::Cow::Borrowed("поноћ")),
            },
            short: None,
            wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("AM"),
                pm: alloc::borrow::Cow::Borrowed("PM"),
                noon: Some(alloc::borrow::Cow::Borrowed("подне")),
                midnight: Some(alloc::borrow::Cow::Borrowed("поноћ")),
            },
        },
        stand_alone: Some(
            ::icu_datetime::provider::calendar::day_periods::StandAloneWidthsV1 {
                abbreviated: None,
                narrow: Some(::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: alloc::borrow::Cow::Borrowed("пре\u{202f}подне"),
                    pm: alloc::borrow::Cow::Borrowed("по\u{202f}подне"),
                    noon: Some(alloc::borrow::Cow::Borrowed("подне")),
                    midnight: Some(alloc::borrow::Cow::Borrowed("поноћ")),
                }),
                short: None,
                wide: None,
            },
        ),
    },
};
static TH: &DataStruct = &::icu_datetime::provider::calendar::TimeSymbolsV1 {
    day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
        format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("AM"),
                pm: alloc::borrow::Cow::Borrowed("PM"),
                noon: Some(alloc::borrow::Cow::Borrowed("เท\u{e35}\u{e48}ยง")),
                midnight: Some(alloc::borrow::Cow::Borrowed("เท\u{e35}\u{e48}ยงค\u{e37}น")),
            },
            narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("a"),
                pm: alloc::borrow::Cow::Borrowed("p"),
                noon: Some(alloc::borrow::Cow::Borrowed("เท\u{e35}\u{e48}ยง")),
                midnight: Some(alloc::borrow::Cow::Borrowed("เท\u{e35}\u{e48}ยงค\u{e37}น")),
            },
            short: None,
            wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("ก\u{e48}อนเท\u{e35}\u{e48}ยง"),
                pm: alloc::borrow::Cow::Borrowed("หล\u{e31}งเท\u{e35}\u{e48}ยง"),
                noon: Some(alloc::borrow::Cow::Borrowed("เท\u{e35}\u{e48}ยง")),
                midnight: Some(alloc::borrow::Cow::Borrowed("เท\u{e35}\u{e48}ยงค\u{e37}น")),
            },
        },
        stand_alone: Some(
            ::icu_datetime::provider::calendar::day_periods::StandAloneWidthsV1 {
                abbreviated: Some(::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: alloc::borrow::Cow::Borrowed("ก\u{e48}อนเท\u{e35}\u{e48}ยง"),
                    pm: alloc::borrow::Cow::Borrowed("หล\u{e31}งเท\u{e35}\u{e48}ยง"),
                    noon: Some(alloc::borrow::Cow::Borrowed("เท\u{e35}\u{e48}ยง")),
                    midnight: Some(alloc::borrow::Cow::Borrowed("เท\u{e35}\u{e48}ยงค\u{e37}น")),
                }),
                narrow: Some(::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: alloc::borrow::Cow::Borrowed("ก\u{e48}อนเท\u{e35}\u{e48}ยง"),
                    pm: alloc::borrow::Cow::Borrowed("หล\u{e31}งเท\u{e35}\u{e48}ยง"),
                    noon: Some(alloc::borrow::Cow::Borrowed("เท\u{e35}\u{e48}ยง")),
                    midnight: Some(alloc::borrow::Cow::Borrowed("เท\u{e35}\u{e48}ยงค\u{e37}น")),
                }),
                short: None,
                wide: None,
            },
        ),
    },
};
static TR: &DataStruct = &::icu_datetime::provider::calendar::TimeSymbolsV1 {
    day_periods: ::icu_datetime::provider::calendar::day_periods::ContextsV1 {
        format: ::icu_datetime::provider::calendar::day_periods::FormatWidthsV1 {
            abbreviated: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("ÖÖ"),
                pm: alloc::borrow::Cow::Borrowed("ÖS"),
                noon: Some(alloc::borrow::Cow::Borrowed("öğle")),
                midnight: Some(alloc::borrow::Cow::Borrowed("gece yarısı")),
            },
            narrow: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("öö"),
                pm: alloc::borrow::Cow::Borrowed("ös"),
                noon: Some(alloc::borrow::Cow::Borrowed("ö")),
                midnight: Some(alloc::borrow::Cow::Borrowed("gece")),
            },
            short: None,
            wide: ::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                am: alloc::borrow::Cow::Borrowed("ÖÖ"),
                pm: alloc::borrow::Cow::Borrowed("ÖS"),
                noon: Some(alloc::borrow::Cow::Borrowed("öğle")),
                midnight: Some(alloc::borrow::Cow::Borrowed("gece yarısı")),
            },
        },
        stand_alone: Some(
            ::icu_datetime::provider::calendar::day_periods::StandAloneWidthsV1 {
                abbreviated: None,
                narrow: Some(::icu_datetime::provider::calendar::day_periods::SymbolsV1 {
                    am: alloc::borrow::Cow::Borrowed("ÖÖ"),
                    pm: alloc::borrow::Cow::Borrowed("ÖS"),
                    noon: Some(alloc::borrow::Cow::Borrowed("öğle")),
                    midnight: Some(alloc::borrow::Cow::Borrowed("gece yarısı")),
                }),
                short: None,
                wide: None,
            },
        ),
    },
};
