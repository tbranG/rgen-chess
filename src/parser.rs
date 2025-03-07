use regex::Regex;

fn preprocess_string(input: &str) -> bool {
    let reg = Regex::new(r"\[(\[([0-9],?){8}\],?){8}\]");
    reg.unwrap().is_match(input)
}

pub fn read_board(input: &String, board: &mut [[i8; 8]; 8]) {
    assert!(preprocess_string(input.as_str()));

    let mut i: i8 = 0;
    let mut j: i8 = 0;

    let mut tmp_num: String = String::new();

    for c in input.chars() {
        match c {
            '['  => j = 0,
            ']' => {
                if i >= 8 {
                    break;
                }

                if !tmp_num.is_empty() {
                    board[i as usize][j as usize] = tmp_num.parse().unwrap();
                    tmp_num.clear();
                }

                i += 1;
                j = 0;
            },
            ',' => {
                if !tmp_num.is_empty() {
                    board[i as usize][j as usize] = tmp_num.parse().unwrap();
                    tmp_num.clear();
                }

                j += 1;
            },
            _ => tmp_num.push(c)
        }
    }
}