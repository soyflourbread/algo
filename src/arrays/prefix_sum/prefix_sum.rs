pub fn to_pref_vec(vec: Vec<u32>) -> Vec<u32> {
    let mut ret = Vec::with_capacity(vec.len() + 1);

    let mut pref_sum = u32::MIN;
    ret.push(pref_sum);
    for e in vec {
        pref_sum += e;
        ret.push(pref_sum);
    }

    ret
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    let pref_vec = to_pref_vec(vec);

    println!("pref_vec: {:?}", pref_vec);
}
