#![cfg_attr(target_os = "uefi", no_main)]
#![no_std]

cfg_select! {
    target_os = "uefi" => {
        #[uefi::entry]
        fn main() -> uefi::Status {
            uefi::helpers::init().unwrap();

            flate2_example_no_std::main();

            cfg_select! {
                not(feature = "qemu") => {
                    uefi::Status::SUCCESS
                }
                any(target_arch = "x86", target_arch = "x86_64") => {
                    use qemu_exit::QEMUExit;
                    let qemu_exit_handle = unsafe { qemu_exit::X86::new(0xF4, 123) };
                    qemu_exit_handle.exit_success();
                }
                _ => unimplemented!()
            }
        }
    }
    _ => {
        extern crate std;

        fn main() {
            flate2_example_no_std::main();
        }
    }
}
