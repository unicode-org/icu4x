import { ICU4XDataProvider, ICU4XDateLength, ICU4XGregorianDateTime, ICU4XGregorianDateTimeFormatter, ICU4XLocale, ICU4XTimeLength } from "icu4x";
import { Ok, Result, result, unwrap } from ".";

class DateTimeDemo {
    #formattedDateTime: HTMLParagraphElement;
    #dataProvider: ICU4XDataProvider;

    #locale: Result<ICU4XLocale>;
    #dateLength: ICU4XDateLength;
    #timeLength: ICU4XTimeLength;

    #formatter: Result<ICU4XGregorianDateTimeFormatter>;
    #dateTime: Result<ICU4XGregorianDateTime> | null;

    constructor(formattedDateTime: HTMLParagraphElement, dataProvider: ICU4XDataProvider) {
        this.#formattedDateTime = formattedDateTime;
        this.#dataProvider = dataProvider;

        this.#locale = Ok(ICU4XLocale.create_en());
        this.#dateLength = ICU4XDateLength.Short;
        this.#timeLength = ICU4XTimeLength.Short;
        this.#dateTime = null;
        this.#updateFormatter();
    }

    setLocale(locid: string): void {
        this.#locale = result(() => ICU4XLocale.create(locid));
        this.#updateFormatter()
    }

    setDateLength(dateLength: string): void {
        this.#dateLength = ICU4XDateLength[dateLength];
        this.#updateFormatter()
    }

    setTimeLength(timeLength: string): void {
        this.#timeLength = ICU4XTimeLength[timeLength];
        this.#updateFormatter()
    }

    setDateTime(dateTime: string): void {
        const date = new Date(dateTime);

        this.#dateTime = result(() => ICU4XGregorianDateTime.try_new(
            date.getFullYear(),
            date.getMonth() + 1,
            date.getDate(),
            date.getHours(),
            date.getMinutes(),
            date.getSeconds(),
        ));
        this.#render()
    }

    #updateFormatter(): void {
        this.#formatter = result(() => ICU4XGregorianDateTimeFormatter.try_new(
            this.#dataProvider,
            unwrap(this.#locale),
            this.#dateLength,
            this.#timeLength,
        ));
        this.#render();
    }

    #render(): void {
        try {
            const formatter = unwrap(this.#formatter);
            if (this.#dateTime !== null) {
                const dateTime = unwrap(this.#dateTime);
                this.#formattedDateTime.innerHTML = formatter.format_datetime(dateTime);
            } else {
                this.#formattedDateTime.innerHTML = "";
            }
        } catch (e) {
            this.#formattedDateTime.innerHTML = `Error: ${e.error_value}`;
        }
    }
}

export function setup(dataProvider: ICU4XDataProvider): void {
    const formattedDateTime = document.getElementById('dtf-formatted') as HTMLInputElement;
    const dateTimeDemo = new DateTimeDemo(formattedDateTime, dataProvider);

    const otherLocaleBtn = document.getElementById('dtf-locale-other') as HTMLInputElement | null;
    otherLocaleBtn?.addEventListener('click', () => {
        dateTimeDemo.setLocale(otherLocaleInput.value);
    });

    const otherLocaleInput = document.getElementById('dtf-locale-other-input') as HTMLInputElement | null;
    otherLocaleInput?.addEventListener('input', () => {
        const otherLocaleBtn = document.getElementById('dtf-locale-other') as HTMLInputElement | null;
        if (otherLocaleBtn?.checked) {
            dateTimeDemo.setLocale(otherLocaleInput.value);
        }
    });

    for (let btn of document.querySelectorAll<HTMLInputElement | null>('input[name="dtf-locale"]')) {
        if (btn?.value !== 'other') {
            btn.addEventListener('click', () => dateTimeDemo.setLocale(btn.value));
        }
    }

    for (let btn of document.querySelectorAll<HTMLInputElement | null>('input[name="dtf-date-length"]')) {
        btn?.addEventListener('click', () => dateTimeDemo.setDateLength(btn.value));
    }

    for (let btn of document.querySelectorAll<HTMLInputElement | null>('input[name="dtf-time-length"]')) {
        btn?.addEventListener('click', () => dateTimeDemo.setTimeLength(btn.value));
    }

    const inputDateTime = document.getElementById('dtf-input') as HTMLInputElement | null;
    inputDateTime?.addEventListener('input', () => dateTimeDemo.setDateTime(inputDateTime.value));
}