// 確実な理解のためには実際にプラグラムを書いて動かしてみる
// まず疑似コードのレベルでアルゴリズムを設計する

#[cfg(test)]
mod tests {
    use super::euclid;

    #[test]
    fn comp() {
        assert_eq!(euclid(1633, 355), 71);
    }

    #[test]
    fn rev() {
        assert_eq!(euclid(355, 1633), 71);
    }

    #[test]
    fn prime() {
        assert_eq!(euclid(3, 2), 1);
    }

    #[test]
    fn zero_rhs() {
        assert_eq!(euclid(3, 0), 3);
    }

    #[test]
    fn zero_lhs() {
        assert_eq!(euclid(0, 3), 3);
    }

    #[test]
    #[should_panic]
    fn zero() {
        euclid(0, 0);
    }

    #[test]
    fn minus_rhs() {
        assert_eq!(euclid(3, -2), 1);
    }

    #[test]
    fn minus_lhs() {
        assert_eq!(euclid(-3, 2), 1);
    }

    #[test]
    fn minus() {
        assert_eq!(euclid(-3, -2), 1);
    }
}

#[allow(dead_code)]
fn euclid(m: i32, n: i32) -> i32 {
    fn euclid(m: i32, n: i32) -> i32 {
        match m % n {
            0 => n,
            r => euclid(n, r),
        }
    }

    if m == 0 && n == 0 {
        panic!("引数のどちらかは 0 以外である必要があります");
    } else if n == 0 {
        m
    } else {
        euclid(m.abs(), n)
    }
}
