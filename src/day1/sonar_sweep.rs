use std::env;
use std::io::prelude::*;
use std::fs::File;

//Read through the problem of day 1 from https://adventofcode.com/2021/day/1


//SOLUTION
//For the input data each input data is unique to an account.
//First I got the input into a text file called input.txt and used rust File::open to get the file,
//I changed the content of the file into a vector and converted it i32 data type.
//crated a fun part one and  I looped through the Vec and check if the current value is lesser than the next, if true increment result.
//The result is sent to the advent of code site
//func part two adds the current value in the loop to the current[index + 1] and current[1+2]
//alThe measurements in the first window are marked A (199, 200, 208); their sum is 199 + 200 + 208 = 607. 
//The second window is marked B (200, 208, 210); its sum is 618. 
//The sum of measurements in the second window is larger than the sum of the first, 
//so result is increased by 1.



pub fn run() {
    println!("Hi From sona sweeps!");
 
    let mut input_numbers : Vec<i32> = Vec::new();
    let mut input_string = String::new();
    let filename = "input.txt";
    let mut file = File::open(filename)
                .expect("Something went wrong reading the file");
                file.read_to_string(&mut input_string)
                         .expect("Something went wrong reading the file");
            
              for num in input_string.split("\r\n") {
               let mut input_number:i32 = num.parse().expect("Expect a numb");
               input_numbers.push(input_number)
              }
           let part_one =   part_one(&input_numbers);
           let part_two = part_two(&input_numbers);
           println!("{}", part_two);


}

pub fn part_one(numbers:&Vec<i32>) -> i32{
    let len  = numbers.len();
    let mut result :i32 = 0;
    for i in 0..len -1  {
          if numbers[i] < numbers[i+1] {
              result += 1;
          }
    }
    result
}

pub fn part_two(numbers:&Vec<i32>) ->i32 {
    let len = numbers.len();
    let mut result :i32 = 0;

    for i in 0..len -3 {
        let sum1 = numbers[i] + numbers[i+1] + numbers[i+2];
        let sum2 = numbers[i+1] + numbers[i+2] + numbers[i+3];
        //println!("first window: {} {} {}", {numbers[i]}, {numbers[i+1]}, {numbers[i+2]});
        //println!("second window: {} {} {}", {numbers[i+1]}, {numbers[i+2]}, {numbers[i+3]});
        //println!("{} {}", {sum1}, {sum2});
        if sum1 < sum2 {
            result += 1;
        }
    }
    result
}