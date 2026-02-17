// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import { getICU } from "./utils/loader.mjs";
import { demos } from "./demos/registry.mjs";

let currentNavId = 0;
let currentDemo = null;

export async function initRouter(appContainer, navContainer) {
    renderNav(navContainer);
    window.addEventListener("hashchange", () => navigate(appContainer));
    await navigate(appContainer);
}

function renderNav(container) {
    let nav = document.createElement("ul");
    nav.className = "nav nav-tabs mb-3";

    for (let demo of demos) {
        let li = document.createElement("li");
        li.className = "nav-item";

        let a = document.createElement("a");
        a.className = "nav-link";
        a.href = `#${demo.slug}`;
        a.textContent = demo.label;
        a.dataset.slug = demo.slug;

        li.appendChild(a);
        nav.appendChild(li);
    }

    container.appendChild(nav);
    updateActiveNav();
}

function updateActiveNav() {
    let slug = location.hash.slice(1) || demos[0].slug;
    document.querySelectorAll(".nav-link").forEach(link => {
        link.classList.toggle("active", link.dataset.slug === slug);
    });
}

async function navigate(container) {
    let navId = ++currentNavId;
    updateActiveNav();

    if (currentDemo?.teardown) {
        try { currentDemo.teardown(); } catch (e) { console.error(e); }
    }
    currentDemo = null;
    container.innerHTML = '<div class="text-center p-5">Loading\u2026</div>';

    let slug = location.hash.slice(1);
    if (!slug) {
        slug = demos[0].slug;
        history.replaceState(null, "", `#${slug}`);
    }

    let demoConfig = demos.find(d => d.slug === slug);
    if (!demoConfig) {
        container.innerHTML = `<p>Unknown demo: <code>${slug}</code></p>`;
        return;
    }

    try {
        let [icu, mod] = await Promise.all([getICU(), demoConfig.load()]);

        if (navId !== currentNavId) return; // stale

        currentDemo = mod;
        container.innerHTML = "";
        mod.setup(container, icu);
    } catch (e) {
        if (navId === currentNavId) {
            console.error(e);
            container.innerHTML = `<p class="error">Failed to load demo: ${e.message}</p>`;
        }
    }
}
