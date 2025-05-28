import { GeneralCategoryGroup } from "icu4x"
export function contains(selfMask, val) {
    
    let self = GeneralCategoryGroup.fromFields({
        mask: selfMask
    });
    
    let out = self.contains(val);
    
    out = out ? 'true' : 'false';;
    

    return out;
}
