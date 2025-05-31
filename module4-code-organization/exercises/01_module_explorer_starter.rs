// TODO: Define your module structure here
// Hint: You'll need to create modules for products, users, orders, and inventory

mod product {
    #[derive(Debug, Clone)]
    pub struct Product {
        pub id: u32,
        pub name: String,
        pub price: f64,
        pub description: String,
    }

    impl Product {
        pub fn new(id: u32, name: &str, price: f64, description: &str) -> Product {
            Product {
                id,
                name: name.to_string(),
                price,
                description: description.to_string(),
            }
        }

        pub fn display(&self) {
            println!("Product: {} (ID: {})", self.name, self.id);
            println!("Price: ${:.2}", self.price);
            println!("Description: {}", self.description);
        }
    }
}

mod user {
    #[derive(Debug)]
    pub struct User {
        pub id: u32,
        pub name: String,
        pub email: String,
        pub address: String,
    }

    impl User {
        pub fn new(id: u32, name: &str, email: &str, address: &str) -> User {
            User {
                id,
                name: name.to_string(),
                email: email.to_string(),
                address: address.to_string(),
            }
        }

        pub fn display(&self) {
            println!("User: {} (ID: {})", self.name, self.id);
            println!("Email: {}", self.email);
            println!("Address: {}", self.address);
        }
    }
}

mod order {
    use crate::product::Product;
    use crate::user::User;

    #[derive(Debug)]
    pub enum OrderStatus {
        Pending,
        Processing,
        Shipped,
        Delivered,
        Cancelled,
    }

    #[derive(Debug)]
    pub struct OrderItem {
        pub product: Product,
        pub quantity: u32,
    }

    #[derive(Debug)]
    pub struct Order {
        pub id: u32,
        pub user: User,
        pub items: Vec<OrderItem>,
        pub status: OrderStatus,
        pub total: f64,
    }

    impl Order {
        pub fn new(id: u32, user: User) -> Order {
            Order {
                id,
                user,
                items: Vec::new(),
                status: OrderStatus::Pending,
                total: 0.0,
            }
        }

        pub fn add_item(&mut self, product: Product, quantity: u32) {
            let item = OrderItem { product, quantity };
            self.items.push(item);
            self.update_total();
        }

        fn update_total(&mut self) {
            self.total = self.items.iter()
                .map(|item| item.product.price * item.quantity as f64)
                .sum();
        }

        pub fn display(&self) {
            println!("\nOrder #{}", self.id);
            println!("Status: {:?}", self.status);
            println!("\nCustomer:");
            self.user.display();
            
            println!("\nItems:");
            for item in &self.items {
                println!("- {} x{} (${:.2} each)", 
                    item.product.name, 
                    item.quantity, 
                    item.product.price
                );
            }
            println!("\nTotal: ${:.2}", self.total);
        }
    }
}

mod inventory {
    use std::collections::HashMap;
    use crate::product::Product;

    pub struct Inventory {
        products: HashMap<u32, (Product, u32)>, // (Product, quantity)
    }

    impl Inventory {
        pub fn new() -> Inventory {
            Inventory {
                products: HashMap::new(),
            }
        }

        pub fn add_product(&mut self, product: Product, quantity: u32) {
            self.products.insert(product.id, (product, quantity));
        }

        pub fn remove_product(&mut self, product_id: u32) -> Option<Product> {
            self.products.remove(&product_id).map(|(product, _)| product)
        }

        pub fn get_stock(&self, product_id: u32) -> Option<u32> {
            self.products.get(&product_id).map(|(_, quantity)| *quantity)
        }

        pub fn update_stock(&mut self, product_id: u32, quantity: u32) -> bool {
            if let Some((_, current_quantity)) = self.products.get_mut(&product_id) {
                *current_quantity = quantity;
                true
            } else {
                false
            }
        }

        pub fn display(&self) {
            println!("\nInventory Status:");
            for (_, (product, quantity)) in &self.products {
                println!("{} (ID: {}): {} in stock", 
                    product.name, 
                    product.id, 
                    quantity
                );
            }
        }
    }
}

use product::Product;
use user::User;
use order::{Order, OrderStatus};
use inventory::Inventory;

fn main() {
    // 1. Create some products
    let laptop = Product::new(1, "Laptop", 999.99, "High-performance laptop");
    let mouse = Product::new(2, "Mouse", 29.99, "Wireless mouse");
    let keyboard = Product::new(3, "Keyboard", 59.99, "Mechanical keyboard");

    // 2. Add products to inventory
    let mut inventory = Inventory::new();
    inventory.add_product(laptop.clone(), 10);
    inventory.add_product(mouse.clone(), 20);
    inventory.add_product(keyboard.clone(), 15);

    // Display initial inventory
    inventory.display();

    // 3. Create a user
    let user = User::new(1, "John Doe", "john@example.com", "123 Main St");

    // 4. Create an order for the user
    let mut order = Order::new(1, user);
    order.add_item(laptop, 1);
    order.add_item(mouse, 2);
    order.add_item(keyboard, 1);

    // 5. Print order details
    order.display();

    // Update inventory after order
    inventory.update_stock(1, 9);  // Laptop
    inventory.update_stock(2, 18); // Mouse
    inventory.update_stock(3, 14); // Keyboard

    // Display updated inventory
    inventory.display();
}