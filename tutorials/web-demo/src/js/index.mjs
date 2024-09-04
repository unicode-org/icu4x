import { RenderInfo } from "icu4x/demo";

Object.values(RenderInfo.termini).forEach((t) => {
	let a = document.createElement("a");
	a.classList.add("list-group-item", "list-group-item-action");
	a.innerHTML = `<a href="template.html?func=${t.funcName}">${t.funcName}</a>`;
	document.getElementById("links").appendChild(a);
});