pub fn raindrops(n: i32) -> String {

    fn int_to_str(drop: i32) -> String {
        match drop {
            3 => "Pling".to_string(),
            5 => "Plang".to_string(),
            7 => "Plong".to_string(),
            _ => drop.to_string()
        }
    }

    let res = (1..n+1).filter(|x| n % x == 0)
        .filter(|&x| x == 3 || x == 5 || x == 7)
        .map(|x| int_to_str(x))
        .fold("".to_string(), |sum, i| sum + &i);

    if res == "" {
        int_to_str(n)
    } else {
        res
    }
}
