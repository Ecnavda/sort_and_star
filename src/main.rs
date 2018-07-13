fn main() {
    println!("Sort the vector of strings alphbetically and return the first value with '***' between each character.");

    println!("{}", two_sort(&["bitcoin", "arnold", "test"]));
}

fn two_sort(arr: &[&str]) -> String {
    let mut x = arr.to_vec();
    
    x.sort_unstable();

    let output: Vec<String> = x[0].chars().map(|a| a.to_string()).collect();

    output.join("***")
}
