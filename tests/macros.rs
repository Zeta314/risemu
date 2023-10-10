#[macro_export]
macro_rules! test_case {
    ($register:expr, $result:expr, $code:expr) => {
        let mut emu = Emulator::new(0x10000);

        emu.init_ram($code);
        emu.run();

        assert_eq!(emu.cpu.xregs[$register], $result);
    };
}
