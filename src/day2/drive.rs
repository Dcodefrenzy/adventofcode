use std::io::prelude::*;
use std::fs::File;

//Read through the problem of day 2 ---  Dive! --- from https://adventofcode.com/2021/day/1


//SOLUTION
//For the input data each input data is unique to an account.
//First I got the input into a text file called input2.txt and used rust File::open to get the file,



pub fn run() {
   let mut input_data_vector: Vec<String> = Vec::new();
   let mut input_string = String::new();


    let filename = "input2.txt";
    let mut file = File::open(filename)
                .expect("Something went wrong reading the file");

                file.read_to_string(&mut input_string)
                         .expect("Something went wrong reading the file");

                         for input in input_string.split("\r\n") {
                              // let mut input_number:i32 = input.parse().expect("Expect a numb");
                              for inp in input.split_whitespace() {
                                 input_data_vector.push(inp.to_string());
                              }        
                          }
                          part_one(&input_data_vector);
                          part_two(&input_data_vector)
                          //println!("{:?}", input_data_vector);
                          //format_input_data(&input_data_vector);
                                              
}

pub fn part_one(data:&Vec<String>) {
   
   let mut horizontal :i32 = 0;
   let mut dept:i32 = 0;
   let len = data.len();
   for i in 0..len -1 {
       if i % 2 == 0 {
          let match_data = &data[i];
          let numb:i32 = data[i+1].parse().expect("Cant convert");
          if match_data == "forward" {
             horizontal += numb;
          }else if match_data == "up" {
             dept -= numb;
          }else {
             dept += numb;
          }
          //println!("dept: {} hor: {}", {dept}, {horizontal})
       }
       
   }
   println!("dept: {} hor: {} multiply {}", {dept}, {horizontal}, {dept * &horizontal});
}

pub fn part_two(data:&Vec<String>) {
   let mut horizontal :i32 = 0;
   let mut dept:i32 = 0;
   let mut aim:i32 = 0;
   let len = data.len();
   for i in 0..len -1 {
       if i % 2 == 0 {
          let match_data = &data[i];
          let numb:i32 = data[i+1].parse().expect("Cant convert");
          if match_data == "forward" {
             horizontal += numb;
             dept = numb * aim + dept;
          }else if match_data == "up" {
             aim -= numb;
          }else {
             aim += numb;
          }
          //println!("dept: {} hor: {}", {dept}, {horizontal})
       }
       
   }
   println!("dept: {}, hor: {}, aim {}, multiply {}", {dept}, {horizontal}, {aim}, {dept * &horizontal});
}


