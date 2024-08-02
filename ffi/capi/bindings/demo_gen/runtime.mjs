import { RenderInfo, lib } from "./index.mjs";
import { initialize } from "./rendering.mjs";

let templates = document.getElementById("templates").contentDocument;
initialize(templates, lib);

let termini = document.querySelectorAll("terminus-render");

termini.forEach((t) => {
    t.attachTerminus(RenderInfo.termini[t.getAttribute("func")]);
});