import { ICU4XDataProvider, ICU4XDateLength, ICU4XDateTime, ICU4XDateTimeFormatter, ICU4XLocale, ICU4XTimeLength, ICU4XCalendar } from "icu4x";
import { Ok, Result, result, unwrap } from ".";

class DateTimeDemo {
    #formattedDateTime: HTMLParagraphElement;
    #dataProvider: ICU4XDataProvider;

    #localeStr: string;
    #calendarStr: string;
    #dateTimeStr: string;
    #locale: Result<ICU4XLocale>;
    #calendar: Result<ICU4XCalendar>;
    #dateLength: ICU4XDateLength;
    #timeLength: ICU4XTimeLength;

    #formatter: Result<ICU4XDateTimeFormatter>;
    #dateTime: Result<ICU4XDateTime> | null;

    constructor(formattedDateTime: HTMLParagraphElement, dataProvider: ICU4XDataProvider) {
        this.#formattedDateTime = formattedDateTime;
        this.#dataProvider = dataProvider;

        this.#locale = Ok(ICU4XLocale.create("en-u-ca-gregory"));
        this.#calendar = Ok(ICU4XCalendar.try_new(dataProvider, unwrap(this.#locale)));
        this.#dateLength = ICU4XDateLength.Short;
        this.#timeLength = ICU4XTimeLength.Short;
        this.#dateTime = null;
        this.#dateTimeStr = "";
        this.#calendarStr = "gregory";
        this.#localeStr = "en";
        this.#updateFormatter();
    }

    setCalendar(calendar: string): void {
        this.#calendarStr = calendar;
        this.#updateLocaleAndCalendar();
        this.#updateFormatter();
    }

    setLocale(locid: string): void {
        this.#localeStr = locid;
        this.#updateLocaleAndCalendar();
        this.#updateFormatter();
    }

    #updateLocaleAndCalendar(): void {
        const locid = (this.#calendarStr == "from-locale") ?
            this.#localeStr :
            `${this.#localeStr}-u-ca-${this.#calendarStr}`;
        this.#locale = result(() => ICU4XLocale.create(locid));
        this.#calendar = result(() => ICU4XCalendar.try_new(this.#dataProvider, unwrap(this.#locale) ));
        this.#updateDateTime();
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
        this.#dateTimeStr = dateTime;
        this.#updateDateTime();
        this.#render()
    }

    #updateDateTime(): void {
        const date = new Date(this.#dateTimeStr);

        this.#dateTime = result(() => ICU4XDateTime.try_new_from_iso_in_calendar(
            date.getFullYear(),
            date.getMonth() + 1,
            date.getDate(),
            date.getHours(),
            date.getMinutes(),
            date.getSeconds(),
            unwrap(this.#calendar)
        ));
    }

    #updateFormatter(): void {
        this.#formatter = result(() => ICU4XDateTimeFormatter.try_new(
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
    for (let btn of document.querySelectorAll<HTMLSelectElement | null>('select[name="dtf-calendar"]')) {
        btn.addEventListener('click', () => dateTimeDemo.setCalendar(btn.value));
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