import { ICU4XFixedDecimal, ICU4XLocale, ICU4XDataProvider, ICU4XFixedDecimalFormatter, ICU4XFixedDecimalGroupingStrategy } from "icu4x";

const locale = ICU4XLocale.create_bn();

const data_provider = ICU4XDataProvider.create_test();

const fdf = ICU4XFixedDecimalFormatter.try_new(data_provider, locale, ICU4XFixedDecimalGroupingStrategy.Auto);

export function format(n: number): string {
    if (n > 2147483647 || n < -2147483648 || n % 1 !== 0) {
        throw Error(`Not an i32: ${n}`);
    }
    return fdf.format(ICU4XFixedDecimal.create(n));
}
