fn era(n: usize) -> Vec<bool> {
    let mut isprime = vec![true; n];
    isprime[0] = false;
    isprime[1] = false;
    for i in 2..n {
        if isprime[i] {
            let mut j = 2 * i;
            while j < n {
                isprime[j] = false;
                j += i;
            }
        }
    }

    isprime
}
