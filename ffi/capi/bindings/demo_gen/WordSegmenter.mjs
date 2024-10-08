import { lib } from "./index.mjs";

export default {
	"WordSegmenter.segment": {
		func: (model, text) => {
			let provider = lib.DataProvider.compiled();

			var segmenter;
			switch (model) {
				case "Auto":
					segmenter = lib.WordSegmenter.createAuto(provider);
					break;
				case "LSTM":
					segmenter = lib.WordSegmenter.createLSTM(provider);
					break;
				case "Dictionary":
					segmenter = lib.WordSegmenter.createDictionary(provider);
			}

			const iter = segmenter.segment(text);
			const segments = [];
			
			let utf8Index = 0;
			let utf16Index = 0;
			while (true) {
				const next = iter.next();

				if (next === -1) {
					segments.push(text.slice(utf16Index));
					break;
				} else {
					const oldUtf16Index = utf16Index;
					while (utf8Index < next) {
						const codePoint = text.codePointAt(utf16Index);
						const utf8Len = (codePoint <= 0x7F) ? 1
							: (codePoint <= 0x7FF) ? 2
							: (codePoint <= 0xFFFF) ? 3
							: 4;
						const utf16Len = (codePoint <= 0xFFFF) ? 1
							: 2;
						utf8Index += utf8Len;
						utf16Index += utf16Len;
					}
					segments.push(text.slice(oldUtf16Index, utf16Index));
				}
			}
			
			return segments.join(" . ");
		},
		funcName: "WordSegmenter.segment",
		parameters: [
			{
				name: "Model Type (Auto, LSTM, or Dictionary)",
				type: "string",
				defaultValue: "Auto"
			},
			{
				name: "Text",
				type: "string",
				defaultValue: "โดยที่การยอมรับนับถือเกียรติศักดิ์ประจำตัว และสิทธิเท่าเทียมกันและโอนมิได้ของบรรดา สมาชิก ทั้ง หลายแห่งครอบครัว มนุษย์เป็นหลักมูลเหตุแห่งอิสรภาพ ความยุติธรรม และสันติภาพในโลก"
			}
		]
	}
};