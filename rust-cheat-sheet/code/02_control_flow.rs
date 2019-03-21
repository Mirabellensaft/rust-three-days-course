fn main() {
    let value = 2;

    if value % 2 == 0 {
        // ...
    } else if value == 5 {
        // ...
    } else { /* ... */ }
}

fn main() {
    let maybe_value = Some(2);
    match maybe_value {
        Some(value) if value == 2 => {
            // ...
        }
        Some(value) => {
            // ...
        },
        None => {
            // ...
        },
    }
}

fn main() {
    let maybe_value = Some(2);

    if let Some(value) = maybe_value {
        // ...
    } else { /* ... */ }
}

fn main() {
    let mut value = 0;
    // Loop with break
    loop {
        if value >= 10 {
            break;
        }
        value += 1;
    }
    // Break on conditional
    while value <= 10 {
        value += 1;
        // ...
    }
}

fn main() {
    // Loop over iterator
    let range = 0..10;
    for i in range {
        // ...
    }
    // while let
    let mut range = 0..10;
    while let Some(v) = range.next() {
        // ...
    }
}
