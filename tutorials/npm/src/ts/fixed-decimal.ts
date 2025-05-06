import { Decimal, DecimalFormatter, DecimalGroupingStrategy, Locale } from "icu4x";
import { Result, Ok, result, unwrap } from './index';

export class DecimalDemo {
    #displayFn: (formatted: string) => void;

    #locale: Result<Locale>;
    #groupingStrategy: DecimalGroupingStrategy;
    #formatter: Result<DecimalFormatter>;
    #fixedDecimal: Result<Decimal> | null;

    constructor(displayFn: (formatted: string) => void) {
        this.#displayFn = displayFn;

        this.#locale = Ok(Locale.fromString("en"));
        this.#groupingStrategy = DecimalGroupingStrategy.Auto;
        this.#fixedDecimal = null;
        this.#updateFormatter()
    }

    setLocale(locid: string): void {
        this.#locale = result(() => Locale.fromString(locid));
        this.#updateFormatter()
    }

    setGroupingStrategy(strategy: string): void {
        this.#groupingStrategy = DecimalGroupingStrategy[strategy];
        this.#updateFormatter()
    }

    setFixedDecimal(digits: string): void {
        this.#fixedDecimal = digits === "" ? null : result(() => Decimal.fromString(digits));
        this.#render();
    }

    #updateFormatter(): void {
        this.#formatter = result(() => DecimalFormatter.createWithGroupingStrategy(
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

export function setup(): void {
    const formattedDecimal = document.getElementById('fdf-formatted') as HTMLParagraphElement;
    const decimalDemo = new DecimalDemo((formatted) => {
        formattedDecimal.innerText = formatted;
    });

    const otherLocaleBtn = document.getElementById('fdf-locale-other') as HTMLInputElement | null;
    otherLocaleBtn?.addEventListener('click', () => decimalDemo.setLocale(otherLocaleInput.value));

    const otherLocaleInput = document.getElementById('fdf-locale-other-input') as HTMLInputElement | null;
    otherLocaleInput?.addEventListener('input', () => {
        if (otherLocaleBtn != null) {
            otherLocaleBtn.checked = true;
            decimalDemo.setLocale(otherLocaleInput.value);
        }
    });

    for (let btn of document.querySelectorAll<HTMLInputElement | null>('input[name="fdf-locale"]')) {
        if (btn?.value !== 'other') {
            btn.addEventListener('click', () => decimalDemo.setLocale(btn.value));
        }
    }

    for (let btn of document.querySelectorAll<HTMLInputElement | null>('input[name="fdf-grouping"]')) {
        btn?.addEventListener('click', () => decimalDemo.setGroupingStrategy(btn.value));
    }

    const inputDecimal = document.getElementById('fdf-input') as HTMLTextAreaElement | null;
    inputDecimal?.addEventListener('input', () => decimalDemo.setFixedDecimal(inputDecimal.value));
}
