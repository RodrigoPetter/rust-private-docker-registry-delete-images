use main_menu::{MainMenu};
use std_in_out::press_enter_to_continue;

mod main_menu;
mod registry;
mod std_in_out;

fn main() {
    let mut scanner = registry::RegistryScanner::new();

    press_enter_to_continue("You area about to perform a full scan of the registry, this operation can can take several seconds...");
    let mut scan_result = scanner.scan();

    MainMenu::open(&mut scan_result);
}
