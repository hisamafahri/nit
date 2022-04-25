mod add;
mod check;
mod commit;
mod error;

pub use self::add::stage_all;
pub use self::check::check_git;
pub use self::commit::commit;
