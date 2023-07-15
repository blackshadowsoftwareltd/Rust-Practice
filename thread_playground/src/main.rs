use rayon::prelude::*;
#[tokio::main]
async fn main() {
    let list = vec![6, 8, 1, 9, 3, 4, 10, 2, 5, 7];
    println!("{:?}", list);

    let find_5: Option<&i32> = list.par_iter().find_any(|&&x| x == 5);
    println!("find 5 {:?}", find_5); // ? find 5 Some(5)

    let find_2_to_8 = list.par_iter().find_any(|&&x| (2..=8).contains(&x));
    println!("find 2 to 8 {:?}", find_2_to_8); // ? find 2 to 8 Some(6)

    let find_first_4 = list.par_iter().find_first(|&&x| x == 4);
    println!("find first 4 {:?}", find_first_4); // ? find first 4 Some(4)

    let find_last_7 = list.par_iter().find_last(|&&x| x == 7);
    println!("find last 7 {:?}", find_last_7); // ? find last 7 Some(7)

    let mut mut_list = list.clone();
    mut_list.par_sort(); // ?  mut_list.par_sort_unstable(); (more faster)
    println!("{:?}", mut_list); // ? [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

    mut_list = list.clone();
    mut_list.par_sort_by(|a, b| b.cmp(a));
    println!("{:?}", mut_list); // ? [10, 9, 8, 7, 6, 5, 4, 3, 2, 1]

    // ? more faster
    mut_list = list.clone();
    mut_list.par_sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
    println!("{:?}", mut_list); // ? [10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
}
