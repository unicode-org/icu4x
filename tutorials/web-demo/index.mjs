import { RenderInfo } from "./dist/index.mjs";

Object.values(RenderInfo.termini).forEach((t) => {
	let a = document.createElement("li");
	a.innerHTML = `<a href="dist/${t.html}">${t.funcName}</a>`;
	document.getElementById("links").appendChild(a);
});