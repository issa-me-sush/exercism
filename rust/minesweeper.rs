pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result = Vec::new();
    for (y, row) in minefield.iter().enumerate() {
        let mut new_row = String::new();
        for (x, ch) in row.chars().enumerate() {
            if ch == '*' {
                // If the current cell is a mine, mark it with an asterisk
                new_row.push('*');
            } else {
                // Otherwise, count the number of mines in the adjacent cells
                let count = count_adjacent_mines(minefield, x, y);
                if count == 0 {
                    // If there are no adjacent mines, leave the cell blank
                    new_row.push(' ');
                } else {
                    // Otherwise, write the count as a digit
                    new_row.push(char::from_digit(count, 10).unwrap());
                }
            }
        }
        result.push(new_row);
    }
    result
}

fn count_adjacent_mines(minefield: &[&str], x: usize, y: usize) -> u32 {
    let mut count = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && ny >= 0 {
                let nx = nx as usize;
                let ny = ny as usize;
                if ny < minefield.len() && nx < minefield[ny].len() {
                    if minefield[ny].chars().nth(nx).unwrap() == '*' {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}
