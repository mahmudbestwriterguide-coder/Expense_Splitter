use std::io;

fn main() {
    println!("=== FlatSplit: Expense Splitter for Shared Apartments ===");

    let mut tenants: Vec<String> = Vec::new();
    let mut expenses: Vec<f64> = Vec::new();

    // Number of tenants
    println!("Enter number of tenants:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let tenant_count: usize = input.trim().parse().unwrap();

    // Tenant names
    for i in 1..=tenant_count {
        println!("Enter name of tenant {}:", i);
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        tenants.push(input.trim().to_string());
    }

    // Number of expenses
    println!("Enter number of shared expenses:");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let expense_count: usize = input.trim().parse().unwrap();

    // Expense amounts
    for i in 1..=expense_count {
        println!("Enter amount for expense {} (€):", i);
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let amount: f64 = input.trim().parse().unwrap();
        expenses.push(amount);
    }

    let total: f64 = expenses.iter().sum();
    let per_person = total / tenant_count as f64;

    println!("\n--- Expense Summary ---");
    println!("Total Expenses: €{:.2}", total);
    println!("Each tenant pays: €{:.2}\n", per_person);

    for tenant in tenants {
        println!("{} should pay €{:.2}", tenant, per_person);
    }

    println!("\nThank you for using FlatSplit!");
}
