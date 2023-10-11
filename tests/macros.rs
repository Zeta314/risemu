#[macro_export]
macro_rules! test_case {
    ($register:expr, $result:expr, $code:expr) => {
        let mut emu = Emulator::new(0x10000);

        let mut code = vec![];
        code.extend($code);
        code.extend([0x73, 0x00, 0x00, 0x00]); // ECALL

        emu.init_ram(code);
        match emu.run() {
            Ok(_) => {}
            Err(ex) => {
                println!("{ex:#?} @ {:#x}", emu.cpu.pc);

                match ex {
                    RVException::EnvironmentCall => {}
                    _ => panic!(),
                }
            }
        }

        assert_eq!(emu.cpu.xregs[$register], $result);
    };
}
