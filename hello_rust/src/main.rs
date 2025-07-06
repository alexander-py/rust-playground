// fn add(a: i32, b: i32) -> i32 {
//     a + b
// }
//
// fn main() {
//     println!("1330 + 7 = {}", add(1330, 7));
// }


// fn main() {
//     for i in 0..11 {
//         if i % 2 == 0 {
//             println!("is even ---> {}", i)
//         } else {
//             println!("is not even ---> {}", i)
//         }
//     }
// }

fn main() {
    let s1 = String::from("testing, attention please");
    let s2 = s1; // ownership moves
    // println!("{}", s1); // let us break it
    println!("{}", s2)
}