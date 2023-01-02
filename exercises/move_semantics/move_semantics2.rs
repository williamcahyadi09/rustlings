// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

 /*
    Solution:
    1. Make a vec0 clone and pass to the function. (based on hint 1)
    2. pass the reference of vec0 to fill_vec function, 
    and in the function copy the data from the vector (hint 2)
    3. pass a mutable reference of vec0 to fill_vec function, 
    and directly modify vec0 with its mutable reference (hint 3, current solution)
 */



fn main() {
    let mut vec0 = Vec::new();

    fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec0.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);
}

fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
