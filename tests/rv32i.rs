mod utils;

use riscv_emu::bus::DRAM_BASE;

#[test]
fn lui_rd_imm() {
    utils::execute(
        vec![
            0x37, 0x28, 0x00, 0x00, // lui x16, 2
        ],
        vec![(16, 8192)],
    );
}

#[test]
fn auipc_rd_imm() {
    utils::execute(
        vec![
            0x17, 0x28, 0x00, 0x00, // auipc x16, 2
        ],
        vec![(16, 0x2000 + DRAM_BASE)],
    );
}

#[test]
fn jal_rd_imm() {
    utils::execute(
        vec![
            0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
            0x93, 0x08, 0x50, 0x00, // addi x17, x0, 5
            0x6f, 0x09, 0xc0, 0x00, // jal x18, 12
        ],
        vec![(16, 3), (17, 5), (18, 12 + DRAM_BASE)],
    );
}

#[test]
fn jalr_rd_imm() {
    utils::execute(
        vec![
            0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
            0x93, 0x08, 0x50, 0x00, // addi x17, x0, 5
            0x67, 0x09, 0xc0, 0x02, // jalr x18, x0, 44
        ],
        vec![(16, 3), (17, 5), (18, 12 + DRAM_BASE)],
    );
}

#[test]
fn beq_rs1_rs2_imm() {
    utils::execute(
        vec![
            0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
            0x93, 0x08, 0x30, 0x00, // addi x17, x0, 3
            0x63, 0x06, 0x18, 0x01, // beq x16, x17, 12
        ],
        vec![(16, 3), (17, 3)],
    );
}

#[test]
fn bne_rs1_rs2_imm() {
    utils::execute(
        vec![
            0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
            0x93, 0x08, 0x50, 0x00, // addi x17, x0, 5
            0x63, 0x16, 0x18, 0x01, // bne x16, x17, 12
        ],
        vec![(16, 3), (17, 5)],
    );
}

#[test]
fn blt_rs1_rs2_imm() {
    utils::execute(
        vec![
            0x13, 0x08, 0xd0, 0xff, // addi x16, x0, -3
            0x93, 0x08, 0x50, 0x00, // addi x17, x0, 5
            0x63, 0x46, 0x18, 0x01, // blt x16, x17, -8
        ],
        vec![(16, -3i32 as u32), (17, 5)],
    );
}

#[test]
fn bge_rs1_rs2_imm() {
    utils::execute(
        vec![
            0x13, 0x08, 0xd0, 0xff, // addi x16, x0, -3
            0x93, 0x08, 0xd0, 0xff, // addi x17, x0, -3
            0x63, 0x56, 0x18, 0x01, // bge x16, x17, 12
        ],
        vec![(16, -3i32 as u32), (17, -3i32 as u32)],
    );
}

#[test]
fn bltu_rs1_rs2_imm() {
    utils::execute(
        vec![
            0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
            0x93, 0x08, 0x50, 0x00, // addi x17, x0, 5
            0x63, 0x66, 0x18, 0x01, // bltu x16, x17, 12
        ],
        vec![(16, 3), (17, 5)],
    );
}

#[test]
fn bgeu_rs1_rs2_imm() {
    utils::execute(
        vec![
            0x13, 0x08, 0x50, 0x00, // addi x16, x0, 5
            0x93, 0x08, 0x30, 0x00, // addi x17, x0, 3
            0x63, 0x76, 0x18, 0x01, // bgeu x16, x17, 12
        ],
        vec![(16, 5), (17, 3)],
    );
}

#[test]
fn lb_rd_offset_rs1() {
    assert!(false);
}

#[test]
fn lh_rd_offset_rs1() {
    assert!(false);
}

#[test]
fn lw_rd_offset_rs1() {
    assert!(false);
}

#[test]
fn lbu_rd_offset_rs1() {
    assert!(false);
}

#[test]
fn lhu_rd_offset_rs1() {
    assert!(false);
}

#[test]
fn sb_rs2_offset_rs1() {
    assert!(false);
}

#[test]
fn sh_rs2_offset_rs1() {
    assert!(false);
}

#[test]
fn sw_rs2_offset_rs1() {
    assert!(false);
}

#[test]
fn addi_rd_rs1_imm() {
    utils::execute(
        vec![
            0x93, 0x0F, 0x40, 0x00, // addi x31, x0, 4
        ],
        vec![(31, 4)],
    );
}

#[test]
fn slti_rd_rs1_imm() {
    utils::execute(
        vec![
            0x13, 0x08, 0xb0, 0xff, // addi x16 x0, -5
            0x93, 0x28, 0xe8, 0xff, // slti x17, x16, -2
        ],
        vec![(16, -5i32 as u32), (17, 1)],
    );
}

#[test]
fn sltiu_rd_rs1_imm() {
    utils::execute(
        vec![
            0x13, 0x08, 0x20, 0x00, // addi x16, x0, 2
            0x93, 0x38, 0x58, 0x00, // sltiu, x17, x16, 5
        ],
        vec![(16, 2), (17, 1)],
    );
}

#[test]
fn xori_rd_rs1_imm() {
    utils::execute(
        vec![
            0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
            0x93, 0x48, 0x68, 0x00, // xori, x17, x16, 6
        ],
        vec![(16, 3), (17, 5)],
    );
}

#[test]
fn ori_rd_rs1_imm() {
    utils::execute(
        vec![
            0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
            0x93, 0x68, 0x68, 0x00, // ori, x17, x16, 6
        ],
        vec![(16, 3), (17, 7)],
    );
}

#[test]
fn andi_rd_rs1_imm() {
    utils::execute(
        vec![
            0x13, 0x08, 0x40, 0x00, // addi x16, x0, 4
            0x93, 0x78, 0x78, 0x00, // andi, x17, x16, 7
        ],
        vec![(16, 4), (17, 4)],
    );
}

#[test]
fn slli_rd_rs1_imm() {
    utils::execute(
        vec![
            0x13, 0x08, 0x20, 0x00, // addi x16 x0, 2
            0x93, 0x18, 0x38, 0x00, // slli x17, x16, 3
        ],
        vec![(16, 2), (17, 16)],
    );
}

#[test]
fn srli_rd_rs1_imm() {
    utils::execute(
        vec![
            0x13, 0x08, 0x80, 0x00, // addi x16, x0, 8
            0x93, 0x58, 0x28, 0x00, // srli x17, x16, 2
        ],
        vec![(16, 8), (17, 2)],
    );
}

#[test]
fn srai_rd_rs1_imm() {
    utils::execute(
        vec![
            0x13, 0x08, 0x80, 0xff, // addi x16, x0, -8
            0x93, 0x58, 0x28, 0x40, // srai x17, x16, 2
        ],
        vec![(16, -8i32 as u32), (17, -2i32 as u32)],
    );
}

#[test]
fn add_rd_rs1_rs2() {
    utils::execute(
        vec![
            0x93, 0x01, 0x50, 0x00, // addi x3, x0, 5
            0x13, 0x02, 0x60, 0x00, // addi x4, x0, 6
            0x33, 0x81, 0x41, 0x00, // add x2, x3, x4
        ],
        vec![(2, 11), (3, 5), (4, 6)],
    );
}

#[test]
fn sub_rd_rs1_rs2() {
    utils::execute(
        vec![
            0x93, 0x01, 0x50, 0x00, // addi x3, x0, 5
            0x13, 0x02, 0x60, 0x00, // addi x4, x0, 6
            0x33, 0x81, 0x41, 0x40, // sub x2, x3, x4
        ],
        vec![(2, -1i32 as u32), (3, 5), (4, 6)],
    );
}

#[test]
fn sll_rd_rs1_rs2() {
    utils::execute(
        vec![
            0x13, 0x08, 0x80, 0x00, // addi x16, x0, 8
            0x93, 0x08, 0x20, 0x00, // addi x17, x0, 2
            0x33, 0x19, 0x18, 0x01, // sll x18, x16, x17
        ],
        vec![(16, 8), (17, 2), (18, 32)],
    );
}

#[test]
fn slt_rd_rs1_rs2() {
    utils::execute(
        vec![
            0x13, 0x08, 0x80, 0xff, // addi x16, x0, -8
            0x93, 0x08, 0x20, 0x00, // addi x17, x0, 2
            0x33, 0x29, 0x18, 0x01, // slt x18, x16, x17
        ],
        vec![(16, -8i32 as u32), (17, 2), (18, 1)],
    );
}

#[test]
fn sltu_rd_rs1_rs2() {
    utils::execute(
        vec![
            0x13, 0x08, 0x80, 0x00, // addi x16, x0, 8
            0x93, 0x08, 0x20, 0x00, // addi x17, x0, 2
            0x33, 0xb9, 0x08, 0x01, // slt x18, x17, x16
        ],
        vec![(16, 8), (17, 2), (18, 1)],
    );
}

#[test]
fn xor_rd_rs1_rs2() {
    utils::execute(
        vec![
            0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
            0x93, 0x08, 0x60, 0x00, // addi x17, x0, 6
            0x33, 0x49, 0x18, 0x01, // xor x18, x16, x17
        ],
        vec![(16, 3), (17, 6), (18, 5)],
    );
}

#[test]
fn srl_rd_rs1_rs2() {
    utils::execute(
        vec![
            0x13, 0x08, 0x00, 0x01, // addi x16, x0, 16
            0x93, 0x08, 0x20, 0x00, // addi x17, x0, 2
            0x33, 0x59, 0x18, 0x01, // srl x18, x16, x17
        ],
        vec![(16, 16), (17, 2), (18, 4)],
    );
}

#[test]
fn sra_rd_rs1_rs2() {
    utils::execute(
        vec![
            0x13, 0x08, 0x00, 0xff, // addi x16, x0, -16
            0x93, 0x08, 0x20, 0x00, // addi x17, x0, 2
            0x33, 0x59, 0x18, 0x41, // sra x18, x16, x17
        ],
        vec![(16, -16i32 as u32), (17, 2), (18, -4i32 as u32)],
    );
}

#[test]
fn or_rd_rs1_rs2() {
    utils::execute(
        vec![
            0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
            0x93, 0x08, 0x50, 0x00, // addi x17, x0, 5
            0x33, 0x69, 0x18, 0x01, // xor x18, x16, x17
        ],
        vec![(16, 3), (17, 5), (18, 7)],
    );
}

#[test]
fn and_rd_rs1_rs2() {
    utils::execute(
        vec![
            0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
            0x93, 0x08, 0x50, 0x00, // addi x17, x0, 5
            0x33, 0x79, 0x18, 0x01, // and x18, x16, x17
        ],
        vec![(16, 3), (17, 5), (18, 1)],
    );
}
