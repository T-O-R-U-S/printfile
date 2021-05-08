use std::{fs::File, io::Read};
use std::env::args;

fn main() -> std::io::Result<()>{
    let mut file = String::new();
    let args:Vec<String> = args().collect();
    let mut f = File::open(&args[1])?;
    f.read_to_string(&mut file)?;
    drop(f);
    let file:Vec<&str> = file.split('\n').collect();
    
    let mut eq_row_prefix = String::new();
    let mut eq_row = String::from("==||");
    let mut f_print:Vec<String> = Vec::new();
    let mut whitespace=  String::from(" ");
    for _ in 0..(file.len()-1).to_string().len() {
        eq_row_prefix = String::from("=") + &eq_row_prefix;
        whitespace.push(' ');
    }
    eq_row_prefix = String::from(&eq_row_prefix[..eq_row_prefix.len()-1]);
    let print = format!("f{}|| {}\n", &whitespace[..whitespace.len()-1], args[1]);
    for _ in 0..print.len()-4 {
        eq_row.push('=');
    }
    eq_row.push('\n');
    f_print.push(print);
    f_print.push(eq_row_prefix.clone()+&eq_row);
    for (l_num, line) in file.iter().enumerate() {
        let to_append = format!("{}{}|| {}\n", l_num, &whitespace[..whitespace.len()-l_num.to_string().len()], line);
        if to_append.len() > eq_row.len() {
            let mut new_eq = String::from("==||");
            for _ in 0..to_append.len()-4 {
                new_eq.push('=')
            }
            f_print[1] = eq_row_prefix.clone() + &new_eq + "\n";
        }
        f_print.push(to_append);
    }
    println!("{}", f_print.join(""));
    Ok(())
}