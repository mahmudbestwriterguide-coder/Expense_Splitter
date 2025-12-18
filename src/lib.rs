use std::io;

/// Represents a shared apartment
pub struct Apartment {
    pub tenants: Vec<String>,
    pub expenses: Vec<f64>,
}

impl Apartment {
    /// Create a new empty apartment
    pub fn new() -> Self {
        Apartment {
            tenants: Vec::new(),
            expenses: Vec::new(),
        }
    }

    /// Read tenant names from user input
    pub fn read_tenants(&mut self) {
        println!("Enter number of tenants:");
        let count = read_usize();

        for i in 1..=count {
            println!("Enter name of tenant {}:", i);
            let name = read_string();
            self.tenants.push(name);
        }
    }

    /// Read shared expenses from user input
    pub fn read_expenses(&mut self) {
        println!("Enter number of shared expenses:");
        let count = read_usize();

        for i in 1..=count {
            println!("Enter amount for expense {} (€):", i);
            let amount = read_f64();
            self.expenses.push(amount);
        }
    }

    /// Calculate total expenses
    pub fn total_expenses(&self) -> f64 {
        self.expenses.iter().sum()
    }

    /// Calculate expense per tenant
    pub fn split_per_tenant(&self) -> f64 {
        self.total_expenses() / self.tenants.len() as f64
    }

    /// Print final summary
    pub fn print_summary(&self) {
        let total = self.total_expenses();
        let per_person = self.split_per_tenant();

        println!("\n--- Expense Summary ---");
        println!("Total Expenses: €{:.2}", total);
        println!("Each tenant pays: €{:.2}\n", per_person);

        for tenant in &self.tenants {
            println!("{} should pay €{:.2}", tenant, per_person);
        }
    }
}

/// Helper: read String
fn read_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/// Helper: read usize
fn read_usize() -> usize {
    let input = read_string();
    input.parse().expect("Please enter a valid number")
}

/// Helper: read f64
fn read_f64() -> f64 {
    let input = read_string();
    input.parse().expect("Please enter a valid amount")
}
