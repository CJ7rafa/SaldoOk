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
