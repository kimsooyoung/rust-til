use cursive::traits::{Nameable, Resizable};
use cursive::views::{Dialog, EditView, SelectView};
use cursive::{Cursive, CursiveExt};

use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Read};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Product {
    product_type: String,
    quantity: usize,
    price_per_unit: f64,
    sales_tax: f64,
    total_price: f64,
}

fn save_products_to_file(products: &Vec<Product>) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_PATH)?;

    serde_json::to_writer_pretty(file, products)?;
    Ok(())
}

const FILE_PATH: &str = "inventory.json";

fn load_products_from_file() -> Vec<Product> {
    if let Ok(mut file) = File::open(FILE_PATH) {
        let mut data = String::new();
        if file.read_to_string(&mut data).is_ok() {
            if let Ok(products) = serde_json::from_str(&data) {
                return products;
            }
        }
    }
    Vec::new()
}

fn main() {
    let mut siv = Cursive::default();
    siv.add_global_callback(cursive::event::Key::Esc, |s| s.quit());

    let products = load_products_from_file();
    let mut products_view = SelectView::new();

    // Add each product as a string representation to the SelectView
    for product in products {
        let product_str = format!(
            "{} - Qty: {}, Price: ${:.2}, Tax: ${:.2}, Total: ${:.2}",
            product.product_type,
            product.quantity,
            product.price_per_unit,
            product.sales_tax,
            product.total_price
        );
        products_view.add_item(product_str, ());
    }

    siv.add_layer(Dialog::around(products_view).title("Inventory System"));
    siv.run();
}
