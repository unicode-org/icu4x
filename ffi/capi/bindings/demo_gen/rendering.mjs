var lib = undefined;
var templateDoc = undefined;

export function initialize(templateDocument, library) {
    lib = library;
    templateDoc = templateDocument;
}

function generateTemplate(className, variable, selector) {
    if (className[variable] === undefined) {
        className[variable] = templateDoc.querySelector(selector).content;
    }
}

class ParameterTemplate extends HTMLElement {
    default = null;

    static baseTemplate;
    constructor(className, selector, ...args) {
        super();
        generateTemplate(ParameterTemplate, "baseTemplate", "#parameter");
        generateTemplate(className, "template", selector);
        let baseClone = ParameterTemplate.baseTemplate.cloneNode(true);

        let clone = className["template"].cloneNode(true);

        this.initialize(clone, ...args);

        let input = clone.querySelector("*[data-oninput]");
        if (input !== null) {
            input.addEventListener("input", this.input.bind(this));
        }
        
        clone.slot = "parameter";
        baseClone.appendChild(clone);

        const shadowRoot = this.attachShadow({ mode: "open" });
        shadowRoot.appendChild(baseClone);
    }

    getEventValue(event) {
        return event.target.value;
    }

    input(event) {
        this.dispatchEvent(new CustomEvent("parameter-input", {
            detail: this.getEventValue(event)
        }));
    }

    initialize(clone) {

    }
}

customElements.define("terminus-param", ParameterTemplate);

class BooleanTemplate extends ParameterTemplate {
    default = false;
    static template;
    constructor() {
        super(BooleanTemplate, "template#boolean");
    }
}

customElements.define("terminus-param-boolean", BooleanTemplate);

class NumberTemplate extends ParameterTemplate {
    default = 0;
    static template;
    constructor() {
        super(NumberTemplate, "template#number");
    }
    
    getEventValue(event) {
        return parseFloat(event.target.value);
    }
}

customElements.define("terminus-param-number", NumberTemplate);

class StringTemplate extends ParameterTemplate {
    default = "";
    static template;
    constructor() {
        super(StringTemplate, "template#string");
    }
}

customElements.define("terminus-param-string", StringTemplate);

class EnumOption extends HTMLElement {
    static template;
    constructor(optionText) {
        super();
        generateTemplate(EnumOption, "template", "template#enum-option");
        let clone = EnumOption.template.cloneNode(true);

        clone.querySelector("slot[name='option-text']").parentElement.innerText = optionText;
        
        this.append(...clone.children);
    }
}

customElements.define("terminus-enum-option", EnumOption);

class EnumTemplate extends ParameterTemplate {
    static template;

    #enumType;
    constructor(enumType) {
        super(EnumTemplate, "template#enum", enumType);
        this.#enumType = enumType;
    }

    initialize(clone, enumType) {
        let options = clone.querySelector("*[data-options]");

        this.default = enumType.values.values().next().value;

        for (let value of enumType.values) {
            options.append(...(new EnumOption(value[0])).children);
        }
    }

    getEventValue(event) {
        return this.#enumType[event.target.value];
    }
}

customElements.define("terminus-param-enum", EnumTemplate);

class TerminusParams extends HTMLElement {
    #params = [];

    constructor(params, evaluateExternal){
        super();

        for (var i = 0; i < params.length; i++) {
            let param = params[i];
            let paramName = document.createElement("span");
            paramName.slot = "param-name";
            paramName.innerText = param.name;

            var newChild;

            switch (param.type) {
                case "string":
                    newChild = new StringTemplate();
                    this.#params[i] = "";
                    break;
                case "boolean":
                    newChild = new BooleanTemplate();
                    this.#params[i] = false;
                    break;
                case "number":
                    newChild = new NumberTemplate();
                    this.#params[i] = 0;
                    break;
                default:
                    if (param.type in lib && "values" in lib[param.type]) {
                        newChild = new EnumTemplate(lib[param.type]);
                        this.#params[i] = newChild.default
                    } else {
                        let updateParamEvent = (value) => {
                            this.#params[i] = value;
                        };
                        evaluateExternal(param, updateParamEvent);
                        continue;
                    }
                    break;
            }

            newChild.addEventListener("parameter-input", this.input.bind(this, i));
            this.#params[i] = newChild.default;

            newChild.appendChild(paramName);
            this.appendChild(newChild);
        }
    }

    input(paramIdx, event) {
        this.#params[paramIdx] = event.detail;
    }

    get paramArray() {
        return this.#params;
    }
}

customElements.define("terminus-params", TerminusParams);

export class TerminusRender extends HTMLElement {
    static template;

    #func = null;
    #parameters;
    #output;
    constructor() {
        super();
    }

    attachTerminus(terminus, evaluateExternal) {
        generateTemplate(TerminusRender, "template", "template#terminus");
        let clone = TerminusRender.template.cloneNode(true);

        this.id = terminus.funcName;

        this.#func = terminus.func;

        let button = clone.querySelector("*[data-submit]");
        button.addEventListener("click", this.submit.bind(this));

        let funcText = document.createElement("span");
        funcText.slot = "func-name";
        funcText.innerText = terminus.funcName;
        this.appendChild(funcText);

        this.#parameters = new TerminusParams(terminus.parameters, evaluateExternal);
        this.#parameters.slot = "parameters";
        this.appendChild(this.#parameters);

        this.#output = document.createElement("span");
        this.#output.slot = "output";

        this.appendChild(this.#output);

        const shadowRoot = this.attachShadow({ mode: "open" });
        shadowRoot.appendChild(clone);
    }

    submit() {
        try {
            this.#output.innerText = this.#func(...this.#parameters.paramArray);
        } catch(e) {
            this.#output.innerText = e;
            throw e;
        }
    }
}

customElements.define("terminus-render", TerminusRender);