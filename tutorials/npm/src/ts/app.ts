import * as fdf from './fixed-decimal';
import * as dtf from './date-time';
import * as seg from './segmenter';

import 'bootstrap/js/dist/tab';
import 'bootstrap/js/dist/dropdown';
import 'bootstrap/js/dist/collapse';

(async function init() {
    fdf.setup();
    dtf.setup();
    seg.setup();
    (document.querySelector("#bigspinner") as HTMLElement).style.display = "none";
})()