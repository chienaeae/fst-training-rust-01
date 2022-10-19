mod persist;

pub use self::persist::{error::Error as PersistError, CardPersistService, DefaultPersistService};
