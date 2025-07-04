mod menu_options;

use menu_options::MenuOptions;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut menu = MenuOptions::new();
    let file_foods = "../CS210_Project_Three_Input_File.txt";
    let file_frequency = "./frequency.dat";

    let f_content: Vec<_> = std::fs::read_to_string(file_foods)
        .expect("unable to open the file")
        .lines()
        .map(|line| line.to_owned())
        .collect();

    // populate the class attribute foodNameCountMap
    menu.populate_food_items(&f_content);
    menu.count_food_items(&f_content);

    /*
    Create a data file, with the naming convention frequency.dat,
    for backing up your accumulated data. The frequency.dat
    file should include every item (represented by a word)
    paired with the number of times that item appears in the input file.
    */
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_frequency)
        .unwrap();

    for (food, freq) in menu.get_food_items() {
        file.write(format!("{} {}\n", food, freq).as_bytes())
            .unwrap();
    }

    // present menu options
    let mut should_run = true;
    while should_run {
        // print menu items
        println!("\nWelcome to the Food Frequency Checker!");

        menu.print_menu();
        let user_in = &mut menu.input("Please choose a option: ");

        // filter unwanted/error pron text
        while !menu.input_menu_filter(user_in) {
            *user_in = menu.input("please input a valid number: ");
        }

        // handle user inp
        match user_in.parse::<u8>().unwrap() {
            1 => {
                let item_name =
                    menu.input("\nPlease input the item or word you wish to look for: ");
                println!("there are {} {item_name}", menu.search_items(&item_name))
            }
            2 => {
                println!();
                menu.show_list();
            }
            3 => {
                println!();
                menu.show_list_alternative();
            }
            4 => should_run = false,
            _ => {}
        }
    }
}
