pub fn sublist<T: PartialEq>(f: &[T], s: &[T]) -> Comparison {
    if f.is_empty() && s.is_empty() { return Comparison::Equal; }
    if f.is_empty() { return Comparison::Sublist; }
    if s.is_empty() { return Comparison::Superlist; }
    if f == s { return Comparison::Equal; }
    if containSlice(f, s) { return Comparison::Superlist; }
    if containSlice(s, f) { return Comparison::Sublist; }
    Comparison::Unequal
}

pub fn containSlice<T: PartialEq>(big: &[T], small: &[T]) -> bool {
    if big.len() < small.len() { return false; }
    let res = false;
    for i in 0..big.len() {
        if big[i] == small[0] {
            for j in 1..small.len() {
                if big[i + j] != small[j] {

                }
            }
        }
    }
    true
}
