use menu::{MainMenu, TagsMenu};
use std_in_out::press_enter_to_continue;

mod menu;
mod registry;
mod std_in_out;

fn main() {
    let mut scanner = registry::RegistryScanner::new();

    press_enter_to_continue("You area about to perform a full scan of the registry, this operation can can take several seconds...");
    let scan_result = scanner.scan();

    loop {
        MainMenu::print(&scan_result);
        let selected = MainMenu::select(&scan_result);
        match selected {
            Some(repo) => TagsMenu::print(&repo),
            None => (),
        }
    }
}
