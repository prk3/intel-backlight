fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => {
            print_help_message();
            std::process::exit(0);
        }
        2 => {
            let value = &args[1];
            if let Some(value) = value.strip_prefix('-') {
                let percent = parse_percent(value).unwrap_or_else(|_| {
                    print_help_message();
                    std::process::exit(1)
                });
                decrease_brightness_by_percent(percent).unwrap_or_else(|error| {
                    print_error_message(error);
                    std::process::exit(2)
                });
            } else if let Some(value) = value.strip_prefix('+') {
                let percent = parse_percent(value).unwrap_or_else(|_| {
                    print_help_message();
                    std::process::exit(1)
                });
                increase_brightness_by_percent(percent).unwrap_or_else(|error| {
                    print_error_message(error);
                    std::process::exit(2)
                });
            } else {
                let percent = parse_percent(&value[..]).unwrap_or_else(|_| {
                    print_help_message();
                    std::process::exit(1)
                });
                set_brightness_percent(percent).unwrap_or_else(|error| {
                    print_error_message(error);
                    std::process::exit(2)
                });
            }
        }
        _ => {
            print_help_message();
            std::process::exit(1);
        }
    }
}

fn print_help_message() {
    println!(
        "{}",
        "
Usage:
intel-backlight 10      # set backlight brightness to 10%
intel-backlight +10     # increase backlight brightness by 10% of the max value
intel-backlight -10     # decrease backlight brightness by 10% of the max value
intel-backlight         # print help message
"
        .trim()
    );
}

fn print_error_message(error: &str) {
    println!("Could not change backlight brightness: {}", error);
}

fn parse_percent(string: &str) -> Result<u64, ()> {
    let percent: u64 = string.parse().map_err(|_| ())?;
    Ok(std::cmp::min(percent, 100))
}

fn get_current_min_max_brightness() -> Result<(u64, u64, u64), &'static str> {
    let max_str = std::fs::read_to_string("/sys/class/backlight/intel_backlight/max_brightness")
        .map_err(|_| "Could not read max brightness")?;
    let max: u64 = max_str
        .trim()
        .parse()
        .map_err(|_| "Could not parse max brightness")?;

    let current_str =
        std::fs::read_to_string("/sys/class/backlight/intel_backlight/actual_brightness")
            .map_err(|_| "Could not read current brightness")?;
    let current: u64 = current_str
        .trim()
        .parse()
        .map_err(|_| "Could not parse current brightness")?;

    Ok((current, 0, max))
}

fn decrease_brightness_by_percent(percent: u64) -> Result<(), &'static str> {
    let (current, min, max) = get_current_min_max_brightness()?;
    let mut new = current.saturating_sub(percent.saturating_mul(max - min) / 100);
    if new < min {
        new = min;
    }
    set_brightness(new)?;
    Ok(())
}

fn increase_brightness_by_percent(percent: u64) -> Result<(), &'static str> {
    let (current, min, max) = get_current_min_max_brightness()?;
    let mut new = current.saturating_add(percent.saturating_mul(max - min) / 100);
    if new > max {
        new = max;
    }
    set_brightness(new)?;
    Ok(())
}

fn set_brightness_percent(percent: u64) -> Result<(), &'static str> {
    let (_current, min, max) = get_current_min_max_brightness()?;
    let new = percent.saturating_mul(max - min) / 100;
    set_brightness(new)?;
    Ok(())
}

fn set_brightness(value: u64) -> Result<(), &'static str> {
    std::fs::write(
        "/sys/class/backlight/intel_backlight/brightness",
        value.to_string(),
    )
    .map_err(|_| "Could not write new brightness")?;
    Ok(())
}
