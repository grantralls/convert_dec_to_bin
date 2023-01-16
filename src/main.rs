use std::env;

const MAX_DIGITS: u8 = 8;

fn main() {
    // Get arguments from the command line
    let args: Vec<String> = env::args().collect();

    println!("|{0: <10} | {1: <10}|", "Base 10", "Base 2");

    // Loop through each argument
    for argument in &args[1..] {
        // Parse the argument as a float
        let argument = match argument.parse::<f32>() {
            Ok(argument) => argument,
            Err(_) => panic!("Invalid argument: {}", argument),
        };

        let result = convert_dec_to_bin(argument);
        println!("|{0: <10} | {1: <10}|", argument, result);
    }
}

fn convert_dec_to_bin(dec: f32) -> f32 {
    let mut bin = String::new();

    // Set initial temp
    let mut temp: f32 = dec * 2.0;

    // Keep looping until the temp is 0 or the max digits is reached
    for _ in 0..MAX_DIGITS {
        if temp == 0.0 {
            // If the result is 0, stop the loop
            break;
        } else if temp >= 1.0 {
            // If the result is greater than or equal to 1, append 1 to the binary string and subtract 1 from the temp
            bin.push('1');
            temp = temp - 1.0;
        } else {
            // If the result is less than 1, append 0 to the binary string
            bin.push('0');
        }

        temp = temp * 2.0;
    }

    bin.insert_str(0, "0.");

    bin.parse::<f32>()
        .expect("Failed to parse binary string to float")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_bin_to_dec() {
        assert_eq!(convert_dec_to_bin(0.5), 0.1);
        assert_eq!(convert_dec_to_bin(0.25), 0.01);
        assert_eq!(convert_dec_to_bin(0.75), 0.11);
        assert_eq!(convert_dec_to_bin(0.333), 0.01010101);
        assert_eq!(convert_dec_to_bin(0.2), 0.00110011);
        assert_eq!(convert_dec_to_bin(0.1), 0.00011001);
        assert_eq!(convert_dec_to_bin(0.142857), 0.00100100);
    }
}
