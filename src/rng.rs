use fastrand::Rng;
use std::cell::RefCell;

thread_local! {
    static RNG: RefCell<Option<Rng>> = const { RefCell::new(None) };
}

/// Domain tag used when deriving sub-seeds for per-file generation.
pub const DOMAIN_FILE: u64 = 0;

/// Domain tag used when deriving sub-seeds for provider draws.
pub const DOMAIN_PROVIDER: u64 = 1;

/// Initialize the global RNG with an optional seed.
/// Kept for backwards compatibility with existing tests; production code should
/// use [`scoped_seeded`] instead so the thread-local is scoped to a known task.
#[cfg(test)]
pub fn initialize_rng(seed: Option<u64>) {
    RNG.with(|rng| {
        let mut rng_ref = rng.borrow_mut();
        *rng_ref = Some(match seed {
            Some(s) => Rng::with_seed(s),
            None => Rng::new(),
        });
    });
}

/// Execute a function with access to the global RNG
/// Auto-initializes with a random seed if not already initialized
pub fn with_rng<T>(f: impl FnOnce(&mut Rng) -> T) -> T {
    RNG.with(|rng| {
        let mut rng_ref = rng.borrow_mut();
        if rng_ref.is_none() {
            // Auto-initialize with random seed if not already done
            *rng_ref = Some(Rng::new());
        }
        let rng = rng_ref.as_mut().unwrap();
        f(rng)
    })
}

/// Generate a random boolean
pub fn bool() -> bool {
    with_rng(|rng| rng.bool())
}

/// Generate a random f64 in range [0.0, 1.0)
pub fn f64() -> f64 {
    with_rng(|rng| rng.f64())
}

/// Generate a random i32 in the given range
pub fn i32(range: std::ops::Range<i32>) -> i32 {
    with_rng(|rng| rng.i32(range))
}

/// Generate a random i64 in the given range
pub fn i64(range: std::ops::Range<i64>) -> i64 {
    with_rng(|rng| rng.i64(range))
}

/// Generate a random u32 in the given range
pub fn u32(range: std::ops::Range<u32>) -> u32 {
    with_rng(|rng| rng.u32(range))
}

/// Generate a random usize in the given range
pub fn usize(range: std::ops::RangeTo<usize>) -> usize {
    with_rng(|rng| rng.usize(range))
}

/// Generate a random alphanumeric character
pub fn alphanumeric() -> char {
    with_rng(|rng| rng.alphanumeric())
}

/// Generate a random f64 in the given range using fastrand_contrib
pub fn f64_range(range: std::ops::Range<f64>) -> f64 {
    with_rng(|rng| {
        // Use the same approach as fastrand_contrib::f64_range but with our RNG
        let start = range.start;
        let end = range.end;
        start + rng.f64() * (end - start)
    })
}

/// SplitMix64 mixing step — a standard, cheap, dependency-free mixer used to
/// derive deterministic sub-seeds from a tuple of coordinates.
fn splitmix64(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Derive a deterministic sub-seed from a root seed, a domain tag, and a list of
/// integer coordinates (e.g. file index, batch index, column index). Same inputs
/// always produce the same output and any coordinate change shifts the result.
pub fn derive_seed(root: u64, domain: u64, coords: &[u64]) -> u64 {
    let mut acc = splitmix64(root);
    acc = splitmix64(acc ^ splitmix64(domain));
    for &c in coords {
        acc = splitmix64(acc ^ splitmix64(c));
    }
    acc
}

/// RAII guard that installs a seeded RNG on the current thread. When dropped,
/// it clears the thread-local so nothing else on this thread accidentally
/// reuses the installed RNG.
pub struct RngScope {
    _private: (),
}

impl Drop for RngScope {
    fn drop(&mut self) {
        RNG.with(|rng| *rng.borrow_mut() = None);
    }
}

/// Install a fresh `Rng::with_seed(seed)` on the current thread. The guard
/// returned must be kept alive for the duration of the work that should draw
/// from this seeded stream. Panics if a scope is already active on this thread
/// to prevent accidental nesting (which would silently shadow the outer seed).
///
/// **Important invariant**: the thread-local must be `None` when this is called.
/// `with_rng` auto-initializes a random RNG on first use if `None`, which would
/// then cause `scoped_seeded` to panic. Callers must ensure no rng helpers are
/// called between a scope drop and the next `scoped_seeded` on the same thread.
pub fn scoped_seeded(seed: u64) -> RngScope {
    RNG.with(|rng| {
        let mut rng_ref = rng.borrow_mut();
        assert!(
            rng_ref.is_none(),
            "scoped_seeded called while an RngScope is already active on this thread"
        );
        *rng_ref = Some(Rng::with_seed(seed));
    });
    RngScope { _private: () }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_with_seed() {
        // Test that same seed produces same results
        initialize_rng(Some(12345));
        let results1: Vec<i32> = (0..10).map(|_| i32(0..100)).collect();

        initialize_rng(Some(12345));
        let results2: Vec<i32> = (0..10).map(|_| i32(0..100)).collect();

        assert_eq!(results1, results2);
    }

    #[test]
    fn test_different_seeds_produce_different_results() {
        initialize_rng(Some(12345));
        let results1: Vec<i32> = (0..10).map(|_| i32(0..100)).collect();

        initialize_rng(Some(54321));
        let results2: Vec<i32> = (0..10).map(|_| i32(0..100)).collect();

        assert_ne!(results1, results2);
    }

    #[test]
    fn test_no_seed_works() {
        initialize_rng(None);
        let result = i32(0..100);
        assert!((0..100).contains(&result));
    }

    #[test]
    fn derive_seed_is_pure() {
        assert_eq!(
            derive_seed(42, DOMAIN_PROVIDER, &[1, 2, 3]),
            derive_seed(42, DOMAIN_PROVIDER, &[1, 2, 3])
        );
    }

    #[test]
    fn derive_seed_varies_with_root() {
        assert_ne!(
            derive_seed(42, DOMAIN_PROVIDER, &[1, 2, 3]),
            derive_seed(43, DOMAIN_PROVIDER, &[1, 2, 3])
        );
    }

    #[test]
    fn derive_seed_varies_with_domain() {
        assert_ne!(
            derive_seed(42, 1, &[1, 2, 3]),
            derive_seed(42, 2, &[1, 2, 3])
        );
    }

    #[test]
    fn derive_seed_varies_with_any_coord() {
        let base = derive_seed(42, DOMAIN_PROVIDER, &[1, 2, 3]);
        assert_ne!(base, derive_seed(42, DOMAIN_PROVIDER, &[0, 2, 3]));
        assert_ne!(base, derive_seed(42, DOMAIN_PROVIDER, &[1, 0, 3]));
        assert_ne!(base, derive_seed(42, DOMAIN_PROVIDER, &[1, 2, 0]));
    }

    #[test]
    fn scoped_seeded_clears_on_drop() {
        // Make sure the thread-local is clean to start.
        RNG.with(|rng| *rng.borrow_mut() = None);
        {
            let _scope = scoped_seeded(99);
            let _ = i32(0..10);
        }
        RNG.with(|rng| {
            assert!(
                rng.borrow().is_none(),
                "RngScope drop should clear the thread-local"
            );
        });
    }

    #[test]
    fn scoped_seeded_is_deterministic_across_threads() {
        let seed = 0xDEAD_BEEF_u64;
        let worker = move || {
            let _scope = scoped_seeded(seed);
            (0..20).map(|_| u32(0..1_000_000)).collect::<Vec<_>>()
        };

        let t1 = std::thread::spawn(worker);
        let t2 = std::thread::spawn(worker);

        assert_eq!(t1.join().unwrap(), t2.join().unwrap());
    }

    #[test]
    fn scoped_seeded_panics_on_nesting() {
        // Run the nesting assertion on a fresh worker thread so prior tests on
        // this thread can't have left the thread-local populated.
        let joined = std::thread::spawn(|| {
            let _outer = scoped_seeded(1);
            let _inner = scoped_seeded(2);
        })
        .join();

        let payload = joined.expect_err("nested scoped_seeded should panic");
        let msg = payload
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| payload.downcast_ref::<&'static str>().copied())
            .unwrap_or("");
        assert!(
            msg.contains("scoped_seeded called while an RngScope is already active"),
            "unexpected panic message: {msg}"
        );
    }
}
