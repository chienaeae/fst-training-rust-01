#[cfg(not(miri))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    println!("Make FST Network great!");
    println!("Сделайте FST Network отличным!");
}
