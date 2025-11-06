// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import { icu } from "./index.mjs";

export default {
	"WordSegmenter.segment": {
		func: (model, text) => {
			var segmenter;
			switch (model) {
				case "Auto":
					segmenter = icu.WordSegmenter.createAuto();
					break;
				case "LSTM":
					segmenter = icu.WordSegmenter.createLstm();
					break;
				case "Dictionary":
					segmenter = icu.WordSegmenter.createDictionary();
			}
			
			let last = 0;
			const iter = segmenter.segment(text);

			const segments = [];
			
			while (true) {
				const next = iter.next();

				if (next === -1) {
					segments.push(text.slice(last));
					break;
				}

				segments.push(text.slice(last, next));
				last = next;
			}
			
			return segments.join(" . ");
		},
		expr: (model, text) => {
			switch (model) {
				case "'Auto'":
					return 'icu.WordSegmenter.createAuto().segment(text)';
				case "'LSTM'":
					return 'icu.WordSegmenter.createLstm().segment(text)';
				case "'Dictionary'":
					return 'icu.WordSegmenter.createDictionary().segment(text)';
			}
			return 'icu.WordSegmenter.type().segment(text)';
		},
		funcName: "WordSegmenter.segment",
		parameters: [
			{
				name: "type",
				typeUse: "enumerator",
				values: ["Auto", "LSTM", "Dictionary"],
			},
			{
				name: "text",
				type: "string",
				typeUse: "string",
				defaultValue: "โดยที่การยอมรับนับถือเกียรติศักดิ์ประจำตัว และสิทธิเท่าเทียมกันและโอนมิได้ของบรรดา สมาชิก ทั้ง หลายแห่งครอบครัว มนุษย์เป็นหลักมูลเหตุแห่งอิสรภาพ ความยุติธรรม และสันติภาพในโลก"
			}
		]
	}
};
