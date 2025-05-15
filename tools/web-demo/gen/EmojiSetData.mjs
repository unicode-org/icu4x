import { EmojiSetData } from "icu4x"
export function containsStr(s) {
    
    let self = EmojiSetData.createBasic();
    
    let out = self.containsStr(s);
    
    out = out ? 'true' : 'false';;
    

    return out;
}
export function contains(cp) {
    
    let self = EmojiSetData.createBasic();
    
    let out = self.contains(cp);
    
    out = out ? 'true' : 'false';;
    

    return out;
}
