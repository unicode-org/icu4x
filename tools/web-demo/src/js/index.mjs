// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import { RenderInfo, icu } from "../../gen/index.mjs";
import { TerminusRender } from "../../gen/rendering/rendering.mjs";
import beautify from 'js-beautify';
import Prism from 'prismjs';

window.icu = window.top.icu = icu;

// Renders all termini into the class="container" element
Object.values(RenderInfo.termini).toSorted((a, b) => a.funcName < b.funcName ? -1 : 1).forEach((t) => {
	let details = document.createElement("details");
	let summary = document.createElement("summary");
	summary.innerHTML = `<code>${t.funcName}</code>`;
	details.appendChild(summary);
	details.appendChild(document.createElement("br"));
	details.appendChild(new TerminusRender(
		RenderInfo.termini[t.funcName],
		(el) => {
			// Necessary for Prism to know the language to highlight for, and also
			// to ensure CSS `white-space: pre-wrap` is applied from selector
			el.classList.add("language-js");
			el.textContent = beautify.js(el.textContent, {
				indent_size: 2,
				indent_char: " ",
				break_chained_methods: true,
				// brace_style: "collapse",
				wrap_line_length: 45,
			});
			Prism.highlightElement(el);
		},
	));
	document.getElementsByClassName("container")[0].appendChild(details);
});

document.querySelector("#loading").hidden = true;
document.querySelector("#loaded").hidden = false;
