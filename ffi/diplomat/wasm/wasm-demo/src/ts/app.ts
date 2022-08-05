import { ICU4XDataProvider } from 'icu4x';
import * as fdf from './fixed-decimal';
import * as dtf from './date-time';
import * as seg from './segmenter';

import '../scss/styles.scss';
import 'bootstrap/js/dist/tab';
import 'bootstrap/js/dist/dropdown';
import 'bootstrap/js/dist/collapse';

const dataProvider = ICU4XDataProvider.create_test();

fdf.setup(dataProvider);
dtf.setup(dataProvider);
seg.setup(dataProvider);

