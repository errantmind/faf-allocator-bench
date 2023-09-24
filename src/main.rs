use core::hint::black_box;

// #[global_allocator]
// static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;
// const ALLOCATOR_NAME: &str = "SnMalloc";

// #[global_allocator]
// static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;
// const ALLOCATOR_NAME: &str = "Jemalloc";

// #[global_allocator]
// static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
// const ALLOCATOR_NAME: &str = "MiMalloc";

// #[global_allocator]
// static GLOBAL: mimalloc_rust::GlobalMiMalloc = mimalloc_rust::GlobalMiMalloc;
// const ALLOCATOR_NAME: &str = "MiMalloc";

#[global_allocator]
static GLOBAL: tcmalloc::TCMalloc = tcmalloc::TCMalloc;
const ALLOCATOR_NAME: &str = "TCMalloc";

//system allocator
//const ALLOCATOR_NAME: &str = "System Allocator";

fn main() {
   assert_eq!(u64::MAX, usize::MAX as u64);

   // must be power of 2
   const ALLOCATION_SIZES_TO_TEST: &[usize; 18] =
      &[2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 32768, 65536, 131072, 262144, 524288, 1048576];

   const NUM_ALLOCATIONS_BASE: usize = u32::MAX as usize;

   let mut ts: Timespec = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };

   {
      for &allocation_size in ALLOCATION_SIZES_TO_TEST {
         //if ALLOCATOR_NAME == "Jemalloc" {
            // use tikv_jemalloc_ctl::{epoch, opt, stats};
            // let e = epoch::mib().unwrap();
            // e.advance().unwrap();
            // let allocated = stats::allocated::mib().unwrap();
            // let resident = stats::resident::mib().unwrap();
            // println!("tcache {:?}", opt::tcache::read().unwrap());
            // println!("tcache_max {:?}", opt::tcache_max::read().unwrap());
            // println!("background_thread {:?}", opt::background_thread::read().unwrap());
            // println!("{} bytes allocated/{} bytes resident", allocated.read().unwrap(), resident.read().unwrap());
         //}
         let num_allocations = NUM_ALLOCATIONS_BASE / 16;
         let start_time_nanos = get_epoch_nanos(&mut ts);
         for _ in 0..num_allocations {
            black_box(Vec::<u8>::with_capacity(allocation_size));
         }
         let end_time_nanos = get_epoch_nanos(&mut ts);
         let elapsed_nanos = end_time_nanos - start_time_nanos;
         print_output(ALLOCATOR_NAME, allocation_size, num_allocations, elapsed_nanos)
      }
   }
}


#[repr(C, align(64))]
pub struct Timespec {
   pub tv_sec: i64,
   pub tv_nsec: i64,
}

extern "C" {
   // We use this function instead of a direct syscall because this function uses VDSO, which is faster
   fn clock_gettime(clk_id: i32, tp: *mut Timespec) -> i32;
}

const CLOCK_MONOTONIC: i32 = 1;

// SAFETY:
// current epoch time is 1694832834
// 1694832834 * 1_000_000_000 = 1,694,832,834,000,000,000
// u64::MAX is                 18,446,744,073,709,551,615
// So, we are safe for a while longer
fn get_epoch_nanos(ts: &mut Timespec) -> u64 {
   unsafe { clock_gettime(CLOCK_MONOTONIC, ts as *mut Timespec) };
   ts.tv_sec as u64 * 1_000_000_000u64 + ts.tv_nsec as u64
}

fn print_output(lib_name: &str, allocation_size: usize, num_allocations: usize, nanoseconds: u64) {
   let ns_per_alloc = format!("{:.0}", nanoseconds as f64 / num_allocations as f64)
      .as_bytes()
      .rchunks(3)
      .rev()
      .map(core::str::from_utf8)
      .collect::<Result<Vec<&str>, _>>()
      .unwrap()
      .join(",");
   println!("{:<8} {:>10} bytes {:>11} allocs {:>8} ns per alloc", lib_name, allocation_size, num_allocations, ns_per_alloc);
}

// fn print_version() {
//    println!("{} v{} | repo: https://github.com/errantmind/faf-allocator-bench\n", statics::PROJECT_NAME, statics::VERSION,);
// }
