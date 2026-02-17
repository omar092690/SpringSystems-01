fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let nums: [i32; 10] = [12, 7, 15, 22, 30, 9, 5, 18, 4, 11];

    for i in 0..nums.len() {
        let n = nums[i];

        if n % 3 == 0 && n % 5 == 0 {
            println!("{n}: FizzBuzz");
        } else if n % 3 == 0 {
            println!("{n}: Fizz");
        } else if n % 5 == 0 {
            println!("{n}: Buzz");
        } else if is_even(n) {
            println!("{n}: even");
        } else {
            println!("{n}: odd");
        }
    }

    //adds all numbers
    let mut i: usize = 0;
    let mut sum: i32 = 0;
    while i < nums.len() {
        sum += nums[i];
        i += 1;
    }
    println!("Sum = {sum}");

    //looks for largets num 
    let mut j: usize = 0;
    let mut largest: i32 = nums[0];

    loop {
        if nums[j] > largest {
            largest = nums[j];
        }
        j += 1;

        if j >= nums.len() {
            break;
        }
    }

    println!("Largest = {largest}");
}
