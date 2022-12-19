import { ICU4XDataProvider, fullData } from 'icu4x';
import * as fdf from './fixed-decimal';
import * as dtf from './date-time';
import * as seg from './segmenter';

import 'bootstrap/js/dist/tab';
import 'bootstrap/js/dist/dropdown';
import 'bootstrap/js/dist/collapse';

(async function init() {
    const dataProvider = ICU4XDataProvider.create_from_byte_slice(await fullData());
    dataProvider.enable_locale_fallback();

    fdf.setup(dataProvider);
    dtf.setup(dataProvider);
    seg.setup(dataProvider);
    (document.querySelector("#bigspinner") as HTMLElement).style.display = "none";
})()