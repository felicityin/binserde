// Generated by Binserde 0.1.0

import { Arr, Base, Bool, Int8, Int16, Int32, Int64, Int128, Option, String, Uint8, Uint16, Uint32, Uint64, Uint128, Float32, Float64, Vec } from 'bincoder';

export class OptionTest extends Base {
  a: Option<Uint8>;
  b: Option<Vec<Uint32>>;
  c: Option<Test8>;
  d: Option<Test9>;

  constructor(a: Option<Uint8> = new Option(new Uint8()), b: Option<Vec<Uint32>> = new Option(new Vec(Uint32)), c: Option<Test8> = new Option(new Test8()), d: Option<Test9> = new Option(new Test9())) {
    super()
    this.a = a;
    this.b = b;
    this.c = c;
    this.d = d;
  }
}

export class Test8 extends Base {
  a: Uint128;

  constructor(a: Uint128 = new Uint128()) {
    super()
    this.a = a;
  }
}

export enum Test9Enum {
  A,
}

export class Test9 extends Base {
  init() {
    return {
      [Test9Enum.A]: new Bool(),
    }
  }
}
