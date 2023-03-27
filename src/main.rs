use main_menu::MainMenu;
use std_in_out::press_enter_to_continue;

mod main_menu;
mod registry;
mod std_in_out;
mod sizes;

fn main() {
    let mut scanner = registry::RegistryScanner::new();

    press_enter_to_continue("You area about to perform a full scan of the registry, this operation can can take several seconds...");
    let mut scan_result = scanner.scan();

    loop {
        let sizes = sizes::calculate(&scan_result);

        //Sort by size
        scan_result.sort_by(|a, b| {
            sizes[&b.repository].size_dedup_global
                .partial_cmp(&sizes[&a.repository].size_dedup_global)
                .unwrap()
        });
        MainMenu::open(&mut scan_result);
    }
}
