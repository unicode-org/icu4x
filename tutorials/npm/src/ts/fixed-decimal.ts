import { DataProvider, FixedDecimal, FixedDecimalFormatter, FixedDecimalGroupingStrategy, Locale } from "icu4x";
import { Result, Ok, result, unwrap } from './index';

export class FixedDecimalDemo {
    #displayFn: (formatted: string) => void;
    #dataProvider: DataProvider;

    #locale: Result<Locale>;
    #groupingStrategy: FixedDecimalGroupingStrategy;
    #formatter: Result<FixedDecimalFormatter>;
    #fixedDecimal: Result<FixedDecimal> | null;

    constructor(displayFn: (formatted: string) => void, dataProvider: DataProvider) {
        this.#displayFn = displayFn;
        this.#dataProvider = dataProvider;

        this.#locale = Ok(Locale.create_from_string("en"));
        this.#groupingStrategy = FixedDecimalGroupingStrategy.Auto;
        this.#fixedDecimal = null;
        this.#updateFormatter()
    }

    setLocale(locid: string): void {
        this.#locale = result(() => Locale.create_from_string(locid));
        this.#updateFormatter()
    }

    setGroupingStrategy(strategy: string): void {
        this.#groupingStrategy = FixedDecimalGroupingStrategy[strategy];
        this.#updateFormatter()
    }

    setFixedDecimal(digits: string): void {
        this.#fixedDecimal = digits === "" ? null : result(() => FixedDecimal.create_from_string(digits));
        this.#render();
    }

    #updateFormatter(): void {
        this.#formatter = result(() => FixedDecimalFormatter.create_with_grouping_strategy(
            this.#dataProvider,
            unwrap(this.#locale),
            this.#groupingStrategy,
        ));
        this.#render();
    }

    #render(): void {
        try {
            const formatter = unwrap(this.#formatter);
            if (this.#fixedDecimal !== null) {
                const fixedDecimal = unwrap(this.#fixedDecimal);
                this.#displayFn(formatter.format(fixedDecimal));
            } else {
                this.#displayFn("");
            }
        } catch (e) {
            if (e.error_value) {
                this.#displayFn(` Error: ${e.error_value}`);
            } else {
                this.#displayFn(`Unexpected Error: ${e}`);
            }
        }
    }
}

export function setup(dataProvider: DataProvider): void {
    const formattedDecimal = document.getElementById('fdf-formatted') as HTMLParagraphElement;
    const fixedDecimalDemo = new FixedDecimalDemo((formatted) => {
        formattedDecimal.innerText = formatted;
    }, dataProvider);

    const otherLocaleBtn = document.getElementById('fdf-locale-other') as HTMLInputElement | null;
    otherLocaleBtn?.addEventListener('click', () => fixedDecimalDemo.setLocale(otherLocaleInput.value));

    const otherLocaleInput = document.getElementById('fdf-locale-other-input') as HTMLInputElement | null;
    otherLocaleInput?.addEventListener('input', () => {
        if (otherLocaleBtn != null) {
            otherLocaleBtn.checked = true;
            fixedDecimalDemo.setLocale(otherLocaleInput.value);
        }
    });

    for (let btn of document.querySelectorAll<HTMLInputElement | null>('input[name="fdf-locale"]')) {
        if (btn?.value !== 'other') {
            btn.addEventListener('click', () => fixedDecimalDemo.setLocale(btn.value));
        }
    }

    for (let btn of document.querySelectorAll<HTMLInputElement | null>('input[name="fdf-grouping"]')) {
        btn?.addEventListener('click', () => fixedDecimalDemo.setGroupingStrategy(btn.value));
    }

    const inputDecimal = document.getElementById('fdf-input') as HTMLTextAreaElement | null;
    inputDecimal?.addEventListener('input', () => fixedDecimalDemo.setFixedDecimal(inputDecimal.value));
}
