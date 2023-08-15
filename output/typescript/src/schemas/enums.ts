// Generated by Binserde 0.1.0

import { Arr, Base, Bool, Int8, Int16, Int32, Int64, Int128, Option, String, Uint8, Uint16, Uint32, Uint64, Uint128, Float32, Float64, Vec } from 'bincoder';

export enum Test5Enum {
  A,
  B,
  C,
}

export class Test5 extends Base {
  init() {
    return {
      [Test5Enum.A]: new Bool(),
      [Test5Enum.B]: new Test6(),
      [Test5Enum.C]: new Vec(Uint32),
    }
  }
}

export class Test6 extends Base {
  a: Uint32;

  constructor(a: Uint32 = new Uint32()) {
    super()
    this.a = a;
  }
}

export class Test7 extends Base {
  a: Vec<Test5>;

  constructor(a: Vec<Test5> = new Vec(Test5)) {
    super()
    this.a = a;
  }
}
