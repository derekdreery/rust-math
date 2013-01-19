fn each(v: &[int], op: fn(v: &int) -> bool) {
   let mut n = 0;
   while n < v.len() {
       if !op(&v[n]) {
           break;
       }
       n += 1;
   }
}

fn main() {
    let mut v = vec::with_capacity(6);
    for each([1, ..6]) |n| {
        v.push(n);
    };
    io::println(fmt!("%?", v));
}
