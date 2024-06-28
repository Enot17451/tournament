pub fn sublist<T: PartialEq>(f: &[T], s: &[T]) -> Comparison {
    let big:&[T];
    let small:&[T];
    if f.len()>s.len() {
        big = f;
        small = s;
    }else {
        big = s;
        small = f;
    }
    for a in 0..big.len() {
        if small[0] == big[a]{
            for b in 0..small.len() {
                if small[b] != big[a+b] {
                    return Comparison::Unequal
                }
            }
            return Comparison::Sublist
        }
    }
    return Comparison::Unequal
}
