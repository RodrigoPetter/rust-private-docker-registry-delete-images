use menu::MainMenu;
use std_in_out::press_enter_to_continue;

mod std_in_out;
mod registry;
mod menu;

fn main() {   
    let mut scanner = registry::RegistryScanner::new();
    
    press_enter_to_continue("You area about to perform a full scan of the registry, this operation can can take several minutes...");
    let scan_result = scanner.scan();

    MainMenu::print(&scan_result);
    MainMenu::select(&scan_result);
}