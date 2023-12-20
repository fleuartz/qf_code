#[derive(Debug, PartialEq)]
pub enum QFError {
    QFEncodeError,
}

pub fn encode(notation: &str) -> Result<String, QFError> {
    let b64chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let xalpha = "abcdefghi";
    let mut qf_bin = String::from("01");

    let mvlist: Vec<&str> = notation.split(',').collect();
    
    if mvlist.len() > 1023 {
        return Err(QFError::QFEncodeError);
    }

    let mvlen_bin = format!("{:010b}", mvlist.len());
    qf_bin.push_str(&mvlen_bin);

    let mut turn = 0;
    let mut p = vec![("x", 5, "y", 1), ("x", 5, "y", 9)];

    for &mv in &mvlist {
        if mv.len() == 2 {
            process_move(mv, &mut qf_bin, xalpha, &mut turn, &mut p)?;
        } else if mv.len() == 3 {
            process_wall(mv, &mut qf_bin, xalpha, &mut turn)?;
        } else {
            return Err(QFError::QFEncodeError);
        }
    }

    pad_binary_string(&mut qf_bin);
    convert_to_base64(&qf_bin, b64chars).map_err(|_| QFError::QFEncodeError)
}

fn process_move(
    mv: &str,
    qf_bin: &mut String,
    xalpha: &str,
    turn: &mut usize,
    p: &mut Vec<(&str, i32, &str, i32)>,
) -> Result<(), QFError> {
    qf_bin.push('0');

    let newx = xalpha.find(mv.chars().nth(0).unwrap()).ok_or(QFError::QFEncodeError)? as i32 + 1;
    let newy = mv.chars().nth(1).and_then(|c| c.to_digit(10)).ok_or(QFError::QFEncodeError)? as i32;
    let (oldx, oldy) = (p[*turn].1, p[*turn].3);

    let direction = match (newx - oldx, newy - oldy) {
        (1, 0) | (2, 0) => 2,
        (-1, 0) | (-2, 0) => 6,
        (0, 1) | (0, 2) => 0,
        (0, -1) | (0, -2) => 4,
        _ => return Err(QFError::QFEncodeError),
    };

    p[*turn] = ("x", newx, "y", newy);

    let direction_bin = format!("{:03b}", direction);
    qf_bin.push_str(&direction_bin);

    *turn = 1 - *turn;

    Ok(())
}

fn process_wall(
    mv: &str, 
    qf_bin: &mut String, 
    xalpha: &str, 
    turn: &mut usize,
) -> Result<(), QFError> {
    qf_bin.push('1');

    let direction_bit = if let Some(c) = mv.chars().nth(2) {
        if c == 'h' { '0' } else { '1' }
    } else {
        return Err(QFError::QFEncodeError);
    };
    qf_bin.push(direction_bit);

    let x = (xalpha.find(mv.chars().nth(0).ok_or(QFError::QFEncodeError)?).ok_or(QFError::QFEncodeError)? + 1) as i32;
    let y = mv.chars().nth(1).and_then(|c| c.to_digit(10)).ok_or(QFError::QFEncodeError)? as i32;

    let wallplace = ((x - 1) + (y - 1) * 8) as usize;

    let wallplace_bin = format!("{:06b}", wallplace);
    qf_bin.push_str(&wallplace_bin);

    *turn = 1 - *turn;

    Ok(())
}

fn pad_binary_string(qf_bin: &mut String) {
    qf_bin.push_str("0".repeat((6 - qf_bin.len() % 6) % 6).as_str());
}

fn convert_to_base64(qf_bin: &String, b64chars: &str) -> Result<String, QFError> {
    let qf_code: String = qf_bin
        .chars()
        .collect::<Vec<char>>()
        .chunks(6)
        .map(|chunk| {
            let index = usize::from_str_radix(chunk.iter().collect::<String>().as_str(), 2).map_err(|_| QFError::QFEncodeError)?;
            b64chars.chars().nth(index).ok_or(QFError::QFEncodeError)
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|chars| chars.into_iter().collect())?;
    Ok(qf_code)
}
