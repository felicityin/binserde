use types::blockchain::*;
use types::complex::*;

fn main() {
    test_blockchain();
    test_complex();
}

fn test_blockchain() {
    let header = Header {
        raw:   RawHeader {
            version:           1,
            compact_target:    2,
            timestamp:         3,
            number:            4,
            epoch:             5,
            parent_hash:       [1u8; 32],
            transactions_root: [2u8; 32],
            proposals_hash:    [3u8; 32],
            uncles_hash:       [4u8; 32],
            dao:               [5u8; 32],
        },
        nonce: 1,
    };

    let tx = Transaction {
        raw:       RawTransaction {
            version:      1,
            cell_deps:    vec![CellDep {
                out_point: OutPoint {
                    tx_hash: [2u8; 32],
                    index:   3,
                },
                dep_type:  1,
            }],
            header_deps:  vec![Bytes32 { v: [4u8; 32] }],
            inputs:       vec![CellInput {
                since:           5,
                previous_output: OutPoint {
                    tx_hash: [6u8; 32],
                    index:   7,
                },
            }],
            outputs:      vec![CellOutput {
                capacity: 8,
                lock:     Script {
                    code_hash: [9u8; 32],
                    hash_type: 10,
                    args:      vec![11],
                },
                type_:    None,
            }],
            outputs_data: vec![Bytes { v: vec![12u8] }],
        },
        witnesses: vec![Bytes { v: vec![1u8] }],
    };

    let block = Block {
        header,
        uncles: vec![],
        transactions: vec![tx],
        proposals: vec![ProposalShortId { v: [13u8; 10] }],
    };

    let encoded: Vec<u8> = block.pack().unwrap();
    println!("encoded block: {:?}, len: {}", encoded, encoded.len());

    let (decoded, len): (Block, usize) = Block::default().unpack(&encoded).unwrap();
    println!("decoded block: {:?}, len: {}\n", decoded, len);
}

fn test_complex() {
    let garage = Garage {
        car:     Vehicle {
            distance: vec![BaseDataOpt {
                v: Some(BaseData::A(vec![1])),
            }],
            gas:      vec![BigNumberOpt {
                v: Some(BigNumber::U64(2)),
            }],
        },
        monitor: AllRoad::BaseData(BaseData::B(3)),
    };

    let encoded: Vec<u8> = garage.pack().unwrap();
    println!("encoded garage: {:?}, len: {}", encoded, encoded.len());

    let (decoded, len): (Garage, usize) = Garage::default().unpack(&encoded).unwrap();
    println!("decoded garage: {:?}, len: {}\n", decoded, len);
}
