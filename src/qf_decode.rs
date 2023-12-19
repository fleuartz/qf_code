#[derive(Debug, PartialEq)]
pub enum QFError {
    QFError,
}

pub fn decode(qf_code: &str) -> Result<String, QFError> {
    let b64chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let xalpha = "abcdefghi";

    let mut qf_bin = qf_code
        .chars()
        .map(|c| b64chars.find(c).ok_or(QFError::QFError))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .map(|&index| format!("{:06b}", index))
        .collect::<String>();

    let state_bit = qf_bin.remove(0);
    let record_bit = qf_bin.remove(0);

    if state_bit == '0' && record_bit == '1' {
        let mut movelist = Vec::new();
        let mut turn = 0;
        let mut p = vec![("x", 5, "y", 1), ("x", 5, "y", 9)];

        let mvlen_bin = qf_bin.drain(..10).collect::<String>();
        let mvlen = usize::from_str_radix(mvlen_bin.as_str(), 2).map_err(|_| QFError::QFError)?;

        for _ in 0..mvlen {
            if qf_bin.remove(0) == '0' {
                process_move(&mut qf_bin, &mut movelist, xalpha, &mut turn, &mut p)?;
            } else {
                process_wall(&mut qf_bin, &mut movelist, xalpha, &mut turn)?;
            }
        }

        Ok(movelist.join(","))
    } else {
        Err(QFError::QFError)
    }
}

fn process_move(
    qf_bin: &mut String,
    movelist: &mut Vec<String>,
    xalpha: &str,
    turn: &mut usize,
    p: &mut Vec<(&str, i32, &str, i32)>,
) -> Result<(), QFError> {
    let direction_bin = qf_bin.drain(..3).collect::<String>();
    let direction = usize::from_str_radix(direction_bin.as_str(), 2).map_err(|_| QFError::QFError)?;

    let (oldx, oldy) = (p[*turn].1, p[*turn].3);
    let (dx, dy) = match direction {
        2 => (1, 0),
        6 => (-1, 0),
        0 => (0, 1),
        4 => (0, -1),
        _ => return Err(QFError::QFError),
    };

    let (newx, newy) = (oldx + dx, oldy + dy);

    if p[1 - *turn].1 == newx && p[1 - *turn].3 == newy {
        let (newx, newy) = (newx + dx, newy + dy);
        p[*turn] = ("x", newx, "y", newy);
        let mv = format!("{}{}", xalpha.chars().nth((newx - 1) as usize).ok_or(QFError::QFError)?, newy);
        movelist.push(mv);
    } else {
        p[*turn] = ("x", newx, "y", newy);
        let mv = format!("{}{}", xalpha.chars().nth((newx - 1) as usize).ok_or(QFError::QFError)?, newy);
        movelist.push(mv);
    }

    *turn = 1 - *turn;

    Ok(())
}

fn process_wall(
    qf_bin: &mut String,
    movelist: &mut Vec<String>,
    xalpha: &str,
    turn: &mut usize,
) -> Result<(), QFError> {
    let direction_bit = qf_bin.remove(0);
    let direction = if direction_bit == '0' { 'h' } else { 'v' };

    let wallplace_bin = qf_bin.drain(..6).collect::<String>();
    let wallplace = usize::from_str_radix(wallplace_bin.as_str(), 2).map_err(|_| QFError::QFError)?;

    let x = (wallplace % 8) + 1;
    let y = (wallplace / 8) + 1;

    let mv = format!("{}{}{}", xalpha.chars().nth(x - 1).ok_or(QFError::QFError)?, y, direction);
    movelist.push(mv);

    *turn = 1 - *turn;

    Ok(())
}
