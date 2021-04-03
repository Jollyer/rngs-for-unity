use std::intrinsics::transmute;
use rand::RngCore;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

#[no_mangle]
pub extern fn create_rng(seed: u64) -> *mut Xoshiro256PlusPlus {
    let rng = Xoshiro256PlusPlus::seed_from_u64(seed);
    unsafe { transmute(Box::new(rng)) }
}

#[no_mangle]
pub extern fn destroy_rng(p_rng: *mut Xoshiro256PlusPlus) {
    let _: Box<Xoshiro256PlusPlus> = unsafe { transmute(p_rng) };
}

#[no_mangle]
pub extern fn next_rng(p_rng: *mut Xoshiro256PlusPlus) -> u64 {
    let rng = unsafe { &mut *p_rng };
    rng.next_u64()
}