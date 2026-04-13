 // //!Assignment 5(Lec 4), Karima Shahzad,   06-04-2026
 use std::fmt;

    // //! 1: Error Handling — SOL Amount Parser
        //Part A
   
//fn parse_sol(input: &str) -> Result<f64, String>{
   // input.parse::<f64>().map_err(|_| String::from("Could not parse input as a number"))}

    // //!Part B: Replace String with a proper custom error enum.
// #[derive(Debug)]

// enum Transaction {
// NotANumber(String),          // input was not a valid number
// Negative,                    // value was below zero
// ExceedsMaxSupply,            // value exceeded 1 billion SOL
// TooManyDecimals(u32),        // more than 9 decimal places
// }

// impl fmt::Display for Transaction {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Transaction::NotANumber (input) => write!(f, "'{}' is not a valid number", input),
//             Transaction::Negative           => write!(f, "SOL amount cannot be negatived"),
//             Transaction::ExceedsMaxSupply   => write!(f, "Amount exceeds maximum SOL supply"),
//             Transaction::TooManyDecimals(n)=> write!(f, "Too many decimal places: {} (maximum is 9)", n),

//         }
//     }
// }
//                 // //! Part C: The ? Operator
// // Step 1: parse the f64 from the string (use Part B error type)
// fn parse_sol(input: &str) -> Result<f64, Transaction> { 
//     input
//         .parse::<f64>()
//         .map_err(|_| Transaction::NotANumber(input.to_string()))
//     }
//     // Step 2: convert SOL to lamports — validate during conversion
// fn sol_to_lamports(input: &str) -> Result<u64, Transaction> {
//     let sol = parse_sol(input)?;  // validate and convert

// if sol < 0.0 {
//         return Err(Transaction::Negative);
//     }

//      if sol > 1_000_000_000.0 {
//         return Err(Transaction::ExceedsMaxSupply);
//     }

//     let decimal_places = input
//         .find('.')                               // find the dot
//         .map(|i| (input.len() - i - 1) as u32)  // count digits after dot
//         .unwrap_or(0);                          // no dot = 0 decimal places

//     if decimal_places > 9 {
//         return Err(Transaction::TooManyDecimals(decimal_places));
//     }
//     Ok((sol * 1_000_000_000.0) as u64)
// }

// fn main(){
//    // Part A tests (parse_sol now uses Transaction, still works the same)
//     match parse_sol("2.5") {
//         Ok(v)  => println!("Valid: {} SOL", v),
//         Err(e) => println!("Error: {}", e),
//     }
//     match parse_sol("abc") {
//         Ok(v)  => println!("Valid: {} SOL", v),
//         Err(e) => println!("Error: {}", e),
//     }
//     println!(); // blank line for readability

//     // Part B: Direct Display test
//     println!("{}", Transaction::NotANumber(String::from("abc"))); 
//     println!("{}", Transaction::Negative);
//     println!("{}", Transaction::ExceedsMaxSupply);
//     println!("{}", Transaction::TooManyDecimals(12));

//     // Part C tests
//     let test_inputs = vec!["2.5", "-1.0", "abc", "2000000000.0", "0.0000000001"];
//     for input in test_inputs {
//         match sol_to_lamports(input) {
//             Ok(lamps) => println!("{} SOL = {} lamports", input, lamps),
//             Err(e)    => println!("Error for {}: {}", input, e),
//         }
//     }
// } 

            // //!Task 2: Traits — Blockchain Item Summary System
           // //!Part A — Define and Implement a Trait
// trait BlockchainItem {
//     fn item_type(&self) -> &str;
//     fn summary(&self) -> String;
//     fn print_info(&self) {
//         println!("[{}] {}", self.item_type(), self.summary());
//     }
// }

// struct Transaction {
//     signature: String,
//     fee: u64,
//     slot: u64,
//     success: bool,
// }
// struct Block {
//     blockhash: String,
//     slot: u64,
//     transaction_count: u32,
//     leader: String, // validator that produced this block
// }

// impl BlockchainItem for Transaction {
//     fn item_type(&self) -> &str {
//          "Transaction"
//         }
//     fn summary(&self) -> String {
//         format!("sig:{} | fee:{} lamports | slot:{} | {}",
//             self.signature, self.fee, self.slot,
//              if self.success { "OK" } else { "FAILED" })
//     }
// }

// impl BlockchainItem for Block {
//     fn item_type(&self) -> &str {
//         "Block"
//     }
//     fn summary(&self) -> String {
//         format!("hash:{} | slot:{} | transact: {} txs | leader:{}",
//             self.blockhash, self.slot, self.transaction_count, self.leader)
//     }
//    fn print_info(&self) {
//     println!("──────────────────────────");
//     println!("[{}] {}", self.item_type(), self.summary());
//     println!("──────────────────────────");
// }
// }

// fn main() {
//     let tx = Transaction {
//         signature: String::from("7xKXtg..."),
//         fee: 5000,
//         slot: 315000042,
//         success: true,
//     };

//     let block = Block {
//         blockhash: String::from("Ab3Gf9..."),
//         slot: 315000042,
//         transaction_count: 128,
//         leader: String::from("JumpCrypto"),
//     };

//     tx.print_info();
//     block.print_info();
// }

        // //!Part B — Standard Library Traits

//? 1. Add #[derive(Debug, Clone, PartialEq)] to Transaction
//? 2. Implement Display for Transaction:

// #[derive(Debug, Clone, PartialEq)]
// struct Transaction {
//     signature: String,
//     fee: u64,
//     slot: u64,
//     success: bool,
// }
// impl fmt::Display for Transaction {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "TX[slot:{}] fee:{}L status:{}",
//             self.slot,
//             self.fee,
//             if self.success { "OK" } else { "FAILED" })
//     }
// }
// fn main(){
//     let tx1 = Transaction {
//         signature: String::from("7xKXtg..."),
//         fee: 500,
//         slot: 315000042,
//         success: true,};
//     let tx2 = tx1.clone();
//     println!("{}", tx1);                    // uses Display
//     println!("{:?}", tx1);                  // uses Debug
//     println!("Equal: {}", tx1 == tx2);      // uses PartialEq
// }

        // //!Part C — Trait Objects (
//?Create a Vec that can hold BOTH Transaction and Block items, and call print_info() on each:
 //use struct defined above

// fn print_all_items(items: &[Box<dyn BlockchainItem>]) {
// println!("=== Blockchain Feed ({} items) ===", items.len());
// for item in items {
// item.print_info();
// }
// }
// fn main() {
// let items: Vec<Box<dyn BlockchainItem>> = vec![
// Box::new(Transaction { 
//     signature: String::from("7xKXtg..."),
//     fee: 5000,
//     slot: 315000042,
//     success: true,
// }),
// Box::new(Block {
//      blockhash: String::from("Ab3Gf9..."),
//      slot: 315000042,
//      transaction_count: 128,
//      leader: String::from("JumpCrypto"),
//  }),
// Box::new(Transaction {
//     signature: String::from("9mPQrs..."),
//     fee: 3000,
//     slot: 315000050,
//     success: false,  // ← make this one FAILED for variety
// }),
// ];
// print_all_items(&items);
// }

/*Step 1: println! prints "=== Blockchain Feed (3 items) ==="

Step 2: loop starts
   → item = Transaction(7xKXtg)  → calls print_info() → prints Transaction line
   → item = Block(Ab3Gf9)        → calls print_info() → prints dashes + Block line + dashes
   → item = Transaction(9mPQrs)  → calls print_info() → prints Transaction line

Step 3: loop ends, function is done*/


    // //!Task 3: Generics — Type-Safe Functions and Structs
         //? Part A — Generic Functions
 // Function 1 — find_min_max
// fn find_min_max<T: PartialOrd + Copy>(list: &[T]) -> Option<(T, T)> {
//     if list.is_empty() {
//         return None;
//     }
//     let mut min = list[0];
//     let mut max = list[0];
//     for &item in list {
//         if item < min { min = item; }
//         if item > max { max = item; }
//     }
//     Some((min, max))        // Return Some((min, max)) otherwise
// }

// //Function 2 — filter_by_threshold
// fn filter_by_threshold<'a, T: PartialOrd>(list: &'a [T], threshold: &T) ->
// Vec<&'a T> {
//     list.iter().filter(|x| *x >= threshold).collect()
// }

// //Function 3 — summarize_all
// use std::fmt::{Display, Debug};
// fn summarize_all<T: Display + Debug>(items: &[T], label: &str) {
// println!("=== {} ({} items) ===", label, items.len());
// for (i, item) in items.iter().enumerate() {
// println!(" [{}] Display: {} Debug: {:?}", i, item, item);
// }
// }
    
// fn main(){
//     let balances: Vec<u64> = vec![500_000_000, 2_000_000_000, 100_000_000,5_000_000_000];
//     let names: Vec<&str> = vec!["Jito", "Jump", "Chorus", "Everstake"];
//     let fees: Vec<f64> = vec![0.00025, 0.0005, 0.00010, 0.00030];
//     println!("{:?}", find_min_max(&balances));
//     println!("{:?}", find_min_max(&names));
//     println!("{:?}", find_min_max(&fees));
//     println!("{:?}", find_min_max::<u64>(&[])); // empty slice → None  

//     let lamports = vec![100_000u64, 500_000, 1_000_000, 5_000_000, 10_000_000];
//     let result = filter_by_threshold(&lamports, &1_000_000);
//     println!("{:?}", result); // [1000000, 5000000, 10000000]
//     let scores = vec![45i32, 72, 88, 55, 91, 60];
//     let passing = filter_by_threshold(&scores, &60);
//     println!("{:?}", passing); // [72, 88, 91, 60]

//     summarize_all(&[1u64, 2, 3, 4, 5], "Lamport values");
//     summarize_all(&["Solana", "Rust", "Anchor"], "Tech stack");
//     summarize_all(&[true, false, true], "Flags");
// }
    

        //? Part B — Generic Struct
// use std::fmt::{Display, Debug};
// #[derive(Debug)]
// struct Labeled<T> {
// value: T,
// label: String,
// unit: String,
// }
// impl<T: Display + PartialOrd + Copy> Labeled<T> {
// // Constructor
// fn new(value: T, label: &str, unit: &str) -> Labeled<T> { 
//     Labeled {
//         value,
//         label: label.to_string(),
//         unit: unit.to_string(),
//     }
// }

// // Print formatted: "Balance: 2.5 SOL"
// fn display(&self) { 
//      println!("{}: {} {}", self.label, self.value, self.unit);
// }
// // Return true if this value > other.value
// fn is_greater_than(&self, other: &Labeled<T>) -> bool { 
//      self.value > other.value
// }
// // Update the value
// fn update(&mut self, new_value: T) {  
//     self.value = new_value;
//  }
// }
// fn main(){
//     let mut balance = Labeled::new(2_000_000_000u64, "Balance", "lamports");
//     let fee = Labeled::new(5_000u64, "Fee", "lamports");
//     let price = Labeled::new(185.50f64, "SOL Price", "USD");
//     let rate = Labeled::new(7.2f64, "APY", "%");
//     balance.display(); // Balance: 2000000000 lamports
//     price.display(); // SOL Price: 185.5 USD
//     println!("{}", balance.is_greater_than(&fee)); // true
//     balance.update(3_000_000_000u64);
//     balance.display(); // Balance: 3000000000 lamports
// }

        // //!Task 4: Collections & Iterators — Account Ledger
        //?Part A — Vec Operations

/*Using only iterator methods, compute and print all of the following:
1. Total lamports across all accounts
2. Number of funded accounts (balance > 0)
3. Number of whale accounts (balance >= 10 SOL)
4. All balances converted to SOL as f64 — collect into Vec<f64>
5. All funded balances sorted descending — collect into Vec<u64>
6. The largest single balance (use max())
7. True/false: does any account have exactly 0 balance?*/
// fn main(){
//     let mut balances: Vec<u64> = vec![
//     5_000_000_000,          // Alice — 5 SOL
//     1_000_000_000,          // Bob — 1 SOL
//     15_000_000_000,         // Carol — 15 SOL
//     500_000_000,            // Dave — 0.5 SOL
//     8_000_000_000,          // Eve — 8 SOL
//     0,                      // Frank — empty
//     3_000_000_000,          // Grace — 3 SOL
// ];
// // 1. Total lamports
// let total: u64 = balances.iter().sum();
// println!("Total lamports: {}", total);

// // 2. Funded accounts
// let funded = balances.iter().filter(|x| **x > 0).count();
// println!("Funded accounts: {}", funded);

// // 3. Whale accounts
// let whales = balances.iter().filter(|x| **x >= 10_000_000_000).count();
// println!("Whale accounts: {}", whales);

// // 4. Convert to SOL
// let in_sol: Vec<f64> = balances.iter().map(|x| *x as f64 / 1_000_000_000.0).collect();
// println!("In SOL: {:?}", in_sol);

// // 5. Funded sorted descending
// let mut sorted: Vec<u64> = balances.iter().filter(|x| **x > 0).copied().collect();
// sorted.sort();
// sorted.reverse();
// println!("Sorted descending: {:?}", sorted);
// //balance.sort_by(|a, b| b.cmp(a)); //balances itself changes permanently. it's gone, not useable for later.

// // 6. Largest balance
// let max = balances.iter().max();
// println!("Largest: {:?}", max);

// // 7. Any zero balance?
// let has_zero = balances.iter().any(|x| *x == 0);
// println!("Has zero balance: {}", has_zero);

// }

            //?Part B — HashMap Operations
// 8. Look up Alice's balance with .get(). Handle the Option with match.
// 9. Alice receives 2 SOL airdrop — use get_mut() to update her balance in place.
// 10. Use the Entry API to add "Fiona" with 0 balance only if she doesn't exist yet. Then call the same
    //entry line again — her balance must NOT be overwritten.
// 11. Remove Dave from the ledger with .remove(). Print what was removed.
// 12. Using iterator pipeline on the HashMap — no manual loops — print all accounts with balance >=
//     1 SOL, sorted alphabetically by name   

// use std::collections::HashMap;
// fn main(){
//     let mut ledger: HashMap<String, u64> = HashMap::new();
//     // Insert these accounts:
//     ledger.insert(String::from("Alice"), 5_000_000_000);
//     ledger.insert(String::from("Bob"), 1_000_000_000);
//     ledger.insert(String::from("Carol"), 15_000_000_000);
//     ledger.insert(String::from("Dave"), 500_000_000);
//     ledger.insert(String::from("Eve"), 8_000_000_000);

//     // 8. Look up Alice
//     match ledger.get("Alice") {
//         Some(balance) => println!("Alice's balance: {}", balance),
//         None => println!("Alice not found"),
//     }

//     // 9. Alice receives 2 SOL airdrop
//     if let Some(balance) = ledger.get_mut("Alice") {
//         *balance += 2_000_000_000;
//     }
//     println!("Alice after airdrop: {}", ledger["Alice"]);

//     // 10. Entry API — add Fiona only if she doesn't exist
//     ledger.entry(String::from("Fiona")).or_insert(0);
//     ledger.entry(String::from("Fiona")).or_insert(0); // won't overwrite
//     println!("Fiona's balance: {}", ledger["Fiona"]);

//     // 11. Remove Dave
//     let removed = ledger.remove("Dave");
//     println!("Removed Dave: {:?}", removed);

//     // 12. Accounts with balance >= 1 SOL, sorted alphabetically
//     let mut result: Vec<(&String, &u64)> = ledger.iter().filter(|(_, v)| **v >= 1_000_000_000).collect();
//     result.sort_by_key(|(k, _)| *k);
//     for (name, balance) in result {
//         println!("{}: {}", name, balance);
//     }
// }
            //?Part C — Iterator Pipeline Challenge

use std::collections::HashMap;

fn main() {
    let accounts: Vec<(&str, u64)> = vec![
        ("Alice", 5_000_000_000),
        ("Bob", 1_000_000_000),
        ("Carol", 15_000_000_000),
        ("Dave", 500_000_000),
        ("Eve", 8_000_000_000),
        ("Frank", 0),
        ("Grace", 3_000_000_000),
    ];

    let sol_map: HashMap<String, f64> = accounts
        .iter()
        .filter(|(_, balance)| *balance >= 1_000_000_000)
        .map(|(name, balance)| (name.to_string(), *balance as f64 / 1_000_000_000.0))
        .collect();

    println!("Funded accounts: {}", sol_map.len());
    for (name, sol) in &sol_map {
        println!(" {}: {:.2} SOL", name, sol);
    }
}



