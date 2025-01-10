fn main() {
    let m1 = String::from("Hello,");
    let m2 = String::from("World!");
    greet(&m1, &m2);
    let s = format!("{} {}", m1, m2);
    println!("{s}")
}
/*
fn greet_equivalent(g1: String, g2: String) -> (String, String) {
    println!("{g1} {g2}");
    (g1, g2)
}
*/
fn greet(g1: &String, g2: &String)  {
    println!("{g1} {g2}");
}
