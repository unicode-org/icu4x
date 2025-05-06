import { RenderInfo, lib } from "icu4x/demo";
import { TerminusRender } from "icu4x/demo/rendering";

let params = new URLSearchParams(window.location.search);

let func = params.get("func");

let terminus = new TerminusRender(lib, () => {}, RenderInfo.termini[func]);

document.getElementById("render").appendChild(terminus);