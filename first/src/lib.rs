// 確実な理解のためには実際にプラグラムを書いて動かしてみる
// まず疑似コードのレベルでアルゴリズムを設計する

// このような時間の短縮は小手先のコーディングの工夫やハードウェアの性能向上では実現することが不可能であり，
// アルゴリズムを適切に設計することが重要であることがよくわかる

// アルゴリズムの評価基準としては，「できるだけシンプルなもの」や「できるだけ早いもの」といったものが考えられる
// 前者は理解しやすくプログラムしやい
// その結果実装時にバグの混入を防ぐことや必要に応じて改変することが容易になるというメリットがある
// 後者は大量の情報を繰り返し処理する場合に要求される要件である
// アルゴリズムの石器や選択にはこのような特性を理解し適切なものを選択することが重要

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
