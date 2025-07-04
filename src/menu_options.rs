use std::{collections::HashMap, io::Write};

#[derive(Debug, Clone)]
pub struct MenuOptions {
    food_name_count_map: HashMap<String, u8>,
}

impl MenuOptions {
    /// initializes a new MenuOptions instance
    pub fn new() -> Self {
        MenuOptions {
            food_name_count_map: HashMap::new(),
        }
    }

    /// prints the menu interface
    pub fn print_menu(&self) {
        println!("Menu Options");
        println!("1. Search");
        println!("2. Show Items");
        println!("3. Show Items (Alternative)");
        println!("4. Exit");
    }

    /// Prompts the user to input an item they wish to look for.
    ///
    /// ### Parameters
    /// -**`food_names`**: A list of food items (a vector of strings).
    ///
    /// ### Returns
    /// A numeric value representing the frequency of the specific word in the list.
    pub fn search_items(&self, food: &String) -> u8 {
        // finds a match and returns it
        if self.food_name_count_map.contains_key(food) {
            return self.food_name_count_map[food];
        } else {
            return 0;
        }
    }

    /// Print the list with numbers that represent the frequency of all items purchased.
    pub fn show_list(&self) {
        for (food_name, frequency) in &self.food_name_count_map {
            println!("{} {}", food_name, frequency);
        }
    }

    /// Print the list with asterisks that representing the frequency of all items purchased.
    pub fn show_list_alternative(&self) {
        // create a nwe variable to hold the asterisk
        let mut asterisk = String::new();
        // loop through the list
        for (food, freq) in &self.food_name_count_map {
            asterisk.clear(); // ensure the string is empty
            // use pair.second to mark the end of the second loop.
            for _ in 0..*freq {
                // add * into asterisk
                asterisk += "*";
            }
            // print the result
            println!("{food} {asterisk}");
        }
    }

    /// Prompts the user to input an item they wish to look for.
    ///
    /// ### Parameters
    /// -**`user_in`**:
    ///
    /// ### Returns
    /// true if user_in is a parsable u8 and within 1..4
    pub fn input_menu_filter(&self, user_in: &String) -> bool {
        // ensure that user_in is within the range of the menu items
        match user_in.parse::<u8>() {
            Ok(int) => {
                if int < 5 && int > 0 {
                    return true;
                } else {
                    return false;
                }
            }
            Err(_) => {
                // Parse the string to a unsigned int
                return false;
            }
        }
    }

    /// Populates the attribute food_name_count_map
    ///
    /// ### Parameters
    /// -**`food_names`**: list of food names
    /// ```
    ///  let menu = MenuOptions::new();
    ///
    ///  menu.populate_food_items(&f_items);
    /// ```
    pub fn populate_food_items(&mut self, food_names: &Vec<String>) {
        // loops through the list and inserts, (food, unsigned int)
        for food in food_names {
            self.food_name_count_map.insert(food.clone(), 0);
        }
    }

    /// count the amount of times a specific food is mentioned within the list
    ///
    /// ### Parameters
    /// -**`food_names`**: list of food names
    /// ```
    /// let menu = MenuOptions::new();
    ///
    ///  menu.count_food_items(&f_items);
    /// ```
    pub fn count_food_items(&mut self, food_names: &Vec<String>) {
        // loop through the content and count the repetitive words
        for (food, freq) in &mut self.food_name_count_map {
            // finds a match and increments
            for item in food_names {
                if food == item {
                    *freq += 1;
                }
            }
        }
    }

    /// ### Returns
    /// mutable reference to food_name_count_map
    pub fn get_food_items(&self) -> &HashMap<String, u8> {
        &self.food_name_count_map
    }

    /// python style input implementation
    pub fn input(&self, comment: &str) -> String {
        let mut input = String::new();
        print!("{comment}");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        return input.trim().to_owned();
    }
}
