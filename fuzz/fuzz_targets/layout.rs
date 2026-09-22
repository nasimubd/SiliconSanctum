#![no_main]

use libfuzzer_sys::fuzz_target;
use sanctum_core::darwin::direct_io::validate_layout;

fuzz_target!(|input: (usize, u8)| {
    let (size, exponent) = input;
    let alignment = 1_usize.checked_shl(u32::from(exponent)).unwrap_or(0);
    let result = validate_layout(size, alignment);
    if result.is_ok() {
        assert!(alignment >= size_of::<*const ()>());
        assert!(alignment.is_power_of_two());
        assert_eq!(alignment % size_of::<*const ()>(), 0);
        assert!(std::alloc::Layout::from_size_align(size.max(1), alignment).is_ok());
    }
});
