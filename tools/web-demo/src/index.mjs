// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import { initRouter } from "./router.mjs";

let app = document.getElementById("app");
let nav = document.getElementById("nav");
initRouter(app, nav);
