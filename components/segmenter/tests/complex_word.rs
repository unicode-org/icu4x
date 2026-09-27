// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_segmenter::*;
use icu_segmenter::{WordSegmenter, options::WordBreakInvariantOptions};

include!("helpers.rs.raw");

// Additional word segmenter tests with complex string.

#[test]
fn word_break_th() {
    for segmenter in [
        WordSegmenter::new_auto(WordBreakInvariantOptions::default()),
        WordSegmenter::new_lstm(WordBreakInvariantOptions::default()),
        {
            let mut s = WordSegmenter::new_neo_for_non_complex_scripts(
                WordBreakInvariantOptions::default(),
            );
            s.load_auto();
            s
        },
        {
            let mut s = WordSegmenter::new_neo_for_non_complex_scripts(
                WordBreakInvariantOptions::default(),
            );
            s.load_auto();
            s
        },
    ] {
        // http://wpt.live/css/css-text/word-break/word-break-normal-th-000.html
        let s = "ภาษาไทยภาษาไทย";
        let expected = ["ภาษา", "ไทย", "ภาษา", "ไทย"];

        check_word(s, &expected, segmenter);

        // Combine non-Thai and Thai.
        let s = "aภาษาไทยภาษาไทยb";
        let expected = ["a", "ภาษา", "ไทย", "ภาษา", "ไทย", "b"];

        check_word(s, &expected, segmenter);

        // Expected values verified against ICU 78 (`Intl.Segmenter` with
        // `granularity: "word"`).
        let s = "โรงพยาบาล";
        let expected = ["โรง", "พยาบาล"];

        check_word(s, &expected, segmenter);

        let s = "น้ำตาลทราย";
        let expected = ["น้ำตาล", "ทราย"];

        check_word(s, &expected, segmenter);

        let s = "น้ำ";
        let expected = ["น้ำ"];

        check_word(s, &expected, segmenter);

        let s = "กำลัง";
        let expected = ["กำลัง"];

        check_word(s, &expected, segmenter);

        let s = "เก้า";
        let expected = ["เก้า"];

        check_word(s, &expected, segmenter);

        let s = "ดำเนินการ";
        let expected = ["ดำเนิน", "การ"];

        check_word(s, &expected, segmenter);

        let s = "นี่คือ ICU4X สำหรับภาษาไทย";
        let expected = ["นี่", "คือ", " ", "ICU4X", " ", "สำหรับ", "ภาษา", "ไทย"];

        check_word(s, &expected, segmenter);

        let s = "ราคา 1,250 บาท รวม VAT แล้ว";
        let expected = [
            "ราคา",
            " ",
            "1,250",
            " ",
            "บาท",
            " ",
            "รวม",
            " ",
            "VAT",
            " ",
            "แล้ว",
        ];

        check_word(s, &expected, segmenter);

        let s = "๑๐๐ บาท";
        let expected = ["๑๐๐", " ", "บาท"];

        check_word(s, &expected, segmenter);

        let s = "เด็ก ๆ วิ่งเล่น";
        let expected = ["เด็ก", " ", "ๆ", " ", "วิ่ง", "เล่น"];

        check_word(s, &expected, segmenter);

        let s = "เขาไปตลาดเมื่อวานนี้";
        let expected = ["เขา", "ไป", "ตลาด", "เมื่อ", "วาน", "นี้"];

        check_word(s, &expected, segmenter);

        let s = "ปัญญาประดิษฐ์";
        let expected = ["ปัญญา", "ประดิษฐ์"];

        check_word(s, &expected, segmenter);

        let s = "ข้อมูลส่วนบุคคล";
        let expected = ["ข้อมูล", "ส่วน", "บุคคล"];

        check_word(s, &expected, segmenter);

        let s = "https://example.com/ภาษาไทย";
        let expected = ["https", ":", "/", "/", "example.com", "/", "ภาษา", "ไทย"];

        check_word(s, &expected, segmenter);

        let s = "อีเมล test@example.com";
        let expected = ["อีเมล", " ", "test", "@", "example.com"];

        check_word(s, &expected, segmenter);

        let s = "Hello สวัสดี World";
        let expected = ["Hello", " ", "สวัสดี", " ", "World"];

        check_word(s, &expected, segmenter);

        let s = "iPad รุ่นใหม่";
        let expected = ["iPad", " ", "รุ่น", "ใหม่"];

        check_word(s, &expected, segmenter);

        let s = "COVID-19 ระบาด";
        let expected = ["COVID", "-", "19", " ", "ระบาด"];

        check_word(s, &expected, segmenter);

        let s = "น้ำแข็งไส";
        let expected = ["น้ำ", "แข็ง", "ไส"];

        check_word(s, &expected, segmenter);

        let s = "ลูกชิ้นปลา";
        let expected = ["ลูก", "ชิ้น", "ปลา"];

        check_word(s, &expected, segmenter);

        let s = "ผัดไทยกุ้งสด";
        let expected = ["ผัด", "ไทย", "กุ้ง", "สด"];

        check_word(s, &expected, segmenter);

        let s = "ประสิทธิภาพ";
        let expected = ["ประสิทธิภาพ"];

        check_word(s, &expected, segmenter);

        let s = "วิทยาศาสตร์และเทคโนโลยี";
        let expected = ["วิทยาศาสตร์", "และ", "เทคโนโลยี"];

        check_word(s, &expected, segmenter);

        let s = "เศรษฐกิจพอเพียง";
        let expected = ["เศรษฐกิจ", "พอ", "เพียง"];

        check_word(s, &expected, segmenter);

        let s = "ประชาธิปไตย";
        let expected = ["ประชาธิปไตย"];

        check_word(s, &expected, segmenter);

        let s = "สุขภาพจิต";
        let expected = ["สุขภาพ", "จิต"];

        check_word(s, &expected, segmenter);
    }
}

// The dictionary model is intended to match ICU4C word segmentation for Thai,
// while the LSTM model may segment words differently. Expected values verified
// against ICU 78 (`Intl.Segmenter` with `granularity: "word"`).
#[test]
fn word_break_th_dictionary() {
    for segmenter in [
        WordSegmenter::new_dictionary(WordBreakInvariantOptions::default()),
        {
            let mut s = WordSegmenter::new_neo_for_non_complex_scripts(
                WordBreakInvariantOptions::default(),
            );
            s.load_dictionary();
            s
        },
    ] {
        let s = "กระเพรา";
        let expected = ["กระ", "เพรา"];

        check_word(s, &expected, segmenter);

        let s = "รถไฟฟ้า";
        let expected = ["รถไฟฟ้า"];

        check_word(s, &expected, segmenter);

        let s = "ซอฟต์แวร์";
        let expected = ["ซอฟต์แวร์"];

        check_word(s, &expected, segmenter);

        let s = "บล็อกเชน";
        let expected = ["บล็อก", "เชน"];

        check_word(s, &expected, segmenter);

        let s = "กรุงเทพฯ";
        let expected = ["กรุงเทพฯ"];

        check_word(s, &expected, segmenter);

        let s = "กรุงเทพมหานคร";
        let expected = ["กรุงเทพมหานคร"];

        check_word(s, &expected, segmenter);

        let s = "ประเทศไทย";
        let expected = ["ประเทศไทย"];

        check_word(s, &expected, segmenter);

        let s = "ฯลฯ";
        let expected = ["ฯลฯ"];

        check_word(s, &expected, segmenter);

        let s = "ทดสอบคำซ้ำ ๆ และเครื่องหมาย ฯลฯ";
        let expected = [
            "ทดสอบ",
            "คำ",
            "ซ้ำ",
            " ",
            "ๆ",
            " ",
            "และ",
            "เครื่องหมาย",
            " ",
            "ฯลฯ",
        ];

        check_word(s, &expected, segmenter);

        let s = "เปิดใช้ ICU4X เวอร์ชัน 2";
        let expected = ["เปิด", "ใช้", " ", "ICU4X", " ", "เวอร์ชัน", " ", "2"];

        check_word(s, &expected, segmenter);

        let s = "สวัสดีครับ ผมชื่อสมชาย";
        let expected = ["สวัสดี", "ครับ", " ", "ผม", "ชื่อ", "สมชาย"];

        check_word(s, &expected, segmenter);

        let s = "การพัฒนาซอฟต์แวร์";
        let expected = ["การ", "พัฒนา", "ซอฟต์แวร์"];

        check_word(s, &expected, segmenter);

        let s = "มหาวิทยาลัยเชียงใหม่";
        let expected = ["มหาวิทยาลัย", "เชียงใหม่"];

        check_word(s, &expected, segmenter);

        let s = "จังหวัดนครราชสีมา";
        let expected = ["จังหวัด", "นครราชสีมา"];

        check_word(s, &expected, segmenter);

        let s = "ระบบขนส่งมวลชน";
        let expected = ["ระบบ", "ขนส่ง", "มวลชน"];

        check_word(s, &expected, segmenter);

        let s = "บริการทางการเงิน";
        let expected = ["บริการ", "ทางการ", "เงิน"];

        check_word(s, &expected, segmenter);

        let s = "ต้มยำกุ้ง";
        let expected = ["ต้มยำ", "กุ้ง"];

        check_word(s, &expected, segmenter);

        let s = "ส้มตำไทย";
        let expected = ["ส้มตำ", "ไทย"];

        check_word(s, &expected, segmenter);

        let s = "ข้าวเหนียวมะม่วง";
        let expected = ["ข้าว", "เหนียว", "มะม่วง"];

        check_word(s, &expected, segmenter);

        let s = "ภาษาไทย ไม่มีการเว้นวรรคทุกคำ";
        let expected = ["ภาษา", "ไทย", " ", "ไม่มี", "การ", "เว้น", "วรรค", "ทุก", "คำ"];

        check_word(s, &expected, segmenter);

        let s = "กรุงเทพฯ เป็นเมืองหลวงของประเทศไทย";
        let expected = ["กรุงเทพฯ", " ", "เป็น", "เมือง", "หลวง", "ของ", "ประเทศไทย"];

        check_word(s, &expected, segmenter);
    }
}

#[test]
fn word_break_my() {
    for segmenter in [
        WordSegmenter::new_auto(WordBreakInvariantOptions::default()),
        {
            let mut s = WordSegmenter::new_neo_for_non_complex_scripts(
                WordBreakInvariantOptions::default(),
            );
            s.load_auto();
            s
        },
    ] {
        let s = "မြန်မာစာမြန်မာစာမြန်မာစာ";
        let expected = ["မြန်မာစာ", "မြန်မာစာ", "မြန်မာ", "စာ"];
        check_word(s, &expected, segmenter);
    }
}

#[test]
fn word_break_hiragana() {
    for segmenter in [
        WordSegmenter::new_auto(WordBreakInvariantOptions::default()),
        WordSegmenter::new_dictionary(WordBreakInvariantOptions::default()),
        {
            let mut s = WordSegmenter::new_neo_for_non_complex_scripts(
                WordBreakInvariantOptions::default(),
            );
            s.load_auto();
            s
        },
        {
            let mut s = WordSegmenter::new_neo_for_non_complex_scripts(
                WordBreakInvariantOptions::default(),
            );
            s.load_dictionary();
            s
        },
    ] {
        let s = "うなぎうなじ";
        let expected = ["うなぎ", "うなじ"];
        check_word(s, &expected, segmenter);
    }
}

#[test]
fn word_break_mixed_han() {
    for segmenter in [
        WordSegmenter::new_auto(WordBreakInvariantOptions::default()),
        WordSegmenter::new_dictionary(WordBreakInvariantOptions::default()),
        {
            let mut s = WordSegmenter::new_neo_for_non_complex_scripts(
                WordBreakInvariantOptions::default(),
            );
            s.load_auto();
            s
        },
        {
            let mut s = WordSegmenter::new_neo_for_non_complex_scripts(
                WordBreakInvariantOptions::default(),
            );
            s.load_dictionary();
            s
        },
    ] {
        let s = "Welcome龟山岛龟山岛Welcome";
        let expected = ["Welcome", "龟山岛", "龟山岛", "Welcome"];
        check_word(s, &expected, segmenter);
    }
}

#[test]
fn word_line_th_wikipedia_auto() {
    use icu_segmenter::LineSegmenter;

    let text = "แพนด้าแดง (อังกฤษ: Red panda, Shining cat; จีน: 小熊貓; พินอิน: Xiǎo xióngmāo) สัตว์เลี้ยงลูกด้วยนมชนิดหนึ่ง มีชื่อวิทยาศาสตร์ว่า Ailurus fulgens";

    for segmenter in [WordSegmenter::new_auto(Default::default()), {
        let mut s =
            WordSegmenter::new_neo_for_non_complex_scripts(WordBreakInvariantOptions::default());
        s.load_auto();
        s
    }] {
        check_word(
            text,
            &[
                "แพน",
                "ด้า",
                "แดง",
                " ",
                "(",
                "อัง",
                "กฤษ",
                ":",
                " ",
                "Red",
                " ",
                "panda",
                ",",
                " ",
                "Shining",
                " ",
                "cat",
                ";",
                " ",
                "จีน",
                ":",
                " ",
                "小熊",
                "貓",
                ";",
                " ",
                "พิน",
                "อิน",
                ":",
                " ",
                "Xiǎo",
                " ",
                "xióngmāo",
                ")",
                " ",
                "สัตว์",
                "เลี้ยง",
                "ลูก",
                "ด้วย",
                "นม",
                "ชนิด",
                "หนึ่ง",
                " ",
                "มี",
                "ชื่อ",
                "วิทยาศาสตร์",
                "ว่า",
                " ",
                "Ailurus",
                " ",
                "fulgens",
            ],
            segmenter,
        );
    }

    check_line(
        text,
        &[
            "แพน",
            "ด้า",
            "แดง",
            " ",
            "(อัง",
            "กฤษ",
            ": ",
            "Red ",
            "panda, ",
            "Shining ",
            "cat; ",
            "จีน",
            ": ",
            "小",
            "熊",
            "貓; ",
            "พิน",
            "อิน",
            ": ",
            "Xiǎo ",
            "xióngmāo) ",
            "สัตว์",
            "เลี้ยง",
            "ลูก",
            "ด้วย",
            "นม",
            "ชนิด",
            "หนึ่ง",
            " ",
            "มี",
            "ชื่อ",
            "วิทยาศาสตร์",
            "ว่า",
            " ",
            "Ailurus ",
            "fulgens",
        ],
        LineSegmenter::new_auto(Default::default()),
    );

    check_line(
        text,
        &[
            "แพน",
            "ด้า",
            "แดง",
            " ",
            "(อัง",
            "กฤษ",
            ": ",
            "Red ",
            "panda, ",
            "Shining ",
            "cat; ",
            "จีน",
            ": ",
            "小",
            "熊",
            "貓; ",
            "พิน",
            "อิน",
            ": ",
            "Xiǎo ",
            "xióngmāo) ",
            "สัตว์",
            "เลี้ยง",
            "ลูก",
            "ด้วย",
            "นม",
            "ชนิด",
            "หนึ่ง",
            " ",
            "มี",
            "ชื่อ",
            "วิทยาศาสตร์",
            "ว่า",
            " ",
            "Ailurus ",
            "fulgens",
        ],
        {
            let mut s = LineSegmenter::new_17_for_non_complex_scripts(Default::default());
            s.load_lstm();
            s
        },
    );

    check_line(
        text,
        &[
            "แพน",
            "ด้า",
            "แดง ",
            "(อัง",
            "กฤษ: ",
            "Red ",
            "panda, ",
            "Shining ",
            "cat; ",
            "จีน: ",
            "小",
            "熊",
            "貓; ",
            "พิน",
            "อิน: ",
            "Xiǎo ",
            "xióngmāo) ",
            "สัตว์",
            "เลี้ยง",
            "ลูก",
            "ด้วย",
            "นม",
            "ชนิด",
            "หนึ่ง ",
            "มี",
            "ชื่อ",
            "วิทยาศาสตร์",
            "ว่า ",
            "Ailurus ",
            "fulgens",
        ],
        {
            let mut s = LineSegmenter::new_neo_for_non_complex_scripts(Default::default());
            s.load_lstm();
            s
        },
    );
}
