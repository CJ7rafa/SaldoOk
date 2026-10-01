pub mod purchases;
pub mod expenses;
pub mod daily_cash;

pub fn get_handlers() -> impl Fn(tauri::ipc::Invoke) -> bool {
    tauri::generate_handler![
        // Suppliers
        purchases::suppliers::get_suppliers,
        purchases::suppliers::create_supplier,
        purchases::suppliers::update_supplier,
        purchases::suppliers::delete_supplier,
        
        // Purchases
        purchases::purchases::get_purchases,
        purchases::purchases::create_purchase,
        purchases::purchases::update_purchase,
        purchases::purchases::delete_purchase,
        
        // Purchase Payments
        purchases::purchase_payments::get_purchase_payments,
        purchases::purchase_payments::create_purchase_payment,
        purchases::purchase_payments::update_purchase_payment,
        purchases::purchase_payments::delete_purchase_payment,

        // Spenders
        expenses::spenders::get_spenders,
        expenses::spenders::get_spender_by_id,
        expenses::spenders::create_spender,
        expenses::spenders::update_spender,
        expenses::spenders::delete_spender,

        // Expenses Categories
        expenses::expenses_categories::get_expense_categories,
        expenses::expenses_categories::get_expense_category_by_id,
        expenses::expenses_categories::create_expense_category,
        expenses::expenses_categories::update_expense_category,
        expenses::expenses_categories::delete_expense_category,

        // Expenses
        expenses::expenses::get_expenses,
        expenses::expenses::get_expense_by_id,
        expenses::expenses::create_expense,
        expenses::expenses::update_expense,
        expenses::expenses::delete_expense,

        // Daily Cash
        daily_cash::daily_cash_registers::create_daily_cash_register,
        daily_cash::daily_cash_registers::get_daily_cash_registers,
        daily_cash::daily_cash_registers::get_daily_cash_register_by_date,
        daily_cash::daily_cash_registers::update_daily_cash_register,
        daily_cash::daily_cash_registers::delete_daily_cash_register
    ]
}
