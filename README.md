# Printfile
A shoddily-coded alternative to cat with two bonus features:
- File name up top
- Line numbers
- ~~Looks fancier?~~

## Alternative: [Bat](https://github.com/sharkdp/bat)
It is programmed much better and has syntax highlighting as well!
Does everything this program does as well.

## Usage
Usage:
`pf <FILENAME>`

Example usage:
`pf main.rs`
Output:
```
f  || src/main.rs
===||========================================
0  || use std::{fs::File, io::Read};
1  || use std::env::args;
2  || 
3  || fn main() -> std::io::Result<()>{
4  ||     let mut file = String::new();
5  ||     let args:Vec<String> = args().collect();
6  ||     let mut f = File::open(&args[1])?;
7  ||     f.read_to_string(&mut file)?;
8  ||     drop(f);
9  ||     let file:Vec<&str> = file.split('\n').collect();
10 ||     
11 ||     let mut eq_row_prefix = String::new();
12 ||     let mut eq_row = String::from("==||");
13 ||     let mut f_print:Vec<String> = Vec::new();
14 ||     let mut whitespace=  String::from(" ");
15 ||     for _ in 0..(file.len()-1).to_string().len() {
16 ||         eq_row_prefix = String::from("=") + &eq_row_prefix;
17 ||         whitespace.push(' ');
18 ||     }
19 ||     eq_row_prefix = String::from(&eq_row_prefix[..eq_row_prefix.len()-1]);
20 ||     let print = format!("f{}|| {}\n", &whitespace[..whitespace.len()-1], args[1]);
21 ||     for _ in 0..print.len()-4 {
22 ||         eq_row.push('=');
23 ||     }
24 ||     eq_row.push('\n');
25 ||     f_print.push(print);
26 ||     f_print.push(eq_row_prefix.clone()+&eq_row);
27 ||     for (l_num, line) in file.iter().enumerate() {
28 ||         let to_append = format!("{}{}|| {}\n", l_num, &whitespace[..whitespace.len()-l_num.to_string().len()], line);
29 ||         if to_append.len() > eq_row.len() {
30 ||             let mut new_eq = String::from("==||");
31 ||             for _ in 0..to_append.len()-4 {
32 ||                 new_eq.push('=')
33 ||             }
34 ||             f_print[1] = eq_row_prefix.clone() + &new_eq + "\n";
35 ||         }
36 ||         f_print.push(to_append);
37 ||     }
38 ||     println!("{}", f_print.join(""));
39 ||     Ok(())
40 || }
```

# Installation:

## Unix systems
**Warning: This remains untested. Use at your own discretion, as I am not held liable for what happens to you/your system**
###### Making a backup is recommended!

Run `curl https://raw.githubusercontent.com/T-O-R-U-S/printfile/master/install_script.sh/ -sSf | sh`. 
This script compiles and moves the printfile executable into /usr/bin.
You can now execute printfile with `pf`.

If you don't wish to move printfile into /usr/bin/, you can still compile and run it by doing
```sh
git clone https://github.com/T-O-R-U-S/printfile.git
cd printfile
cargo b --release
mv ./target/release/printfile .
printfile /etc/passwd
```

## Windows users
You must suffer.