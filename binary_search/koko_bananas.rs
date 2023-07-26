// Solution for https://leetcode.com/problems/koko-eating-bananas

#[test]
fn test_min_eating_speed() {
    let piles = vec![3, 6, 7, 11];
    let h = 8;
    assert_eq!(min_eating_speed(piles, h), 4);

    let piles = vec![30, 11, 23, 4, 20];
    let h = 5;
    assert_eq!(min_eating_speed(piles.clone(), h), 30);

    let h = 6;
    assert_eq!(min_eating_speed(piles.clone(), h), 23);
}

fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut upper = *piles.iter().max().unwrap();
    if (piles.len() as i32) == h {
        return upper;
    }

    let mut lower = 1;
    while lower < upper {
        let k = (upper + lower) / 2;
        let hours = piles.iter().fold(0, |acc, val| acc + ((val + k - 1) / k));

        if hours <= h {
            upper = k;
        } else {
            lower = k + 1;
        }
    }
    upper
}
