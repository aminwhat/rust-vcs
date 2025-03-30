use add::add_file;
use checkout::checkout;
use commit::commit_changes;
use init::init_repo;
use log::show_log;

mod add;
mod checkout;
mod commit;
mod init;
mod log;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "init" {
        init_repo();
    } else if args.len() > 2 && args[1] == "add" {
        add_file(&args[2]);
    } else if args.len() > 2 && args[1] == "commit" {
        commit_changes(&args[2]);
    } else if args.len() > 1 && args[1] == "log" {
        show_log();
    } else if args.len() > 2 && args[1] == "checkout" {
        checkout(&args[2]);
    }
}
