// Solution for: https://leetcode.com/problems/car-fleet/

#[test]
fn test_car_fleet() {
    let mut position = vec![10, 8, 0, 5, 3];
    let mut speed = vec![2, 4, 1, 1, 3];
    assert_eq!(car_fleet(12, position, speed), 3);

    position = vec![3];
    speed = vec![3];
    assert_eq!(car_fleet(10, position, speed), 1);

    position = vec![0, 2, 4];
    speed = vec![4, 2, 1];
    assert_eq!(car_fleet(100, position, speed), 1);

    position = vec![6, 8];
    speed = vec![3, 2];
    assert_eq!(car_fleet(10, position, speed), 2);

    position = vec![0, 4, 2];
    speed = vec![2, 1, 3];
    assert_eq!(car_fleet(10, position, speed), 1);
}

fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut cars = position
        .into_iter()
        .zip(speed.into_iter())
        .collect::<Vec<_>>();
    cars.sort_unstable_by(|(a, _), (b, _)| b.cmp(a));

    let mut longest = 0.0;
    let mut count = 0;
    for (position, speed) in cars.into_iter() {
        let time = (target as f32 - position as f32) / speed as f32;
        if time > longest {
            count += 1;
            longest = time;
        }
    }
    count
}
