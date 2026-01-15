use cursive::traits::{Nameable, Resizable};
use cursive::views::{Dialog, EditView, ListView, SelectView};
use cursive::{Cursive, CursiveExt};

use std::fs::{File, OpenOptions};
use std::io::{self, Read};
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Product {
    product_type: String,
    quantity: usize,
    price_per_unit: f64,
    sales_tax: f64,
    total_price: f64,
}

const FILE_PATH: &str = "inventory.json";

fn save_products_to_file(products: &Vec<Product>) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_PATH)?;

    serde_json::to_writer_pretty(file, products)?;
    Ok(())
}

fn load_products_from_file() -> Vec<Product> {
    if let Ok(mut file) = File::open(FILE_PATH) {
        let mut data = String::new();
        if file.read_to_string(&mut data).is_ok() {
            if let Ok(products) = serde_json::from_str(&data) {
                return products;
            }
        }
    }
    // return empty vector if file does not exist or if there is an error
    Vec::new()
}

fn main() {
    let mut siv = Cursive::default();

    let products = Arc::new(Mutex::new(load_products_from_file()));

    // Add a dialog layer to the UI for managing the inventory.
    siv.add_layer(
        Dialog::new() // Create a new dialog.
            .title("Inventory Management") // Set the dialog title.
            .content(
                ListView::new() // Set the dialog content to a new ListView.
                    .child("Product Type:", EditView::new().with_name("product_type")) // Add an EditView for entering the product type.
                    .child("Quantity:", EditView::new().with_name("quantity")) // Add an EditView for entering the quantity.
                    .child(
                        "Price per Unit:",
                        EditView::new().with_name("price_per_unit"),
                    ), // Add an EditView for entering the price per unit.
            )
            .button("Save", {
                // Add a button to save the product.
                let products_clone = Arc::clone(&products); // Clone the Arc for thread-safe access.
                move |s| {
                    // Closure that runs when the button is pressed.
                    let product_type = s // Get the content from the EditView named "product_type".
                        .call_on_name("product_type", |view: &mut EditView| {
                            view.get_content()
                        })
                        .unwrap() // Unwrap the result, panicking if there’s an error.
                        .to_string(); // Convert the content to a String.

                    let quantity = s // Get the content from the EditView named "quantity".
                        .call_on_name("quantity", |view: &mut EditView| {
                            view.get_content()
                        })
                        .unwrap() // Unwrap the result.
                        .parse::<usize>() // Parse the content as usize.
                        .unwrap_or(0); // If parsing fails, default to 0.

                    let price_per_unit = s // Get the content from the EditView named "price_per_unit".
                        .call_on_name("price_per_unit", |view: &mut EditView| {
                            view.get_content()
                        })
                        .unwrap() // Unwrap the result.
                        .parse::<f64>() // Parse the content as f64.
                        .unwrap_or(0.0); // If parsing fails, default to 0.0.

                    // Validation: Check if the fields are empty or invalid.
                    if product_type.is_empty() {
                        // Check if the product type is empty.
                        s.add_layer(Dialog::info("Error: Please enter a product type.")); // Show an error dialog.
                        return; // Exit the closure.
                    }

                    if quantity == 0 {
                        // Check if the quantity is invalid.
                        s.add_layer(Dialog::info("Error: Please enter a valid quantity.")); // Show an error dialog.
                        return; // Exit the closure.
                    }

                    if price_per_unit == 0.0 {
                        // Check if the price per unit is invalid.
                        s.add_layer(Dialog::info("Error: Please enter a valid price.")); // Show an error dialog.
                        return; // Exit the closure.
                    }

                    let sales_tax = 0.10 * price_per_unit; // Calculate sales tax at a rate of 10%.
                    let total_price = (price_per_unit + sales_tax) * quantity as f64; // Calculate total price.

                    let product = Product {
                        // Create a new Product instance.
                        product_type,
                        quantity,
                        price_per_unit,
                        sales_tax,
                        total_price,
                    };

                    let mut product_store = products_clone.lock().unwrap(); // Lock the Mutex to safely access the products.
                    product_store.push(product.clone()); // Add the new product to the product store.

                    // Save to file
                    if let Err(err) = save_products_to_file(&product_store) {
                        // Try to save the products to file.
                        s.add_layer(Dialog::info(format!("Error saving product: {}", err))); // Show an error dialog if saving fails.
                    } else {
                        s.add_layer(Dialog::info("Product saved successfully!")); // Show a success dialog.
                    }
                }
            })
            .button("Show All", {
                // Add a button to show all products.
                let products = Arc::clone(&products); // Clone the Arc for thread-safe access.
                move |s| {
                    // Closure that runs when the button is pressed.
                    let product_store = products.lock().unwrap(); // Lock the Mutex to access the products.
                    let mut output = String::new(); // Create a string to hold the output.

                    for (index, product) in product_store.iter().enumerate() {
                        // Iterate through each product.
                        output.push_str(&format!(
                            // Format the product details into the output string.
                            "{}. Item: {}, Qty: {}, Price: ${}, Sales Tax: ${}, T.Price: ${}\n",
                            index + 1,              // Product index (1-based).
                            product.product_type,   // Product type.
                            product.quantity,       // Quantity.
                            product.price_per_unit, // Price per unit.
                            product.sales_tax,      // Sales tax.
                            product.total_price     // Total price.
                        ));
                    }

                    if output.is_empty() {
                        // Check if there are no products.
                        output = "No products in the inventory.".to_string(); // Set a message if there are no products.
                    }

                    s.add_layer(Dialog::info(output)); // Show the output in a dialog.
                }
            })
            .button("Delete by ID", {
                // Add a button to delete a product by ID.
                let products_clone = Arc::clone(&products); // Clone the Arc for thread-safe access.
                move |s| {
                    // Closure that runs when the button is pressed.
                    // Get ID from user
                    let id_input = EditView::new().with_name("delete_id").min_width(10); // Create an EditView for entering the product ID.
                    s.add_layer(Dialog::new() // Create a new dialog for deleting a product.
                        .title("Delete Product") // Set the dialog title.
                        .content(ListView::new() // Set the content of the dialog.
                            .child("Enter product ID to delete:", id_input) // Add the ID input field.
                        )
                        .button("Confirm", { // Add a button to confirm deletion.
                            let products_clone = Arc::clone(&products_clone); // Clone the Arc for thread-safe access.
                            move |s: &mut Cursive| { // Closure that runs when the button is pressed.
                                let id_str = s // Get the content from the EditView named "delete_id".
                                    .call_on_name("delete_id", |view: &mut EditView| {
                                        view.get_content()
                                    })
                                    .unwrap() // Unwrap the result.
                                    .to_string(); // Convert the content to a String.

                                // Parse ID
                                if let Ok(id) = id_str.parse::<usize>() { // Try to parse the ID as usize.
                                    let mut product_store = products_clone.lock().unwrap(); // Lock the Mutex to access the products.

                                    // Check if ID is valid
                                    if id > 0 && id <= product_store.len() { // Check if the ID is within the valid range.
                                        product_store.remove(id - 1); // Remove the product from the store (adjusting for 0-based index).
                                        if let Err(err) = save_products_to_file(&product_store) { // Try to save the updated products to file.
                                            s.add_layer(Dialog::info(format!("Error deleting product: {}", err))); // Show an error dialog if saving fails.
                                        } else {
                                            s.add_layer(Dialog::info("Product deleted successfully!")); // Show a success dialog.
                                        }
                                    } else {
                                        s.add_layer(Dialog::info("Error: Invalid product ID.")); // Show an error dialog if the ID is invalid.
                                    }
                                } else {
                                    s.add_layer(Dialog::info("Error: Please enter a valid number.")); // Show an error dialog if the ID is not a valid number.
                                }
                            }
                        })
                        .button("Cancel", |s| { // Add a button to cancel the deletion.
                            s.pop_layer(); // Remove the delete dialog layer.
                        })
                    );
                }
            })
            .button("Quit", |s| s.quit()), // Add a button to quit the application.
    );

    siv.run();
}
