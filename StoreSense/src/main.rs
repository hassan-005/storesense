use std::{io::{self, stdin}, str::FromStr};

fn read_integer<T>() -> T
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => {
                println!("Invalid input. Please enter an integer.");
                continue;
            }
        }
    }
}

#[derive(Debug)]
struct InventoryItem {
    id: u32,
    name: String,
    description: String,
    quantity: u32,
    price: f64,
}

struct Inventory {
    items: Vec<InventoryItem>,
}

impl Inventory {
    fn add_item(&mut self, item: InventoryItem) {
        self.items.push(item);
    }

    fn remove_item(&mut self, id: u32) {
        self.items.retain(|item| item.id != id);
    }

    fn update_quantity(&mut self, id: u32, new_quantity: u32) {
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            item.quantity = new_quantity;
        }
    }

    fn update_price(&mut self, id: u32, new_price: f64) {
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            item.price = new_price;
        }
    }

    fn display_inventory(&self) {
        println!("ID   |  QUANTITY  |    PRICE    |      NAME      |\t\t  DESCRIPTION\t\t|");
        for item in &self.items {
            println!("{}   |     {}     |    {}    |       {}       |    {}", item.id, item.quantity, item.price, item.name, item.description);
        }
    }
}

fn get_info() -> InventoryItem {
    println!("Enter the product id: ");
    let id: u32 = read_integer();
    
    println!("Enter the product name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to get input");
    
    println!("Enter the product description: ");
    let mut description = String::new();
    io::stdin().read_line(&mut description).expect("Failed to get input");
    
    println!("Enter product quantity: ");
    let quantity: u32 = read_integer();
    
    println!("Enter product price: ");
    let price: f64 = read_integer();

    InventoryItem {
        id,
        name: name.trim().to_string(),
        description: description.trim().to_string(),
        quantity,
        price,
    }
}

fn home_menu(store: &mut Inventory) {
    println!("******************************************************************************************************************************");
    println!("*\t\t\t\t\t\t\t Welcome to IMS\t\t\t\t\t\t\t     *");
    println!("******************************************************************************************************************************");
    
    loop {
        println!("\t\t\t\t\t\t **** HOME MENU **** ");
        println!("\t\t\t\t\t\t   (1) Manage Items  ");
        println!("\t\t\t\t\t\t   (2) Display Inventory");
        println!("Enter your choice: ");
        let choice: u32 = read_integer();
        
        match choice {
            1 => manage_items_menu(store),
            2 => store.display_inventory(),
            _ => println!("Wrong Choice"),
        }
    }
}

fn manage_items_menu(store: &mut Inventory) {
    loop {
        println!("\t\t\t\t\t\t **** ITEMS MENU **** ");
        println!("\t\t\t\t\t\t   (0) Go Back to Home Menu  ");
        println!("\t\t\t\t\t\t   (1) Add Item  ");
        println!("\t\t\t\t\t\t   (2) Remove Item");
        println!("\t\t\t\t\t\t   (3) Edit Price");
        println!("\t\t\t\t\t\t   (4) Edit Quantity");
        println!("Enter your choice: ");
        let choice: u32 = read_integer();
        
        match choice {
            0 => return,
            1 => {
                let product = get_info();
                store.add_item(product);
            },
            2 => {
                println!("Enter the product id to remove: ");
                let id: u32 = read_integer();
                store.remove_item(id);
            },
            3 => {
                println!("Enter the product id to edit price: ");
                let id: u32 = read_integer();
                println!("Enter the new price: ");
                let price: f64 = read_integer();
                store.update_price(id, price);
            },
            4 => {
                println!("Enter the product id to edit quantity: ");
                let id: u32 = read_integer();
                println!("Enter the new quantity: ");
                let quantity: u32 = read_integer();
                store.update_quantity(id, quantity);
            },
            _ => println!("Wrong Choice"),
        }
    }
}

fn main() {
    let mut inventory = Inventory { items: Vec::new() };
    home_menu(&mut inventory);
}
