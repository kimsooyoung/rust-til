//! Cursive TUI inventory app.
//!
//! Provides a small terminal UI for adding, listing, and deleting products
//! in a JSON-backed inventory file. The entry point is [`run`], which is
//! called by `main.rs` after setting up the terminal.

use std::fs::{File, OpenOptions};
use std::io::Read;
use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};
use cursive::traits::{Nameable, Resizable};
use cursive::views::{Dialog, EditView, ListView};
use cursive::{Cursive, CursiveExt};
use serde::{Deserialize, Serialize};

const FILE_PATH: &str = "inventory.json";
const SALES_TAX_RATE: f64 = 0.10;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Product {
    product_type: String,
    quantity: usize,
    price_per_unit: f64,
    sales_tax: f64,
    total_price: f64,
}

/// Run the inventory TUI event loop until the user quits.
pub fn run() -> Result<()> {
    let mut siv = Cursive::default();
    let products = Arc::new(Mutex::new(load_products_from_file()));

    siv.add_layer(build_main_dialog(Arc::clone(&products)));
    siv.run();
    Ok(())
}

fn build_main_dialog(products: Arc<Mutex<Vec<Product>>>) -> Dialog {
    Dialog::new()
        .title("Inventory Management")
        .content(
            ListView::new()
                .child("Product Type:", EditView::new().with_name("product_type"))
                .child("Quantity:", EditView::new().with_name("quantity"))
                .child(
                    "Price per Unit:",
                    EditView::new().with_name("price_per_unit"),
                ),
        )
        .button("Save", {
            let products = Arc::clone(&products);
            move |s| handle_save(s, &products)
        })
        .button("Show All", {
            let products = Arc::clone(&products);
            move |s| handle_show_all(s, &products)
        })
        .button("Delete by ID", {
            let products = Arc::clone(&products);
            move |s| handle_delete(s, &products)
        })
        .button("Quit", |s| s.quit())
}

fn handle_save(s: &mut Cursive, products: &Arc<Mutex<Vec<Product>>>) {
    let Some(product_type) = read_field(s, "product_type") else {
        return;
    };
    let quantity = read_field(s, "quantity")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    let price_per_unit = read_field(s, "price_per_unit")
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(0.0);

    if product_type.is_empty() {
        s.add_layer(Dialog::info("Error: Please enter a product type."));
        return;
    }
    if quantity == 0 {
        s.add_layer(Dialog::info("Error: Please enter a valid quantity."));
        return;
    }
    if price_per_unit == 0.0 {
        s.add_layer(Dialog::info("Error: Please enter a valid price."));
        return;
    }

    let sales_tax = SALES_TAX_RATE * price_per_unit;
    let total_price = (price_per_unit + sales_tax) * quantity as f64;
    let product = Product {
        product_type,
        quantity,
        price_per_unit,
        sales_tax,
        total_price,
    };

    let mut store = match products.lock() {
        Ok(guard) => guard,
        Err(_) => {
            s.add_layer(Dialog::info("Error: inventory state was poisoned."));
            return;
        }
    };
    store.push(product);
    match save_products_to_file(&store) {
        Ok(()) => s.add_layer(Dialog::info("Product saved successfully!")),
        Err(err) => s.add_layer(Dialog::info(format!("Error saving product: {err}"))),
    }
}

fn handle_show_all(s: &mut Cursive, products: &Arc<Mutex<Vec<Product>>>) {
    let store = match products.lock() {
        Ok(guard) => guard,
        Err(_) => {
            s.add_layer(Dialog::info("Error: inventory state was poisoned."));
            return;
        }
    };
    let mut output = String::new();
    for (index, product) in store.iter().enumerate() {
        output.push_str(&format!(
            "{}. Item: {}, Qty: {}, Price: ${}, Sales Tax: ${}, T.Price: ${}\n",
            index + 1,
            product.product_type,
            product.quantity,
            product.price_per_unit,
            product.sales_tax,
            product.total_price,
        ));
    }
    if output.is_empty() {
        output = "No products in the inventory.".to_string();
    }
    s.add_layer(Dialog::info(output));
}

fn handle_delete(s: &mut Cursive, products: &Arc<Mutex<Vec<Product>>>) {
    let id_input = EditView::new().with_name("delete_id").min_width(10);
    let products = Arc::clone(products);
    s.add_layer(
        Dialog::new()
            .title("Delete Product")
            .content(ListView::new().child("Enter product ID to delete:", id_input))
            .button("Confirm", move |s| {
                let id_str = read_field(s, "delete_id").unwrap_or_default();
                let Ok(id) = id_str.parse::<usize>() else {
                    s.add_layer(Dialog::info("Error: Please enter a valid number."));
                    return;
                };
                let mut store = match products.lock() {
                    Ok(guard) => guard,
                    Err(_) => {
                        s.add_layer(Dialog::info("Error: inventory state was poisoned."));
                        return;
                    }
                };
                if id == 0 || id > store.len() {
                    s.add_layer(Dialog::info("Error: Invalid product ID."));
                    return;
                }
                store.remove(id - 1);
                match save_products_to_file(&store) {
                    Ok(()) => s.add_layer(Dialog::info("Product deleted successfully!")),
                    Err(err) => {
                        s.add_layer(Dialog::info(format!("Error deleting product: {err}")));
                    }
                }
            })
            .button("Cancel", |s| {
                s.pop_layer();
            }),
    );
}

fn read_field(s: &mut Cursive, name: &str) -> Option<String> {
    s.call_on_name(name, |view: &mut EditView| view.get_content())
        .map(|content| content.to_string())
}

fn save_products_to_file(products: &[Product]) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_PATH)
        .with_context(|| format!("opening {FILE_PATH} for writing"))?;
    serde_json::to_writer_pretty(file, products)
        .with_context(|| format!("serializing inventory to {FILE_PATH}"))?;
    Ok(())
}

fn load_products_from_file() -> Vec<Product> {
    let Ok(mut file) = File::open(FILE_PATH) else {
        return Vec::new();
    };
    let mut data = String::new();
    if file.read_to_string(&mut data).is_err() {
        return Vec::new();
    }
    serde_json::from_str(&data).unwrap_or_default()
}
