import { CanonicalComposition } from "icu4x"
export function compose(starter, second) {
    
    let self = new CanonicalComposition();
    
    let out = self.compose(starter,second);
    
    out = String.fromCharCode(out);;
    

    return out;
}
