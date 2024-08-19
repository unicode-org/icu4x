import { DataProvider, DateLength, DateTime, DateTimeFormatter, Locale, TimeLength, Calendar } from "icu4x";
import { Ok, Result, result, unwrap } from "./index";

export class DateTimeDemo {
    #displayFn: (formatted: string) => void;
    #dataProvider: DataProvider;

    #localeStr: string;
    #calendarStr: string;
    #dateTimeStr: string;
    #locale: Result<Locale>;
    #calendar: Result<Calendar>;
    #dateLength: DateLength;
    #timeLength: TimeLength;

    #formatter: Result<DateTimeFormatter>;
    #dateTime: Result<DateTime> | null;

    constructor(displayFn: (formatted: string) => void, dataProvider: DataProvider) {
        this.#displayFn = displayFn;
        this.#dataProvider = dataProvider;

        this.#locale = Ok(Locale.createFromString("en"));
        this.#calendar = Ok(Calendar.createForLocale(dataProvider, unwrap(this.#locale)));
        this.#dateLength = DateLength.Short;
        this.#timeLength = TimeLength.Short;
        this.#dateTime = null;
        this.#dateTimeStr = "";
        this.#calendarStr = "from-locale";
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
        let locid = this.#localeStr;
        if (this.#calendarStr != "from-locale") {
            if (locid.indexOf("-u-") == -1) {
                locid = `${locid}-u-ca-${this.#calendarStr}`;
            } else {
                // Don't bother trying to patch up the situation where a calendar
                // is already specified; this is GIGO and the current locale parsing behavior
                // will just default to the first one (#calendarStr)
                locid = locid.replace("-u-", `-u-ca-${this.#calendarStr}-`);
            }
        }
        this.#locale = result(() => Locale.createFromString(locid));
        this.#calendar = result(() => Calendar.createForLocale(this.#dataProvider, unwrap(this.#locale) ));
        this.#updateDateTime();
    }

    setDateLength(dateLength: string): void {
        this.#dateLength = DateLength[dateLength];
        this.#updateFormatter()
    }

    setTimeLength(timeLength: string): void {
        this.#timeLength = TimeLength[timeLength];
        this.#updateFormatter()
    }

    setDateTime(dateTime: string): void {
        this.#dateTimeStr = dateTime;
        this.#updateDateTime();
        this.#render()
    }

    #updateDateTime(): void {
        const date = new Date(this.#dateTimeStr);

        this.#dateTime = result(() => DateTime.createFromIsoInCalendar(
            date.getFullYear(),
            date.getMonth() + 1,
            date.getDate(),
            date.getHours(),
            date.getMinutes(),
            date.getSeconds(),
            0,
            unwrap(this.#calendar)
        ));
    }

    #updateFormatter(): void {
        this.#formatter = result(() => DateTimeFormatter.createWithLengths(
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
                this.#displayFn(formatter.format_datetime(dateTime));
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
    const formattedDateTime = document.getElementById('dtf-formatted') as HTMLInputElement;
    const dateTimeDemo = new DateTimeDemo((formatted) => formattedDateTime.innerText = formatted, dataProvider);

    const otherLocaleBtn = document.getElementById('dtf-locale-other') as HTMLInputElement | null;
    otherLocaleBtn?.addEventListener('click', () => {
        dateTimeDemo.setLocale(otherLocaleInput.value);
    });

    const otherLocaleInput = document.getElementById('dtf-locale-other-input') as HTMLInputElement | null;
    otherLocaleInput?.addEventListener('input', () => {
        const otherLocaleBtn = document.getElementById('dtf-locale-other') as HTMLInputElement | null;
        if (otherLocaleBtn != null) {
            otherLocaleBtn.checked = true;
            dateTimeDemo.setLocale(otherLocaleInput.value);
        }
    });

    for (let input of document.querySelectorAll<HTMLInputElement | null>('input[name="dtf-locale"]')) {
        if (input?.value !== 'other') {
            input.addEventListener('input', () => {
                dateTimeDemo.setLocale(input.value)
            });
        }
    }
    for (let selector of document.querySelectorAll<HTMLSelectElement | null>('select[name="dtf-calendar"]')) {
        // <select> doesn't have oninput
        selector?.addEventListener('change', () => {
            dateTimeDemo.setCalendar(selector.value)
        });
    }

    for (let input of document.querySelectorAll<HTMLInputElement | null>('input[name="dtf-date-length"]')) {
        input?.addEventListener('input', () => {
            dateTimeDemo.setDateLength(input.value)
        });
    }

    for (let input of document.querySelectorAll<HTMLInputElement | null>('input[name="dtf-time-length"]')) {
        input?.addEventListener('input', () => {
            dateTimeDemo.setTimeLength(input.value)
        });
    }

    const inputDateTime = document.getElementById('dtf-input') as HTMLInputElement | null;
    inputDateTime?.addEventListener('input', () => {
        dateTimeDemo.setDateTime(inputDateTime.value)
    });
    
    // Annoyingly `toISOString()` gets us the format we need, but it converts to UTC first
    // We instead get the current datetime and recast it to a date that is the current datetime
    // when represented in UTC
    let now = new Date();
    const offset = now.getTimezoneOffset();
    now.setMinutes(now.getMinutes() - offset);
    const nowISO = now.toISOString().slice(0,16);
    if (inputDateTime != undefined) {
        // this seems like the best way to get something compatible with inputDateTIme
        inputDateTime.value = nowISO;
    }
    dateTimeDemo.setDateTime(nowISO);
}
