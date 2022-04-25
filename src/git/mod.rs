mod add;
mod check;
mod commit;
mod push;

pub use self::add::stage_all;
pub use self::check::check_git;
pub use self::check::check_branch;
pub use self::check::check_remote;
pub use self::commit::commit;
pub use self::push::push;
