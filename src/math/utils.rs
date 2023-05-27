pub fn pop_zeros(coeffs: &mut Vec<i64>) {
    while coeffs.last().is_some_and(|&x| x == 0) {
        coeffs.pop().unwrap();
    }
}

pub fn multinomial(n: usize, ks: &[usize]) -> usize {
    assert_eq!(n, ks.iter().sum());
    let mut result = (1..=n).product();
    for &k in ks {
        result /= (1..=k).product::<usize>();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalization() {
        let mut v = vec![3, 5, 4, 0, 0];
        pop_zeros(&mut v);
        assert_eq!(v, vec![3, 5, 4]);

        let mut v = vec![0, 0, 0];
        pop_zeros(&mut v);
        assert_eq!(v, vec![]);
    }

    #[test]
    fn multinomials() {
        assert_eq!(multinomial(6, &[2, 3, 1]), 60);
    }
}
