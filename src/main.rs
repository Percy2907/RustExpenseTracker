use std::io::{self, Write};

/// Stores the information for one expense.
#[derive(Debug, Clone)]
struct Expense {
    id: u32,
    description: String,
    category: String,
    amount: f64,
}

impl Expense {
    /// Creates a new expense with the provided information.
    fn new(id: u32, description: String, category: String, amount: f64) -> Self {
        Self {
            id,
            description,
            category,
            amount,
        }
    }

    /// Displays one expense in a consistent format.
    fn display(&self) {
        println!(
            "ID: {} | {} | Category: {} | Amount: ${:.2}",
            self.id, self.description, self.category, self.amount
        );
    }
}

/// Starts the terminal menu and keeps it running until the user exits.
fn main() {
    let program_name = "Rust Expense Tracker";
    let mut expenses: Vec<Expense> = Vec::new();
    let mut next_id: u32 = 1;

    println!("Welcome to {program_name}!");

    loop {
        display_menu();
        let choice = read_text("Choose an option: ");

        match choice.as_str() {
            "1" => add_expense(&mut expenses, &mut next_id),
            "2" => display_expenses(&expenses),
            "3" => search_by_category(&expenses),
            "4" => display_total(&expenses),
            "5" => delete_expense(&mut expenses),
            "6" => {
                println!("Thank you for using the expense tracker.");
                break;
            }
            _ => println!("Invalid option. Enter a number from 1 to 6."),
        }
    }
}

/// Displays all available menu options.
fn display_menu() {
    println!("\nRust Expense Tracker");
    println!("1. Add expense");
    println!("2. Show all expenses");
    println!("3. Search by category");
    println!("4. Show total amount");
    println!("5. Delete expense");
    println!("6. Exit");
}

/// Reads text from the terminal and returns it without extra spaces.
fn read_text(prompt: &str) -> String {
    loop {
        print!("{prompt}");
        if let Err(error) = io::stdout().flush() {
            eprintln!("Could not display the prompt: {error}");
        }

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let value = input.trim().to_string();
                if value.is_empty() {
                    println!("This value cannot be empty.");
                } else {
                    return value;
                }
            }
            Err(error) => println!("Could not read the input: {error}"),
        }
    }
}

/// Reads and validates a positive amount from the terminal.
fn read_amount(prompt: &str) -> f64 {
    loop {
        let input = read_text(prompt);
        match input.parse::<f64>() {
            Ok(amount) if amount > 0.0 && amount.is_finite() => return amount,
            _ => println!("Enter a valid amount greater than zero."),
        }
    }
}

/// Reads and validates a positive whole number from the terminal.
fn read_id(prompt: &str) -> u32 {
    loop {
        let input = read_text(prompt);
        match input.parse::<u32>() {
            Ok(id) if id > 0 => return id,
            _ => println!("Enter a valid positive ID."),
        }
    }
}

/// Gets expense information from the user and adds it to the list.
fn add_expense(expenses: &mut Vec<Expense>, next_id: &mut u32) {
    println!("\nAdd Expense");
    let description = read_text("Description: ");
    let category = read_text("Category: ");
    let amount = read_amount("Amount: $");
    let expense = Expense::new(*next_id, description, category, amount);

    expenses.push(expense);
    println!("Expense added with ID {}.", *next_id);
    *next_id += 1;
}

/// Displays every expense without taking ownership of the list.
fn display_expenses(expenses: &[Expense]) {
    println!("\nAll Expenses");
    if expenses.is_empty() {
        println!("No expenses have been added.");
        return;
    }

    for expense in expenses {
        expense.display();
    }
    println!("Total expenses: {}", expenses.len());
}

/// Searches for expenses whose category matches the user's text.
fn search_by_category(expenses: &[Expense]) {
    println!("\nSearch by Category");
    if expenses.is_empty() {
        println!("There are no expenses to search.");
        return;
    }

    let category = read_text("Enter a category: ");
    let (match_count, category_total) = category_summary(expenses, &category);

    if match_count == 0 {
        println!("No expenses were found in the '{category}' category.");
        return;
    }

    for expense in expenses {
        if expense.category.eq_ignore_ascii_case(&category) {
            expense.display();
        }
    }

    println!("Category total: ${category_total:.2}");
}

/// Counts matching expenses and calculates their total with a loop.
fn category_summary(expenses: &[Expense], category: &str) -> (usize, f64) {
    let mut count = 0;
    let mut total = 0.0;

    for expense in expenses {
        if expense.category.eq_ignore_ascii_case(category) {
            count += 1;
            total += expense.amount;
        }
    }

    (count, total)
}

/// Calculates and displays the amount of all expenses.
fn display_total(expenses: &[Expense]) {
    let total = calculate_total(expenses);
    println!("\nTotal amount: ${total:.2}");
}

/// Uses a loop and an expression to return the total amount.
fn calculate_total(expenses: &[Expense]) -> f64 {
    let mut total = 0.0;

    for expense in expenses {
        total += expense.amount;
    }

    total
}

/// Removes an expense selected by its ID.
fn delete_expense(expenses: &mut Vec<Expense>) {
    println!("\nDelete Expense");
    if expenses.is_empty() {
        println!("There are no expenses to delete.");
        return;
    }

    display_expenses(expenses);
    let id = read_id("Enter the ID to delete: ");
    if remove_by_id(expenses, id) {
        println!("Expense {id} was deleted.");
    } else {
        println!("Expense {id} does not exist.");
    }
}

/// Finds an expense by ID and removes it with a mutable reference.
fn remove_by_id(expenses: &mut Vec<Expense>, id: u32) -> bool {
    let mut index = 0;

    while index < expenses.len() {
        if expenses[index].id == id {
            expenses.remove(index);
            return true;
        }

        index += 1;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates sample data used by the automated tests.
    fn sample_expenses() -> Vec<Expense> {
        vec![
            Expense::new(1, "Lunch".to_string(), "Food".to_string(), 12.50),
            Expense::new(2, "Bus".to_string(), "Transport".to_string(), 3.25),
            Expense::new(3, "Dinner".to_string(), "Food".to_string(), 18.00),
        ]
    }

    /// Verifies that all amounts are included in the total.
    #[test]
    fn calculates_total() {
        let expenses = sample_expenses();
        assert_eq!(calculate_total(&expenses), 33.75);
    }

    /// Verifies that category search ignores uppercase and lowercase differences.
    #[test]
    fn finds_category_without_case_difference() {
        let expenses = sample_expenses();
        let (count, total) = category_summary(&expenses, "food");
        assert_eq!(count, 2);
        assert_eq!(total, 30.50);
    }

    /// Verifies that an existing expense can be removed by ID.
    #[test]
    fn removes_existing_expense() {
        let mut expenses = sample_expenses();
        assert!(remove_by_id(&mut expenses, 2));
        assert_eq!(expenses.len(), 2);
        assert_eq!(expenses[0].id, 1);
        assert_eq!(expenses[1].id, 3);
    }

    /// Verifies that an unknown ID does not change the list.
    #[test]
    fn keeps_list_when_id_does_not_exist() {
        let mut expenses = sample_expenses();
        assert!(!remove_by_id(&mut expenses, 99));
        assert_eq!(expenses.len(), 3);
    }
}
