mod command;

use command::*;
pub use std::{io, collections::HashMap};

fn main() {
    let mut enterprise: HashMap<String, Vec<String>> = HashMap::new();
    println!("**** Add Employee Information Text Interface ****");
    println!("*   Add => \"add {{name}} to {{department}}\"         *");
    println!("* Check department employees => \"{{department}}\"  *");
    println!("*  Check enterprise employees => \"enterprise\"   *");
    println!("*               Exit => \"exit\"                  *");
    println!("*************************************************");

    loop {
        let mut query = String::new();
        let mut check = 0;

        println!("Input: ");
        match io::stdin().read_line(&mut query) {
            Ok(_) => {
                if &query == "exit\n" {
                    break;
                } else if &query == "enterprise\n" {
                    view_enterprise(&enterprise);
                } else {
                    for (k, _) in &enterprise {
                        if k == &query.trim() {
                            view_department_employees(&enterprise, k);
                            check = 1;
                        }
                    }
                    if check == 0 {
                        match add_employee(&mut enterprise, query) {
                            false => {
                                println!("Input Error!");
                                continue;
                            },
                            true => {
                                println!("Complete!");
                            },
                        }
                    }
                }
            },
            Err(_) => {
                println!("Input Error!");
                continue;
            },
        }
        println!("Running...");
    }
}