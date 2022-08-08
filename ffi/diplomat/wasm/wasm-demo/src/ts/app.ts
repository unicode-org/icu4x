import { ICU4XDataProvider } from 'icu4x';
import * as fdf from './fixed-decimal.js';
import * as dtf from './date-time.js';
import * as seg from './segmenter.js';

const dataProvider = ICU4XDataProvider.create_test();

fdf.setup(dataProvider);
dtf.setup(dataProvider);
seg.setup(dataProvider);
