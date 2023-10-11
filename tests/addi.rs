use risemu::emulator::Emulator;
use risemu::exception::RVException;

mod macros;

#[test]
fn imm_op_addi1() {
    test_case!(
        14,
        0x00000000,
        vec![
            0x93, 0x00, 0x00, 0x00, // li x1, 0
            0x13, 0x87, 0x00, 0x00, // addi x14, x1, 0
        ]
    );
}

#[test]
fn imm_op_addi2() {
    test_case!(
        14,
        0x00000002,
        vec![
            0x93, 0x00, 0x10, 0x00, // li x1, 1
            0x13, 0x87, 0x10, 0x00, // addi x14, x1, 1
        ]
    );
}

#[test]
fn imm_op_addi3() {
    test_case!(
        14,
        0x0000000a,
        vec![
            0x93, 0x00, 0x30, 0x00, // li x1, 3
            0x13, 0x87, 0x70, 0x00, // addi x14, x1, 7
        ]
    );
}

#[test]
fn imm_op_addi4() {
    test_case!(
        14,
        0xfffffffffffff800,
        vec![
            0x93, 0x00, 0x00, 0x00, // li x1, 0
            0x13, 0x87, 0x00, 0x80, // addi x14, x1, 0x800
        ]
    );
}

#[test]
fn imm_op_addi5() {
    test_case!(
        14,
        0xffffffff80000000,
        vec![
            0xb7, 0x00, 0x00, 0x80, // lui x1, 0x80000
            0x13, 0x87, 0x00, 0x00, // addi x14, x1, 0
        ]
    );
}

#[test]
fn imm_op_addi6() {
    test_case!(
        14,
        0xffffffff7ffff800,
        vec![
            0xb7, 0x00, 0x00, 0x80, // lui x1, 0x80000
            0x13, 0x87, 0x00, 0x80, // addi x14, x1, 0x800
        ]
    );
}

#[test]
fn imm_op_addi7() {
    test_case!(
        14,
        0x00000000000007ff,
        vec![
            0x93, 0x00, 0x00, 0x00, // li x1, 0
            0x13, 0x87, 0xf0, 0x7f, // addi x14, x1, 0x7ff
        ]
    );
}

#[test]
fn imm_op_addi8() {
    test_case!(
        14,
        0x000000007fffffff,
        vec![
            0xb7, 0x00, 0x00, 0x80, // lui x1, 0x80000
            0x9b, 0x80, 0xf0, 0xff, // addiw x1, x1, -1
            0x13, 0x87, 0x00, 0x00, // addi x14, x1, 0
        ]
    );
}

#[test]
fn imm_op_addi9() {
    test_case!(
        14,
        0x00000000800007fe,
        vec![
            0xb7, 0x00, 0x00, 0x80, // lui x1, 0x80000
            0x9b, 0x80, 0xf0, 0xff, // addiw x1, x1, -1
            0x13, 0x87, 0xf0, 0x7f, // addi x14, x1, 0x7ff
        ]
    );
}

#[test]
fn imm_op_addi10() {
    test_case!(
        14,
        0xffffffff800007ff,
        vec![
            0xb7, 0x00, 0x00, 0x80, // lui x1, 0x80000
            0x13, 0x87, 0xf0, 0x7f, // addi x14, x1, 0x7ff
        ]
    );
}

#[test]
fn imm_op_addi11() {
    test_case!(
        14,
        0x000000007ffff7ff,
        vec![
            0xb7, 0x00, 0x00, 0x80, // lui x1, 0x80000
            0x9b, 0x80, 0xf0, 0xff, // addiw x1, x1, -1
            0x13, 0x87, 0x00, 0x80, // addi x14, x1, 0x800
        ]
    );
}

#[test]
fn imm_op_addi12() {
    test_case!(
        14,
        0xffffffffffffffff,
        vec![
            0x93, 0x00, 0x00, 0x00, // li x1, 0
            0x13, 0x87, 0xf0, 0xff, // addi x14, x1, 0xfff
        ]
    );
}

#[test]
fn imm_op_addi13() {
    test_case!(
        14,
        0x0000000000000000,
        vec![
            0x93, 0x00, 0xf0, 0xff, // li x1, 0xffffffffffffffff
            0x13, 0x87, 0x10, 0x00, // addi x14, x1, 1
        ]
    );
}

#[test]
fn imm_op_addi14() {
    test_case!(
        14,
        0xfffffffffffffffe,
        vec![
            0x93, 0x00, 0xf0, 0xff, // li x1, 0xffffffffffffffff
            0x13, 0x87, 0xf0, 0xff, // addi x14, x1, -1
        ]
    );
}

#[test]
fn imm_op_addi15() {
    test_case!(
        14,
        0x0000000080000000,
        vec![
            0xb7, 0x00, 0x00, 0x80, // lui x1, 0x80000
            0x9b, 0x80, 0xf0, 0xff, // addiw x1, x1, -1
            0x13, 0x87, 0x10, 0x00, // addi x14, x1, 1
        ]
    );
}

#[test]
fn imm_src1_eq_dest_addi1() {
    test_case!(
        1,
        24,
        vec![
            0x93, 0x00, 0xd0, 0x00, // li x1, 13
            0x93, 0x80, 0xb0, 0x00, // addi x1, x1, 11
        ]
    );
}

#[test]
fn imm_dest_bypass_addi1() {
    test_case!(
        14,
        24,
        vec![
            0x13, 0x02, 0x00, 0x00, // li x4, 0
            0x93, 0x00, 0xd0, 0x00, // li x1, 13
            0x13, 0x87, 0xb0, 0x00, // addi x14, x1, 11
            0x13, 0x03, 0x07, 0x00, // addi x6, x14, 0
            0x13, 0x02, 0x12, 0x00, // addi x4, x4, 1
            0x93, 0x02, 0x20, 0x00, // li x5, 2
            0xe3, 0x16, 0x52, 0xfe, // bne x4, x5, 1b
        ]
    );
}

#[test]
fn imm_dest_bypass_addi2() {
    test_case!(
        14,
        23,
        vec![
            0x13, 0x02, 0x00, 0x00, // li x4, 0
            0x93, 0x00, 0xd0, 0x00, // li x1, 13
            0x13, 0x87, 0xa0, 0x00, // addi x14, x1, 10
            0x13, 0x00, 0x00, 0x00, // nop
            0x13, 0x03, 0x07, 0x00, // addi x6, x14, 0
            0x13, 0x02, 0x12, 0x00, // addi x4, x4, 1
            0x93, 0x02, 0x20, 0x00, // li x5, 2
            0xe3, 0x16, 0x52, 0xfe, // bne x4, x5, 1b
        ]
    );
}

#[test]
fn imm_dest_bypass_addi3() {
    test_case!(
        14,
        22,
        vec![
            0x13, 0x02, 0x00, 0x00, // li x4, 0
            0x93, 0x00, 0xd0, 0x00, // li x1, 13
            0x13, 0x87, 0x90, 0x00, // addi x14, x1, 9
            0x13, 0x00, 0x00, 0x00, // nop
            0x13, 0x00, 0x00, 0x00, // nop
            0x13, 0x03, 0x07, 0x00, // addi x6, x14, 0
            0x13, 0x02, 0x12, 0x00, // addi x4, x4, 1
            0x93, 0x02, 0x20, 0x00, // li x5, 2
            0xe3, 0x16, 0x52, 0xfe, // bne x4, x5, 1b
        ]
    );
}

#[test]
fn imm_src1_bypass_addi1() {
    test_case!(
        14,
        24,
        vec![
            0x13, 0x02, 0x00, 0x00, // li x4, 0
            0x93, 0x00, 0xd0, 0x00, // li x1, 13
            0x13, 0x87, 0xb0, 0x00, // addi x14, x1, 11
            0x13, 0x03, 0x07, 0x00, // addi x6, x14, 0
            0x13, 0x02, 0x12, 0x00, // addi x4, x4, 1
            0x93, 0x02, 0x20, 0x00, // li x5, 2
            0xe3, 0x16, 0x52, 0xfe, // bne x4, x5, 1b
        ]
    );
}

#[test]
fn imm_src1_bypass_addi2() {
    test_case!(
        14,
        23,
        vec![
            0x13, 0x02, 0x00, 0x00, // li x4, 0
            0x93, 0x00, 0xd0, 0x00, // li x1, 13
            0x13, 0x00, 0x00, 0x00, // nop
            0x13, 0x87, 0xa0, 0x00, // addi x14, x1, 10
            0x13, 0x03, 0x07, 0x00, // addi x6, x14, 0
            0x13, 0x02, 0x12, 0x00, // addi x4, x4, 1
            0x93, 0x02, 0x20, 0x00, // li x5, 2
            0xe3, 0x16, 0x52, 0xfe, // bne x4, x5, 1b
        ]
    );
}

#[test]
fn imm_src1_bypass_addi3() {
    test_case!(
        14,
        22,
        vec![
            0x13, 0x02, 0x00, 0x00, // li x4, 0
            0x93, 0x00, 0xd0, 0x00, // li x1, 13
            0x13, 0x00, 0x00, 0x00, // nop
            0x13, 0x00, 0x00, 0x00, // nop
            0x13, 0x87, 0x90, 0x00, // addi x14, x1, 9
            0x13, 0x03, 0x07, 0x00, // addi x6, x14, 0
            0x13, 0x02, 0x12, 0x00, // addi x4, x4, 1
            0x93, 0x02, 0x20, 0x00, // li x5, 2
            0xe3, 0x16, 0x52, 0xfe, // bne x4, x5, 1b
        ]
    );
}

#[test]
fn imm_zerosrc1_addi1() {
    test_case!(
        1,
        32,
        vec![
            0x93, 0x00, 0x00, 0x02, // addi x1, x0, 32
        ]
    );
}

#[test]
fn imm_zerodest_addi1() {
    test_case!(
        0,
        0,
        vec![
            0x93, 0x00, 0x10, 0x02, // li x1, 33
            0x13, 0x80, 0x20, 0x03, // addi x0, x1, 50
        ]
    );
}