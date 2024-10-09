pub fn sum_of_multiples(limit:u32, divs: &[u32]) ->  u32 {
    (1..limit)
    .filter(|x|divs.iter().any(|d| *d != 0 && x % d == 0))
    .sum()
}

fn main(){
    let limit = 10;
    let divisors = [3,5];
    let result = sum_of_multiples(limit, &divisors);
    print!("Sum of multiples of 3 and 5 below {}:{}",limit,result);

    let limit2 = 20;
    let divisor2 = [2,7];

    let result2 = sum_of_multiples(limit2, &divisor2);
    println!("Sum of multiples of 2 and 7 below {}: {}", limit2, result2);
}