import { SegmenterWordType } from "icu4x"
export function isWordLike(self) {
    
    let out = new SegmenterWordType(self).isWordLike;
    
    out = out ? 'true' : 'false';;
    

    return out;
}
