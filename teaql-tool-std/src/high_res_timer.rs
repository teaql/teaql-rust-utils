#[derive(Debug, Clone)]
pub struct HighResTimerTool;

impl HighResTimerTool {
    pub fn new() -> Self {
        Self
    }

    /// 启动一个高精度计时器
    pub fn start(&self) -> HighResTimer {
        HighResTimer::start()
    }
}

impl Default for HighResTimerTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(windows))]
mod imp {
    use std::sync::OnceLock;

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    use core::arch::x86_64::{_mm_lfence, _rdtsc};

    #[derive(Debug, Clone)]
    pub struct HighResTimer {
        start_cycles: u64,
    }

    static TICK_HZ: OnceLock<u64> = OnceLock::new();

    #[inline(always)]
    fn global_tick_hz() -> u64 {
        *TICK_HZ.get_or_init(|| calibrate_tick_hz())
    }

    impl HighResTimer {
        pub fn start() -> Self {
            let _ = global_tick_hz();
            Self { start_cycles: Self::get_ticks() }
        }

        #[inline(always)]
        fn get_ticks() -> u64 {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            unsafe {
                _mm_lfence();
                let t = _rdtsc();
                _mm_lfence();
                return t;
            }

            #[cfg(target_arch = "aarch64")]
            {
                let val: u64;
                unsafe {
                    core::arch::asm!("mrs {}, cntvct_el0", out(reg) val);
                }
                return val;
            }

            #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
            {
                0
            }
        }

        pub fn ns(&self) -> u64 {
            let end_ticks = Self::get_ticks();
            let delta = end_ticks.wrapping_sub(self.start_cycles) as u128;
            ((delta * 1_000_000_000u128) / global_tick_hz() as u128) as u64
        }
    }

    fn calibrate_tick_hz() -> u64 {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            return calibrate_tsc_with_monotonic();
        }

        #[cfg(target_arch = "aarch64")]
        {
            return read_cntfrq_el0();
        }
        
        #[allow(unreachable_code)]
        2_500_000_000
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn calibrate_tsc_with_monotonic() -> u64 {
        use libc::{clock_gettime, timespec, CLOCK_MONOTONIC};

        unsafe {
            let mut ts_start = timespec { tv_sec: 0, tv_nsec: 0 };
            let mut ts_end = timespec { tv_sec: 0, tv_nsec: 0 };

            clock_gettime(CLOCK_MONOTONIC, &mut ts_start);
            _mm_lfence();
            let tsc_start = _rdtsc();
            _mm_lfence();

            spin_wait_ns(10_000_000); // ~10ms

            clock_gettime(CLOCK_MONOTONIC, &mut ts_end);
            _mm_lfence();
            let tsc_end = _rdtsc();
            _mm_lfence();

            let delta_tsc = tsc_end - tsc_start;
            let delta_ns = (ts_end.tv_sec - ts_start.tv_sec) as u128 * 1_000_000_000u128
                + (ts_end.tv_nsec - ts_start.tv_nsec) as u128;

            (delta_tsc as u128 * 1_000_000_000u128 / delta_ns) as u64
        }
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[inline(always)]
    fn spin_wait_ns(ns: u64) {
        use libc::{clock_gettime, timespec, CLOCK_MONOTONIC};

        let start = unsafe {
            let mut ts = timespec { tv_sec: 0, tv_nsec: 0 };
            clock_gettime(CLOCK_MONOTONIC, &mut ts);
            ts.tv_sec as u128 * 1_000_000_000u128 + ts.tv_nsec as u128
        };

        loop {
            let now = unsafe {
                let mut ts = timespec { tv_sec: 0, tv_nsec: 0 };
                clock_gettime(CLOCK_MONOTONIC, &mut ts);
                ts.tv_sec as u128 * 1_000_000_000u128 + ts.tv_nsec as u128
            };
            if now - start >= ns as u128 {
                break;
            }
        }
    }

    #[cfg(target_arch = "aarch64")]
    #[inline(always)]
    fn read_cntfrq_el0() -> u64 {
        let freq: u64;
        unsafe {
            core::arch::asm!("mrs {}, cntfrq_el0", out(reg) freq);
        }
        freq
    }
}

#[cfg(windows)]
mod imp {
    use std::time::Instant;

    #[derive(Debug, Clone)]
    pub struct HighResTimer {
        start_time: Instant,
    }

    impl HighResTimer {
        pub fn start() -> Self {
            Self { start_time: Instant::now() }
        }

        pub fn ns(&self) -> u64 {
            self.start_time.elapsed().as_nanos() as u64
        }
    }
}

pub use imp::HighResTimer;

impl HighResTimer {
    #[inline(always)]
    pub fn us(&self) -> u64 {
        self.ns() / 1_000
    }

    #[inline(always)]
    pub fn ms(&self) -> u64 {
        self.ns() / 1_000_000
    }
}
