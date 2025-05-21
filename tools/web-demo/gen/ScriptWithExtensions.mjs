import { ScriptWithExtensions } from "icu4x"
export function getScriptVal(ch) {
    
    let self = new ScriptWithExtensions();
    
    let out = self.getScriptVal(ch);
    

    return out;
}
export function hasScript(ch, script) {
    
    let self = new ScriptWithExtensions();
    
    let out = self.hasScript(ch,script);
    
    out = out ? 'true' : 'false';;
    

    return out;
}
