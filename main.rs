pub fn sublist<T: PartialEq>(f: &[T], s: &[T]) -> Comparison {
    if f.is_empty() && s.is_empty() {return Comparison::Equal }
    if f.is_empty() {return Comparison::Sublist  }
    if s.is_empty() {return Comparison::Superlist }
    return if f.len() >= s.len() {
        search(f, s)
    } else {
        let result = search(s, f);
        match result {
            Comparison::Sublist => Comparison::Superlist,
            Comparison::Superlist => Comparison::Sublist,
            _ => result
        }
    }
}

fn search<T: PartialEq>(small: &[T], big: &[T])->Comparison{
    for a in 0..big.len() {
        println!("small1 - {:p}",&small[0]);
        println!("big1 - {:p}",&big[a]);
        println!("***");
        if small[0] == big[a]{
            for b in 0..small.len() {
                if small[b] != big[a+b] {
                    println!("small2 - {:p}",&small[b]);
                    println!("big2 - {:p}",&big[b+a]);
                    println!("***");
                    break;
                }
            }
        }
    }
    return Comparison::Unequal;
}
