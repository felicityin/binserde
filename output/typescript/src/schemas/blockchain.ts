// Generated by Binserde 0.1.0

import {
    Arr,
    Base,
    Bool,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Option,
    String,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uint128,
    Float32,
    Float64,
    Vec,
} from "bincoder";

export class Script extends Base {
    code_hash: Arr<Uint8, 32>;
    hash_type: Uint8;
    args: Vec<Uint8>;

    constructor(
        code_hash: Arr<Uint8, 32> = new Arr(Uint8, 32),
        hash_type: Uint8 = new Uint8(),
        args: Vec<Uint8> = new Vec(Uint8),
    ) {
        super();
        this.code_hash = code_hash;
        this.hash_type = hash_type;
        this.args = args;
    }
}

export class OutPoint extends Base {
    tx_hash: Arr<Uint8, 32>;
    index: Uint32;

    constructor(
        tx_hash: Arr<Uint8, 32> = new Arr(Uint8, 32),
        index: Uint32 = new Uint32(),
    ) {
        super();
        this.tx_hash = tx_hash;
        this.index = index;
    }
}

export class CellInput extends Base {
    since: Uint64;
    previous_output: OutPoint;

    constructor(
        since: Uint64 = new Uint64(),
        previous_output: OutPoint = new OutPoint(),
    ) {
        super();
        this.since = since;
        this.previous_output = previous_output;
    }
}

export class CellOutput extends Base {
    capacity: Uint64;
    lock: Script;
    type_: Option<Script>;

    constructor(
        capacity: Uint64 = new Uint64(),
        lock: Script = new Script(),
        type_: Option<Script> = new Option(new Script()),
    ) {
        super();
        this.capacity = capacity;
        this.lock = lock;
        this.type_ = type_;
    }
}

export class CellDep extends Base {
    out_point: OutPoint;
    dep_type: Uint8;

    constructor(
        out_point: OutPoint = new OutPoint(),
        dep_type: Uint8 = new Uint8(),
    ) {
        super();
        this.out_point = out_point;
        this.dep_type = dep_type;
    }
}

export class RawTransaction extends Base {
    version: Uint32;
    cell_deps: Vec<CellDep>;
    header_deps: Vec<Bytes32>;
    inputs: Vec<CellInput>;
    outputs: Vec<CellOutput>;
    outputs_data: Vec<Bytes>;

    constructor(
        version: Uint32 = new Uint32(),
        cell_deps: Vec<CellDep> = new Vec(CellDep),
        header_deps: Vec<Bytes32> = new Vec(Bytes32),
        inputs: Vec<CellInput> = new Vec(CellInput),
        outputs: Vec<CellOutput> = new Vec(CellOutput),
        outputs_data: Vec<Bytes> = new Vec(Bytes),
    ) {
        super();
        this.version = version;
        this.cell_deps = cell_deps;
        this.header_deps = header_deps;
        this.inputs = inputs;
        this.outputs = outputs;
        this.outputs_data = outputs_data;
    }
}

export class Transaction extends Base {
    raw: RawTransaction;
    witnesses: Vec<Bytes>;

    constructor(
        raw: RawTransaction = new RawTransaction(),
        witnesses: Vec<Bytes> = new Vec(Bytes),
    ) {
        super();
        this.raw = raw;
        this.witnesses = witnesses;
    }
}

export class Bytes extends Base {
    v: Vec<Uint8>;

    constructor(v: Vec<Uint8> = new Vec(Uint8)) {
        super();
        this.v = v;
    }
}

export class Bytes32 extends Base {
    v: Arr<Uint8, 32>;

    constructor(v: Arr<Uint8, 32> = new Arr(Uint8, 32)) {
        super();
        this.v = v;
    }
}

export class RawHeader extends Base {
    version: Uint32;
    compact_target: Uint32;
    timestamp: Uint64;
    number: Uint64;
    epoch: Uint64;
    parent_hash: Arr<Uint8, 32>;
    transactions_root: Arr<Uint8, 32>;
    proposals_hash: Arr<Uint8, 32>;
    uncles_hash: Arr<Uint8, 32>;
    dao: Arr<Uint8, 32>;

    constructor(
        version: Uint32 = new Uint32(),
        compact_target: Uint32 = new Uint32(),
        timestamp: Uint64 = new Uint64(),
        number: Uint64 = new Uint64(),
        epoch: Uint64 = new Uint64(),
        parent_hash: Arr<Uint8, 32> = new Arr(Uint8, 32),
        transactions_root: Arr<Uint8, 32> = new Arr(Uint8, 32),
        proposals_hash: Arr<Uint8, 32> = new Arr(Uint8, 32),
        uncles_hash: Arr<Uint8, 32> = new Arr(Uint8, 32),
        dao: Arr<Uint8, 32> = new Arr(Uint8, 32),
    ) {
        super();
        this.version = version;
        this.compact_target = compact_target;
        this.timestamp = timestamp;
        this.number = number;
        this.epoch = epoch;
        this.parent_hash = parent_hash;
        this.transactions_root = transactions_root;
        this.proposals_hash = proposals_hash;
        this.uncles_hash = uncles_hash;
        this.dao = dao;
    }
}

export class Header extends Base {
    raw: RawHeader;
    nonce: Uint128;

    constructor(
        raw: RawHeader = new RawHeader(),
        nonce: Uint128 = new Uint128(),
    ) {
        super();
        this.raw = raw;
        this.nonce = nonce;
    }
}

export class UncleBlock extends Base {
    header: Header;
    proposals: Vec<ProposalShortId>;

    constructor(
        header: Header = new Header(),
        proposals: Vec<ProposalShortId> = new Vec(ProposalShortId),
    ) {
        super();
        this.header = header;
        this.proposals = proposals;
    }
}

export class Block extends Base {
    header: Header;
    uncles: Vec<UncleBlock>;
    transactions: Vec<Transaction>;
    proposals: Vec<ProposalShortId>;

    constructor(
        header: Header = new Header(),
        uncles: Vec<UncleBlock> = new Vec(UncleBlock),
        transactions: Vec<Transaction> = new Vec(Transaction),
        proposals: Vec<ProposalShortId> = new Vec(ProposalShortId),
    ) {
        super();
        this.header = header;
        this.uncles = uncles;
        this.transactions = transactions;
        this.proposals = proposals;
    }
}

export class ProposalShortId extends Base {
    v: Arr<Uint8, 10>;

    constructor(v: Arr<Uint8, 10> = new Arr(Uint8, 10)) {
        super();
        this.v = v;
    }
}

export class CellbaseWitness extends Base {
    lock: Script;
    message: Bytes;

    constructor(lock: Script = new Script(), message: Bytes = new Bytes()) {
        super();
        this.lock = lock;
        this.message = message;
    }
}

export class WitnessArgs extends Base {
    lock: Option<Bytes>;
    input_type: Option<Bytes>;
    output_type: Option<Bytes>;

    constructor(
        lock: Option<Bytes> = new Option(new Bytes()),
        input_type: Option<Bytes> = new Option(new Bytes()),
        output_type: Option<Bytes> = new Option(new Bytes()),
    ) {
        super();
        this.lock = lock;
        this.input_type = input_type;
        this.output_type = output_type;
    }
}
