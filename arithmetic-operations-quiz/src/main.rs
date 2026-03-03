fn main() {
    println!("1 + 1 = ??");
    println!("??の値を入力して下さい:");
    let mut ans_input = String::new();
    std::io::stdin().read_line(&mut ans_input).unwrap();
    dbg!(ans_input);
}
