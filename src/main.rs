extern crate core;

use infrastructure::domain::respositories::InMemoryMonthsRepository;
use std::io;
use std::io::Write;
use application::handlers::GetMonthHandler;
use application::requests::GetMonthRequest;

mod infrastructure;
mod domain;
mod application;

fn menu() -> i8 {
    print!("\nInput kode: ");
    io::stdout().flush().expect("Error flusing");

    let mut option: String = String::new();
    io::stdin().read_line(&mut option)
    .expect("Failed to read line");

    let option = option.trim().parse();

    match option {
        Ok(o) => return o,
        Err(_) => {
            eprintln!("Error: Please, type a number");
            -1
        }
    }
}

fn main() {
    let month_repo: InMemoryMonthsRepository = InMemoryMonthsRepository::new();

    let get_month_handler = GetMonthHandler::new(&month_repo);
    
    while  {
        let option: i8 = menu();

        match option {
            3 => {
                let req = GetMonthRequest::new(&3);
                let month = get_month_handler.execute(req);
                match month {
                    Ok(r) => println!("{}", r),
                    Err(e) => println!("{}", e)
                }
            }
            7 => {
                let req = GetMonthRequest::new(&7);
                let month = get_month_handler.execute(req);
                match month {
                    Ok(r) => println!("{}", r),
                    Err(e) => println!("{}", e)
                }
            }
            10 => {
                std::process::exit(0);
            }
            _ => println!("Invalid option")
        };
        option != 10
    } {}
}
