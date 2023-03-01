use std::io;

fn main() {
    loop {
        let mut mode = "";

        println!("Type the number 0 for Fahrenheit or 1 for Celsius to choose the starting mode.");
    
        let mut select = String::new();

        io::stdin()
            .read_line(&mut select)
            .expect("Failed to read line");

        let select: u32 = match select.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if select == 0 {
            mode = "Fahrenheit";
            println!("You have selected {mode}");
        }
        else {
            mode = "Celsius";
            println!("You have selected {mode}");
        }

        loop {
            let mut mode_value = String::new();

            println!("Please input the {mode} value for conversion.");
    
            io::stdin()
                .read_line(&mut mode_value)
                .expect("Failed to read line");
    
            let mode_value: f64 = match mode_value.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
    
            if select == 0 {
                let mode_value = 5.0 * (mode_value - 32.0) / 9.0;
                println!("{mode_value}");
            }
            else {
                let mode_value = (9.0/5.0) * mode_value + 32.0;
                println!("{mode_value}");
            }
        }
    }
}
