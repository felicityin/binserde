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

import {
    Block,
    Bytes,
    Bytes32,
    CellDep,
    CellOutput,
    CellInput,
    Transaction,
    ProposalShortId,
    OutPoint,
    Script,
    UncleBlock,
    Header,
    RawHeader,
    RawTransaction,
} from "./schemas/blockchain";
import {
    AllRoad,
    AllRoadEnum,
    BaseData,
    BaseDataEnum,
    BaseDataOpt,
    BigNumber,
    BigNumberEnum,
    BigNumberOpt,
    Garage,
    Vehicle,
} from "./schemas/complex";

function test_blockchain() {
    let header = new Header(
        new RawHeader(
            new Uint32(1),
            new Uint32(2),
            new Uint64(3n),
            new Uint64(4n),
            new Uint64(5n),
            new Arr(Uint8, 32, new Array(32).fill(new Uint8(1))),
            new Arr(Uint8, 32, new Array(32).fill(new Uint8(2))),
            new Arr(Uint8, 32, new Array(32).fill(new Uint8(3))),
            new Arr(Uint8, 32, new Array(32).fill(new Uint8(4))),
            new Arr(Uint8, 32, new Array(32).fill(new Uint8(5))),
        ),
        new Uint128(1n),
    );

    let tx = new Transaction(
        new RawTransaction(
            new Uint32(1),
            new Vec(CellDep, [
                new CellDep(
                    new OutPoint(
                        new Arr(Uint8, 32, new Array(32).fill(new Uint8(2))),
                        new Uint32(3),
                    ),
                    new Uint8(1),
                ),
            ]),
            new Vec(Bytes32, [
                new Bytes32(
                    new Arr(Uint8, 32, new Array(32).fill(new Uint8(4))),
                ),
            ]),
            new Vec(CellInput, [
                new CellInput(
                    new Uint64(5n),
                    new OutPoint(
                        new Arr(Uint8, 32, new Array(32).fill(new Uint8(6))),
                        new Uint32(7),
                    ),
                ),
            ]),
            new Vec(CellOutput, [
                new CellOutput(
                    new Uint64(8n),
                    new Script(
                        new Arr(Uint8, 32, new Array(32).fill(new Uint8(9))),
                        new Uint8(10),
                        new Vec(Uint8, [new Uint8(11)]),
                    ),
                    new Option<Script>(),
                ),
            ]),
            new Vec(Bytes, [new Bytes(new Vec(Uint8, [new Uint8(12)]))]),
        ),
        new Vec(Bytes, [new Bytes(new Vec(Uint8, [new Uint8(1)]))]),
    );

    let block = new Block(
        header,
        new Vec(UncleBlock, []),
        new Vec(Transaction, [tx]),
        new Vec(ProposalShortId, [
            new ProposalShortId(
                new Arr(Uint8, 10, new Array(10).fill(new Uint8(13))),
            ),
        ]),
    );

    let encoded = new Uint8Array(block.pack());
    var bytes = "";
    for (var i = 0; i < encoded.byteLength; i++) {
        bytes = bytes.concat(encoded[i].toString());
        bytes = bytes.concat(", ");
    }
    console.log("encoded block: ", bytes);

    let [decoded, size] = new Block().unpack(encoded);
    console.log("decoded block: ", decoded, "size: ", size);
}

function test_complex() {
    let baseData = new BaseData(BaseDataEnum.A);
    baseData.t[BaseDataEnum.A] = new Vec(Uint8, [new Uint8(1)]);

    let bigNum = new BigNumber(BigNumberEnum.U64);
    bigNum.t[BigNumberEnum.U64] = new Uint64(2n);

    let allRoad = new AllRoad(AllRoadEnum.BaseData);
    let baseData1 = new BaseData(BaseDataEnum.B);
    baseData1.t[BaseDataEnum.B] = new Uint32(3);
    allRoad.t[AllRoadEnum.BaseData] = baseData1;

    let garage = new Garage(
        new Vehicle(
            new Vec(BaseDataOpt, [new BaseDataOpt(new Option(baseData))]),
            new Vec(BigNumberOpt, [new BigNumberOpt(new Option(bigNum))]),
        ),
        allRoad,
    );

    let encoded = new Uint8Array(garage.pack());
    var bytes = "";
    for (var i = 0; i < encoded.byteLength; i++) {
        bytes = bytes.concat(encoded[i].toString());
        bytes = bytes.concat(", ");
    }
    console.log("encoded garage: ", bytes);

    let [decoded, size] = new Garage().unpack(encoded);
    console.log("decoded garage: ", decoded, "size: ", size);
}

test_blockchain();
test_complex();
