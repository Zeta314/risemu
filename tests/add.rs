use risemu::emulator::Emulator;
use risemu::exception::RVException;

mod macros;

#[test]
fn rr_op_add1() {
    test_case!(
        14,
        0x00000000,
        vec![
            0x93, 0x00, 0x00, 0x00, // li x1, 0
            0x13, 0x01, 0x00, 0x00, // li x2, 0
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
        ]
    );
}

#[test]
fn rr_op_add2() {
    test_case!(
        14,
        0x00000002,
        vec![
            0x93, 0x00, 0x10, 0x00, // li x1, 1
            0x13, 0x01, 0x10, 0x00, // li x2, 1
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
        ]
    );
}

#[test]
fn rr_op_add3() {
    test_case!(
        14,
        0x0000000a,
        vec![
            0x93, 0x00, 0x30, 0x00, // li x1, 3
            0x13, 0x01, 0x70, 0x00, // li x2, 7
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
        ]
    );
}

#[test]
fn rr_op_add4() {
    test_case!(
        14,
        0xffffffffffff8000,
        vec![
            0x93, 0x00, 0x00, 0x00, // li x1, 0x00000
            0x37, 0x81, 0xff, 0xff, // lui x2, 0xffff8
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
        ]
    );
}

#[test]
fn rr_op_add5() {
    test_case!(
        14,
        0xffffffff80000000,
        vec![
            0xb7, 0x00, 0x00, 0x80, // lui x1, 0xffff8
            0x37, 0x01, 0x00, 0x00, // li x2, 0x00000
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
        ]
    );
}

#[test]
fn rr_op_add6() {
    test_case!(
        14,
        0xffffffff7fff8000,
        vec![
            0xb7, 0x00, 0x00, 0x80, // lui x1, 0xffff8
            0x37, 0x81, 0xff, 0xff, // li x2, 0xffff8
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
        ]
    );
}

#[test]
fn rr_op_add7() {
    test_case!(
        14,
        0x0000000000007fff,
        vec![
            0x93, 0x00, 0x00, 0x00, // li x1, 0
            0x37, 0x81, 0x00, 0x00, // lui x2, 8
            0x1b, 0x01, 0xf1, 0xff, // addiw x2, x2, -1
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
        ]
    );
}

#[test]
fn rr_op_add8() {
    test_case!(
        14,
        0x000000007fffffff,
        vec![
            0xb7, 0x00, 0x00, 0x80, // lui x1,0x80000
            0x9b, 0x80, 0xf0, 0xff, // addiw x1, x1, -1
            0x13, 0x01, 0x00, 0x00, // li x2, 0
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
        ]
    );
}

#[test]
fn rr_op_add9() {
    test_case!(
        14,
        0x0000000080007ffe,
        vec![
            0xb7, 0x00, 0x00, 0x80, // lui x1, 0x80000
            0x9b, 0x80, 0xf0, 0xff, // addiw x1, x1, -1
            0x37, 0x81, 0x00, 0x00, // lui x2, 0x8
            0x1b, 0x01, 0xf1, 0xff, // addiw x2, x2, -1
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
        ]
    );
}

#[test]
fn rr_op_add10() {
    test_case!(
        14,
        0xffffffff80007fff,
        vec![
            0xb7, 0x00, 0x00, 0x80, // lui x1, 0x80000
            0x37, 0x81, 0x00, 0x00, // lui x2, 0x8
            0x1b, 0x01, 0xf1, 0xff, // addiw x2, x2, -1
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
        ]
    );
}

#[test]
fn rr_op_add11() {
    test_case!(
        14,
        0x000000007fff7fff,
        vec![
            0xb7, 0x00, 0x00, 0x80, // lui x1, 0x80000
            0x9b, 0x80, 0xf0, 0xff, // addiw x1, x1, -1
            0x37, 0x81, 0xff, 0xff, // lui x2, 0xffff8
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
        ]
    );
}

#[test]
fn rr_op_add12() {
    test_case!(
        14,
        0xffffffffffffffff,
        vec![
            0x93, 0x00, 0x00, 0x00, // li x1, 0
            0x13, 0x01, 0xf0, 0xff, // li x2, -1
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
        ]
    );
}

#[test]
fn rr_op_add13() {
    test_case!(
        14,
        0x0000000000000000,
        vec![
            0x93, 0x00, 0xf0, 0xff, // li, x1, -1
            0x13, 0x01, 0x10, 0x00, // li, x2, 1
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
        ]
    );
}

#[test]
fn rr_op_add14() {
    test_case!(
        14,
        0xfffffffffffffffe,
        vec![
            0x93, 0x00, 0xf0, 0xff, // li x1, -1
            0x13, 0x01, 0xf0, 0xff, // li x2, -1
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
        ]
    );
}

#[test]
fn rr_op_add15() {
    test_case!(
        14,
        0x0000000080000000,
        vec![
            0x93, 0x00, 0x10, 0x00, // li x1, 1
            0x37, 0x01, 0x00, 0x80, // lui x2, 0x80000
            0x1b, 0x01, 0xf1, 0xff, // addiw x2, x2, -1
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
        ]
    );
}

#[test]
fn rr_src1_eq_dest_add1() {
    test_case!(
        1,
        24,
        vec![
            0x93, 0x00, 0xd0, 0x00, // li x1, 13
            0x13, 0x01, 0xb0, 0x00, // li x2, 11
            0xb3, 0x80, 0x20, 0x00, // add x1, x1, x2
        ]
    );
}

#[test]
fn rr_src1_eq_dest_add2() {
    test_case!(
        1,
        25,
        vec![
            0x93, 0x00, 0xe0, 0x00, // li x1, 14
            0x13, 0x01, 0xb0, 0x00, // li x2, 11
            0xb3, 0x80, 0x20, 0x00, // add x1, x1, x2
        ]
    );
}

#[test]
fn rr_src12_eq_dest_add1() {
    test_case!(
        1,
        26,
        vec![
            0x93, 0x00, 0xd0, 0x00, // li x1, 13
            0xb3, 0x80, 0x10, 0x00, // add x1, x1, x1
        ]
    );
}

#[test]
fn rr_dest_bypass_add1() {
    test_case!(
        6,
        24,
        vec![
            0x13, 0x02, 0x00, 0x00, // li x4, 0
            0x93, 0x00, 0xd0, 0x00, // li x1, 13
            0x13, 0x01, 0xb0, 0x00, // li x2, 11
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
            0x13, 0x03, 0x07, 0x00, // addi x6, x14, 0
            0x13, 0x02, 0x12, 0x00, // addi x4, x4, 1
            0x93, 0x02, 0x20, 0x00, // li x5, 2
            0xe3, 0x14, 0x52, 0xfe, // bne x4, x5, 1b
        ]
    );
}

#[test]
fn rr_dest_bypass_add2() {
    test_case!(
        6,
        25,
        vec![
            0x13, 0x02, 0x00, 0x00, // li x4, 0
            0x93, 0x00, 0xe0, 0x00, // li x1, 14
            0x13, 0x01, 0xb0, 0x00, // li x2, 11
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
            0x13, 0x00, 0x00, 0x00, // nop
            0x13, 0x03, 0x07, 0x00, // addi x6, x14, 0
            0x13, 0x02, 0x12, 0x00, // addi x4, x4, 1
            0x93, 0x02, 0x20, 0x00, // li x5, 2
            0xe3, 0x14, 0x52, 0xfe, // bne x4, x5, 1b
        ]
    );
}

#[test]
fn rr_dest_bypass_add3() {
    test_case!(
        6,
        26,
        vec![
            0x13, 0x02, 0x00, 0x00, // li x4, 0
            0x93, 0x00, 0xf0, 0x00, // li x1, 15
            0x13, 0x01, 0xb0, 0x00, // li x2, 11
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
            0x13, 0x00, 0x00, 0x00, // nop
            0x13, 0x00, 0x00, 0x00, // nop
            0x13, 0x03, 0x07, 0x00, // addi x6, x14, 0
            0x13, 0x02, 0x12, 0x00, // addi x4, x4, 1
            0x93, 0x02, 0x20, 0x00, // li x5, 2
            0xe3, 0x14, 0x52, 0xfe, // bne x4, x5, 1b
        ]
    );
}

#[test]
fn rr_src12_bypass_add1() {
    test_case!(
        14,
        24,
        vec![
            0x13, 0x02, 0x00, 0x00, // li x4, 0
            0x93, 0x00, 0xd0, 0x00, // li x1, 13
            0x13, 0x01, 0xb0, 0x00, // li x2, 11
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
            0x13, 0x02, 0x12, 0x00, // addi x4, x4, 1
            0x93, 0x02, 0x20, 0x00, // li x5, 2
            0xe3, 0x16, 0x52, 0xfe, // bne x4, x5, 1b
        ]
    );
}

#[test]
fn rr_src12_bypass_add2() {
    test_case!(
        14,
        25,
        vec![
            0x13, 0x02, 0x00, 0x00, // li x4, 0
            0x93, 0x00, 0xe0, 0x00, // li x1, 13
            0x13, 0x01, 0xb0, 0x00, // li x2, 11
            0x13, 0x00, 0x00, 0x00, // nop
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
            0x13, 0x02, 0x12, 0x00, // addi x4, x4, 1
            0x93, 0x02, 0x20, 0x00, // li x5, 2
            0xe3, 0x16, 0x52, 0xfe, // bne x4, x5, 1b
        ]
    );
}

#[test]
fn rr_src12_bypass_add3() {
    test_case!(
        14,
        26,
        vec![
            0x13, 0x02, 0x00, 0x00, // li x4, 0
            0x93, 0x00, 0xf0, 0x00, // li x1, 15
            0x13, 0x01, 0xb0, 0x00, // li x2, 11
            0x13, 0x00, 0x00, 0x00, // nop
            0x13, 0x00, 0x00, 0x00, // nop
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
            0x13, 0x02, 0x12, 0x00, // addi x4, x4, 1
            0x93, 0x02, 0x20, 0x00, // li x5, 2
            0xe3, 0x16, 0x52, 0xfe, // bne x4, x5, 1b
        ]
    );
}

#[test]
fn rr_src12_bypass_add4() {
    test_case!(
        14,
        24,
        vec![
            0x13, 0x02, 0x00, 0x00, // li x4, 0
            0x93, 0x00, 0xd0, 0x00, // li x1, 13
            0x13, 0x00, 0x00, 0x00, // nop
            0x13, 0x01, 0xb0, 0x00, // li x2, 11
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
            0x13, 0x02, 0x12, 0x00, // addi x4, x4, 1
            0x93, 0x02, 0x20, 0x00, // li x5, 2
            0xe3, 0x16, 0x52, 0xfe, // bne x4, x5, 1b
        ]
    );
}

#[test]
fn rr_src12_bypass_add5() {
    test_case!(
        14,
        25,
        vec![
            0x13, 0x02, 0x00, 0x00, // li x4, 0
            0x93, 0x00, 0xe0, 0x00, // li x1, 13
            0x13, 0x00, 0x00, 0x00, // nop
            0x13, 0x01, 0xb0, 0x00, // li x2, 11
            0x13, 0x00, 0x00, 0x00, // nop
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
            0x13, 0x02, 0x12, 0x00, // addi x4, x4, 1
            0x93, 0x02, 0x20, 0x00, // li x5, 2
            0xe3, 0x16, 0x52, 0xfe, // bne x4, x5, 1b
        ]
    );
}

#[test]
fn rr_src12_bypass_add6() {
    test_case!(
        14,
        26,
        vec![
            0x13, 0x02, 0x00, 0x00, // li x4, 0
            0x93, 0x00, 0xf0, 0x00, // li x1, 15
            0x13, 0x00, 0x00, 0x00, // nop
            0x13, 0x00, 0x00, 0x00, // nop
            0x13, 0x01, 0xb0, 0x00, // li x2, 11
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
            0x13, 0x02, 0x12, 0x00, // addi x4, x4, 1
            0x93, 0x02, 0x20, 0x00, // li x5, 2
            0xe3, 0x16, 0x52, 0xfe, // bne x4, x5, 1b
        ]
    );
}

#[test]
fn rr_src21_bypass_add1() {
    test_case!(
        14,
        24,
        vec![
            0x13, 0x02, 0x00, 0x00, // li x4, 0
            0x13, 0x01, 0xb0, 0x00, // li x2, 11
            0x93, 0x00, 0xd0, 0x00, // li x1, 13
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
            0x13, 0x02, 0x12, 0x00, // addi x4, x4, 1
            0x93, 0x02, 0x20, 0x00, // li x5, 2
            0xe3, 0x16, 0x52, 0xfe, // bne x4, x5, 1b
        ]
    );
}

#[test]
fn rr_src21_bypass_add2() {
    test_case!(
        14,
        25,
        vec![
            0x13, 0x02, 0x00, 0x00, // li x4, 0
            0x13, 0x01, 0xb0, 0x00, // li x2, 11
            0x93, 0x00, 0xe0, 0x00, // li x1, 13
            0x13, 0x00, 0x00, 0x00, // nop
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
            0x13, 0x02, 0x12, 0x00, // addi x4, x4, 1
            0x93, 0x02, 0x20, 0x00, // li x5, 2
            0xe3, 0x16, 0x52, 0xfe, // bne x4, x5, 1b
        ]
    );
}

#[test]
fn rr_src21_bypass_add3() {
    test_case!(
        14,
        26,
        vec![
            0x13, 0x02, 0x00, 0x00, // li x4, 0
            0x13, 0x01, 0xb0, 0x00, // li x2, 11
            0x93, 0x00, 0xf0, 0x00, // li x1, 15
            0x13, 0x00, 0x00, 0x00, // nop
            0x13, 0x00, 0x00, 0x00, // nop
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
            0x13, 0x02, 0x12, 0x00, // addi x4, x4, 1
            0x93, 0x02, 0x20, 0x00, // li x5, 2
            0xe3, 0x16, 0x52, 0xfe, // bne x4, x5, 1b
        ]
    );
}

#[test]
fn rr_src21_bypass_add4() {
    test_case!(
        14,
        24,
        vec![
            0x13, 0x02, 0x00, 0x00, // li x4, 0
            0x13, 0x01, 0xb0, 0x00, // li x2, 11
            0x13, 0x00, 0x00, 0x00, // nop
            0x93, 0x00, 0xd0, 0x00, // li x1, 13
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
            0x13, 0x02, 0x12, 0x00, // addi x4, x4, 1
            0x93, 0x02, 0x20, 0x00, // li x5, 2
            0xe3, 0x16, 0x52, 0xfe, // bne x4, x5, 1b
        ]
    );
}

#[test]
fn rr_src21_bypass_add5() {
    test_case!(
        14,
        25,
        vec![
            0x13, 0x02, 0x00, 0x00, // li x4, 0
            0x13, 0x01, 0xb0, 0x00, // li x2, 11
            0x13, 0x00, 0x00, 0x00, // nop
            0x13, 0x00, 0x00, 0x00, // nop
            0x93, 0x00, 0xe0, 0x00, // li x1, 13
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
            0x13, 0x02, 0x12, 0x00, // addi x4, x4, 1
            0x93, 0x02, 0x20, 0x00, // li x5, 2
            0xe3, 0x16, 0x52, 0xfe, // bne x4, x5, 1b
        ]
    );
}

#[test]
fn rr_src21_bypass_add6() {
    test_case!(
        14,
        26,
        vec![
            0x13, 0x02, 0x00, 0x00, // li x4, 0
            0x13, 0x01, 0xb0, 0x00, // li x2, 11
            0x13, 0x00, 0x00, 0x00, // nop
            0x13, 0x00, 0x00, 0x00, // nop
            0x93, 0x00, 0xf0, 0x00, // li x1, 15
            0x33, 0x87, 0x20, 0x00, // add x14, x1, x2
            0x13, 0x02, 0x12, 0x00, // addi x4, x4, 1
            0x93, 0x02, 0x20, 0x00, // li x5, 2
            0xe3, 0x16, 0x52, 0xfe, // bne x4, x5, 1b
        ]
    );
}

#[test]
fn rr_zerosrc1_add1() {
    test_case!(
        2,
        15,
        vec![
            0x93, 0x00, 0xf0, 0x00, // li x1, 15
            0x33, 0x01, 0x10, 0x00, // add x2, x0, x1
        ]
    );
}

#[test]
fn rr_zerosrc2_add1() {
    test_case!(
        2,
        32,
        vec![
            0x93, 0x00, 0x00, 0x02, // li x1, 32
            0x33, 0x81, 0x00, 0x00, // add x2, x1, x0
        ]
    );
}

#[test]
fn rr_zerosrc12_add1() {
    test_case!(
        2,
        0,
        vec![
            0x33, 0x01, 0x00, 0x00, // add x2, x0, x0
        ]
    );
}

#[test]
fn rr_zerodest_add1() {
    test_case!(
        0,
        0,
        vec![
            0x93, 0x00, 0x00, 0x01, // li x1, 16
            0x13, 0x01, 0xe0, 0x01, // li x2, 30
            0x33, 0x80, 0x20, 0x00, // add x0, x1, x2
        ]
    );
}
