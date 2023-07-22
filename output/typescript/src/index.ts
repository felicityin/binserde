import { Uint8, Uint16 } from 'bincoder';

import { Hello } from './schemas';

let entity = new Hello();
entity.a = new Uint8(1);
entity.b = new Uint16(2);

let encoded = entity.pack();
console.log("encoded: ", encoded);

let [decoded, size] = new Hello().unpack(encoded);
console.log("decoded: ", decoded, "size: ", size);
