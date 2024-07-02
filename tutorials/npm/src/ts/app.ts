import { DataProviderManager } from './data-provider-manager';
import * as fdf from './fixed-decimal';
import * as dtf from './date-time';
import * as seg from './segmenter';

import 'bootstrap/js/dist/tab';
import 'bootstrap/js/dist/dropdown';
import 'bootstrap/js/dist/collapse';

(async function init() {
    const dataManager = await DataProviderManager.create();

    fdf.setup(dataManager);
    dtf.setup(dataManager);
    seg.setup(dataManager);

    (document.querySelector("#bigspinner") as HTMLElement).style.display = "none";
})()