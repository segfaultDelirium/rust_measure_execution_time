use std::{time::Instant, collections::HashMap};


fn main() {
    let now = Instant::now();

    let mut results_map: HashMap<i64, i64> = HashMap::new();

    let iterations = 1000000;
    let fibbonacci_iterations = 15;
    for _i in 1..iterations{
        fibbonacci(fibbonacci_iterations, &mut results_map);
    }
    let after1 = now.elapsed();


    let now = Instant::now();   
    for _i in 1..iterations{
        naive_fibbonacci(fibbonacci_iterations);
    }
    
    let after2 = now.elapsed();

    println!("hashmap fibbonacci time: {:?} naive fibbonacci time: {:?}", 
        after1, after2);
}


fn fibbonacci(n: i64, results_map: &mut HashMap<i64, i64>) -> i64{
    if n < 1 { return 0 }
    if n == 1 { return 1 }
    if results_map.contains_key(&n) {
        return *results_map.get(&n).unwrap();
    }
    let res = fibbonacci(n-1, results_map) + fibbonacci(n-2, results_map);
    results_map.insert(n, res);
    return res;
}


fn naive_fibbonacci(n: i64) -> i64{
    if n < 1 {return 0};
    if n == 1 {return 1};
    
    return naive_fibbonacci(n-1) + naive_fibbonacci(n-2);
}