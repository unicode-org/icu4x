import { ICU4XDataProvider, ICU4XWordSegmenter } from "icu4x";

export class SegmenterDemo {
    #displayFn: (formatted: string) => void;

    #segmenter: ICU4XWordSegmenter;
    #text: string;

    constructor(displayFn: (formatted: string) => void, dataProvider: ICU4XDataProvider) {
        this.#displayFn = displayFn;
        this.#segmenter = ICU4XWordSegmenter.create_auto(dataProvider);
    }

    setText(text: string): void {
        this.#text = text;
        this.#render();
    }

    #render(): void {
        const segments = [];

        const sizeOfLetter = (codePoint: number) => {
            let numBytes = 0;
            while (codePoint > 0) {
                codePoint >>= 4;
                numBytes += 1;
            }
            return numBytes - 1;
        };

        let nextIndex = 0;
        let bytes = 0;
        const updateIndexToByte = (byte: number) => {
            if (bytes >= byte) {
                return nextIndex;
            }
            for (let n = 0; n < 8; ++n) {
                bytes += sizeOfLetter(this.#text.codePointAt(nextIndex));
                nextIndex += 1;
                if (bytes >= byte) {
                    return nextIndex;
                }
            }
            return -1;
        };

        const iter8 = this.#segmenter.segment_utf8(this.#text);
        let index = 0;
        while (true) {
            const next = iter8.next();

            if (next === -1) {
                segments.push(this.#text.slice(index));
                break;
            } else {
                const nextIndex = updateIndexToByte(next);
                if (nextIndex === -1) {
                    this.#displayFn("Error: characters currently not support in the JS demo");
                    return;
                }
                segments.push(this.#text.slice(index, nextIndex));
                index = nextIndex;
            }
        }

        this.#displayFn(segments.join('<span class="seg-delim"> . </span>'));
    }
}

export function setup(dataProvider: ICU4XDataProvider): void {
    const segmentedText = document.getElementById('seg-segmented') as HTMLParagraphElement;
    const segmenterDemo = new SegmenterDemo((formatted) => {
        // Use innerHTML because we have actual HTML we want to display
        segmentedText.innerHTML = formatted;
    }, dataProvider);

    const inputText = document.getElementById('seg-input') as HTMLTextAreaElement | null;
    inputText?.addEventListener('input', () => segmenterDemo.setText(inputText.value));

    const japaneseSamples = [
        "全部の人間は、生まれながらにして自由であり、かつ、尊厳と権利と について平等である。人間は、理性と良心とを授けられており、互いに同 胞の精神をもって行動しなければならない。",
        "全部の人は、人種、皮膚の色、性、言語、宗教、政治上その他の意見、国民的もしくは社会的出身、財産、門地その他の地位又はこれに類するいかなる自由による差別をも受けることなく、この宣言に掲げるすべての権利と自由とを享有することができる。",
        "さらに、個人の属する国又は地域が独立国であると、信託統治地域であると、非自治地域であると、又は他のなんらかの主権制限の下にあるとを問わず、その国又は地域の政治上、管轄上又は国際上の地位に基ずくいかなる差別もしてはならない。",
        "全部の人は、生命、自由と身体の安全に対する権利がある。",
        "何人も、奴隷にされ、又は苦役に服することはない。奴隷制度と奴隷売買は、いかなる形においても禁止する。",
    ];

    const sampleJapaneseBtn = document.getElementById('seg-sample-japanese') as HTMLButtonElement | null;
    sampleJapaneseBtn?.addEventListener('click', () => {
        inputText.value = japaneseSamples[Math.floor(Math.random() * japaneseSamples.length)];
        segmenterDemo.setText(inputText.value);
    });

    const chineseSamples = [
        "人人生而自由，在尊严合权利上一律平等。因赋有脾胃合道行，并着以兄弟关系的精神相对待。",
        "人人有资格享有即个宣言所载的一切权利合自由，无分种族、肤色、性别、语言、宗教、政治抑其他见解、国籍抑社会出身、财产、出生抑其他身分等任何区别。",
        "并且勿会用因一人所属的国家抑地的政治的、行政的抑国际的地位之不同而有所区别，无论即个地是独立地、托管地、非自治地抑处于其他任何主权受限制的代志之下。",
        "人人过过有权享有生命、自由合人身安全。",
        "任何人勿会用互为奴隶抑奴役；一切形式的奴隶制度合奴隶买卖，过过着予以禁止。",
    ];
    
    const sampleChineseBtn = document.getElementById('seg-sample-chinese') as HTMLButtonElement | null;
    sampleChineseBtn?.addEventListener('click', () => {
        inputText.value = chineseSamples[Math.floor(Math.random() * chineseSamples.length)];
        segmenterDemo.setText(inputText.value);
    });
}
