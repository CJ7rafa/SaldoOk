pub mod suppliers;
pub mod purchases;
pub mod purchase_payments;

// Creas una función que agrupa todo
pub fn get_handlers() -> impl Fn(tauri::ipc::Invoke) -> bool {
    tauri::generate_handler![
        // Suppliers
        suppliers::get_suppliers,
        suppliers::create_supplier,
        suppliers::update_supplier,
        suppliers::delete_supplier,
        // Purchases
        purchases::get_purchases,
        purchases::create_purchase,
        purchases::update_purchase,
        purchases::delete_purchase,
        // Purchase Payments
        purchase_payments::get_purchase_payments,
        purchase_payments::create_purchase_payment,
        purchase_payments::update_purchase_payment,
        purchase_payments::delete_purchase_payment
    ]
}
