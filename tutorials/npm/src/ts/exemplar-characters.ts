import { Locale, ExemplarCharacters } from "icu4x";

type Kind = typeof KINDS[number];
const KINDS = ['Main', 'Auxiliary', 'Punctuation', 'Numbers', 'Index'] as const;

type ExemplarCharactersData = Partial<Record<Kind, string[] | Error>>;

export class ExemplarCharactersWrapper {
	#exemplarCharacters: ExemplarCharacters;
	#collator: Intl.Collator;

	constructor(locid: string, kind: Kind) {
		this.#exemplarCharacters = ExemplarCharacters[`create${kind}`](Locale.fromString(locid));
		this.#collator = new Intl.Collator(locid);
	}

	*#chars(): Generator<string> {
		const codePointRanges = this.#exemplarCharacters.codePointRanges();

		while (true) {
			const { start, end, done } = codePointRanges.next();
			if (done) return;
			for (let cp = start; cp <= end; ++cp) yield String.fromCodePoint(cp);
		}
	}

	*#strings(): Generator<string> {
		const codePointRanges = this.#exemplarCharacters.strings();
		let str: string;
		while ((str = codePointRanges.next())) yield str.slice(0, -1);
	}

	alphabet(): string[] {
		return [...this.#chars(), ...this.#strings()].sort(this.#collator.compare);
	}
}

export class ExemplarCharactersDemo {
	#displayFn: (x: ExemplarCharactersData) => void;

	constructor(displayFn: (x: ExemplarCharactersData) => void) {
		this.#displayFn = displayFn;
		this.#render('en');
	}

	setLocale(locid: string): void {
		this.#render(locid);
	}

	#render(locid: string): void {
		try {
			const charsDisplay = Object.fromEntries(KINDS.map((k) => [
				k,
				new ExemplarCharactersWrapper(locid, k).alphabet(),
			]));
			this.#displayFn(charsDisplay);
		} catch (e: any) {
			if (e.error_value) {
				this.#displayFn({ Main: new Error(`Error: ${e.error_value}`) });
			} else {
				this.#displayFn({ Main: new Error(`Unexpected Error: ${e}`) });
			}
		}
	}
}

export function setup(): void {
	const exemplarCharactersDemo = new ExemplarCharactersDemo((x) => {
		for (const k of KINDS) {
			const val = x[k]
			const el = document.getElementById(`ech-output-${k.toLowerCase()}`)!

			if (val instanceof Error) {
				el.textContent = val.message;
			} else {
				el.textContent = '';
				for (const text of val ?? []) {
					el.append(
						Object.assign(document.createElement('span'), { textContent: text, className: 'badge bg-secondary me-1' }),
					);
				}
			}
		}
	});

	const otherLocaleBtn = document.getElementById('ech-locale-other') as HTMLInputElement;
	otherLocaleBtn.addEventListener('click', () => exemplarCharactersDemo.setLocale(otherLocaleInput.value));

	const otherLocaleInput = document.getElementById('ech-locale-other-input') as HTMLInputElement;
	otherLocaleInput.addEventListener('input', () => {
		otherLocaleBtn.checked = true;
		exemplarCharactersDemo.setLocale(otherLocaleInput.value);
	});

	for (const btn of document.querySelectorAll<HTMLInputElement>('input[name="ech-locale"]')) {
		if (btn.value !== 'other') {
			btn.addEventListener('click', () => exemplarCharactersDemo.setLocale(btn.value));
		}
	}
}
