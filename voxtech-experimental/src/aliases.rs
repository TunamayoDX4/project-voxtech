/* --- std::error::Error関連 --- */
pub type StdError = Box<dyn std::error::Error>;
pub type StdResult<T> = Result<T, StdError>;

/* --- parking_lot::Mutex関連 --- */
pub type PMutex<T> = parking_lot::Mutex<T>;
pub type PMutexGuard<'a, T> = parking_lot::MutexGuard<'a, T>;
pub type MappedPMutexGuard<'a, T> =
  parking_lot::MappedMutexGuard<'a, T>;

/* --- parking_lot::FairMutex関連 --- */
pub type PFairMutex<T> = parking_lot::FairMutex<T>;
pub type PFairMutexGuard<'a, T> =
  parking_lot::FairMutexGuard<'a, T>;
pub type MappedPFairMutexGuard<'a, T> =
  parking_lot::MappedFairMutexGuard<'a, T>;

/* --- parking_lot::RwLock関連 --- */
pub type PRwLock<T> = parking_lot::RwLock<T>;
pub type PRwLockReadGuard<'a, T> =
  parking_lot::RwLockReadGuard<'a, T>;
pub type MappedPRwLockReadGuard<'a, T> =
  parking_lot::MappedRwLockReadGuard<'a, T>;
pub type PRwLockUpgradableReadGuard<'a, T> =
  parking_lot::RwLockUpgradableReadGuard<'a, T>;
pub type PRwLockWriteGuard<'a, T> =
  parking_lot::RwLockWriteGuard<'a, T>;
pub type MappedPRwLockWriteGuard<'a, T> =
  parking_lot::MappedRwLockWriteGuard<'a, T>;
