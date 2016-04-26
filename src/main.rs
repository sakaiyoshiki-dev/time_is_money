// Proverb Programming
// "Time is money"
// Count duration time, convert it to money, and display.

use std::time;
use std::io;

fn main() {
  println!("Please input your hourly wage:");
  let mut hourly_wage_string = String::new();
  io::stdin().read_line(&mut hourly_wage_string)
              .expect("Failed to read line");
  println!("Your hourly wage is :{}",hourly_wage_string);
  hourly_wage_string = hourly_wage_string.trim_right().to_string(); // trim_right is important
  let hourly_wage: f32 = hourly_wage_string.parse().unwrap();
  //let second_wage: f32 = hourly_wage/60/60;
  let second_wage: f32 = hourly_wage/60.0/60.0;
  println!("second_wage :{:?}",second_wage);

  // Count time in second
  let start_time = time::Instant::now();
  let mut count_second:u64 = 0;
  loop {
    if count_second <= start_time.elapsed().as_secs() {
      count_second += 1;
      println!("You have wasted {:.0} secs. = {:.3} Yen",count_second,second_wage* (count_second as f32) );
    }
  }


  //let hourly_wage = i32::from_str("10000").unwrap();
  //println!("Your hourly wage is :{:?}",hourly_wage);
  // let hourly_wage_str = &hourly_wage_string;
  // printlnlet !("hourly_wage_str :{:?}",hourly_wage_str);
  // let aaa = hourly_wage_str.parse::<i32>(); //;
  // println!("Your hourly wage is :{:?}",aaa);
  //let hourly_wage: f32 = hourly_wage_string.parse::<f32>().unwrap();
  //println!("Your hourly wage is :{:?}",hourly_wage);


  // create instan
  let start_time = time::Instant::now();
  println!("now is :{:?}",start_time);

  // calc. duration
  let duration = time::Instant::now() - start_time;
  println!("duration :{:?}",duration);

  // elapsed
  // loop {
  //     println!("duration 2 :{:?}",start_time.elapsed());
  // }

}