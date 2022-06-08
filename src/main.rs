//3x+1 if odd or x/2 if even
//print output after each iteration
//start at 2^68
//if number goes down to 4, add one to the count and start over

//struct to store u128 integer
#[derive(Debug)]
struct BIGINT {
    value: u128,
}


impl BIGINT {
    //constructor
    fn new(value: u128) -> BIGINT {
        BIGINT {
            value: value,
        }
    }
    //method to check if number is odd
    fn is_odd(&self) -> bool {
        self.value % 2 == 1
    }
    //method to check if number is even
    fn is_even(&self) -> bool {
        self.value % 2 == 0
    }
    //method to multiply BIGINT by 3 and add 1
    fn three_plus_one(&mut self) {
        self.value = self.value * 3 + 1;
    }
    //method to divide BIGINT by 2
    fn divide_by_two(&mut self) {
        self.value = self.value / 2;
    }
    //method to print the value of the BIGINT
    fn print_value(&self) {
        println!("{}", self.value);
    }
    //method to check if the value of the BIGINT is 4
    fn is_four(&self) -> bool {
        self.value == 4
    }
    //method to check if the value of the BIGINT is 1
    fn is_one(&self) -> bool {
        self.value == 1
    }
}

impl PartialEq for BIGINT {
    fn eq(&self, other: &BIGINT) -> bool {
        self.value == other.value
    }
}


fn problem() {
    let mut count = 0;
    // new BIGINT object
    let mut big_int = BIGINT::new(2_u128.pow(127));
    // loop until the value of the BIGINT is 1
    while !big_int.is_one() {
        // if the value of the BIGINT is 4, add 1 to the count and start over
        if big_int.is_four() {
            count += 1;
            big_int = BIGINT::new(2_u128.pow(68));
        }
        // if the value of the BIGINT is odd, multiply by 3 and add 1
        if big_int.is_odd() {
            big_int.three_plus_one();
        }
        // if the value of the BIGINT is even, divide by 2
        if big_int.is_even() {
            big_int.divide_by_two();
        }
        // print the value of the BIGINT
        big_int.print_value();
    }
    
}

//function to print the number of iterations
fn print_count(count: u128) {
    println!("{}", count);
}

fn main() {
    problem();
}
