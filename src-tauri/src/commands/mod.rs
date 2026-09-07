pub mod suppliers;
pub mod purchases;
pub mod purchase_payments;
pub mod spenders;
pub mod expenses_categories;
pub mod expenses;

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
        purchase_payments::delete_purchase_payment,

        // Spenders
        spenders::get_spenders,
        spenders::get_spender_by_id,
        spenders::create_spender,
        spenders::update_spender,
        spenders::delete_spender,

        // Expenses Categories
        expenses_categories::get_all_expense_categories,
        expenses_categories::get_expense_category_by_id,
        expenses_categories::create_expense_category,
        expenses_categories::update_expenses_categories,
        expenses_categories::delete_expenses_categories,

        // Expenses
        expenses::get_expenses,
        expenses::get_expense_by_id,
        expenses::create_expense,
        expenses::update_expense,
        expenses::delete_expense
    ]
}
