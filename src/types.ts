export interface Supplier {
  id: number;
  name: string;
  active: number;
  color: string;
}

export interface Purchase {
  id?: number;
  supplier_id: number;
  invoice_number?: string;
  issue_date: string;
  due_date?: string;
  total_amount: number;
  payment_type: string;
  status: string;
  notes?: string;
  supplier_name?: string;
}

export interface Spender {
  id: number;
  name: string;
  active: number;
}

export interface ExpenseCategory {
  id: number;
  name: string;
  color: string;
  active: number;
}

export interface Expense {
  id?: number;
  spender_id: number;
  category_id: number;
  amount: number;
  expense_date: string;
  notes?: string;
}
