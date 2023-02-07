use std::sync::{LockResult, RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError, TryLockResult};
use std::thread;
use std::time::{Duration, Instant};

// newtype pattern
pub struct RwLockWrapped<T: ?Sized>(RwLock<T>);

impl<T> RwLockWrapped<T> {
    pub fn new(t: T) -> RwLockWrapped<T> {
        // info!("SETUP RWLOCK WRAPPER");
        println!("SETUP RWLOCK WRAPPER");
        RwLockWrapped(RwLock::new(t))
    }

    pub fn write(&self) -> LockResult<RwLockWriteGuard<'_, T>> {
        println!("enter WRITE");
        info!("enter WRITE");

        write_smart(&self.0)
    }

    pub fn try_read(&self) -> TryLockResult<RwLockReadGuard<'_, T>> {
        println!("enter TRYWRITE");
        info!("enter TRYWRITE");

        self.0.try_read()
    }

    pub fn read(&self) -> LockResult<RwLockReadGuard<'_, T>> {
        println!("enter READ");
        info!("enter READ");

        self.0.read()
    }
}

impl<T: Default> Default for RwLockWrapped<T> {
    /// Creates a new `RwLockWrapped<T>`, with the `Default` value for T.
    fn default() -> RwLockWrapped<T> {
        RwLockWrapped::new(Default::default())
    }
}



fn write_smart<T: ?Sized>(rwlock: &RwLock<T>) -> LockResult<RwLockWriteGuard<'_, T>> {
    println!("ENTER WRITELOCK");
    info!("ENTER WRITELOCK");
    let mut cnt: u64 = 0;
    // consider using SystemTime here
    let wait_since = Instant::now();
    loop {
        match rwlock.try_write() {
            Ok(guard) => {
                return LockResult::Ok(guard);
            }
            Err(err) => {
                match err {
                    TryLockError::Poisoned(poison) => {
                        return LockResult::Err(poison);
                    }
                    TryLockError::WouldBlock => {
                        let waittime_elapsed = wait_since.elapsed();

                        // dispatch to custom handle
                        // note: implementation must deal with debounce, etc.
                        handle_block_event(wait_since, waittime_elapsed);

                        sleep_backoff(cnt);
                        cnt += 1;
                    }
                }
            }
        }
    }
}

// custom handling
fn handle_block_event(since: Instant, elapsed: Duration) {
    println!("LOOP {:?}, elapsed {:?}", since, elapsed);
    info!("LOOP {:?}, elapsed {:?}", since, elapsed);
    // LOGGER.log();
}

const SAMPLING_RATE_STAGE1: Duration = Duration::from_micros(100);
const SAMPLING_RATE_STAGE2: Duration = Duration::from_millis(10);
const SAMPLING_RATE_STAGE3: Duration = Duration::from_millis(100);

fn sleep_backoff(cnt: u64) {
    if cnt < 100 {
        thread::sleep(SAMPLING_RATE_STAGE1);
        return;
    } else if cnt < 500 {
        thread::sleep(SAMPLING_RATE_STAGE2);
        return;
    } else {
        thread::sleep(SAMPLING_RATE_STAGE3);
    }
}
