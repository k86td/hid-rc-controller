pub fn steering_wheel_chart(value: usize, width: usize, max: usize, symbol: &str) -> String {
    let left: String;
    let right: String;

    let mid_max = max / 2;

    if value < max / 2 {
        let number = (mid_max - value) * (width / 2) / (mid_max);

        left = symbol.repeat(number);
        right = " ".repeat(number);
    } else {
        let number = (value - mid_max) * (width / 2) / mid_max;

        left = " ".repeat(number);
        right = symbol.repeat(number);
    }

    format!("{}|{}", left, right)
}
