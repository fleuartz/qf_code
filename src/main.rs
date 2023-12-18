mod qf_encode {
    #[derive(Debug, PartialEq)]
    pub enum QFError {
        QFError,
    }

    pub fn encode(notation: &str) -> Result<String, QFError> {
        let b64chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let xalpha = "abcdefghi";
        let mut qf_bin = String::from("01");

        let mvlist: Vec<&str> = notation.split(',').collect();
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
                return Err(QFError::QFError);
            }
        }

        pad_binary_string(&mut qf_bin);
        convert_to_base64(&qf_bin, b64chars).map_err(|_| QFError::QFError)
    }

    fn process_move(
        mv: &str,
        qf_bin: &mut String,
        xalpha: &str,
        turn: &mut usize,
        p: &mut Vec<(&str, i32, &str, i32)>,
    ) -> Result<(), QFError> {
        qf_bin.push('0');

        let newx = xalpha.find(mv.chars().nth(0).unwrap()).ok_or(QFError::QFError)? as i32 + 1;
        let newy = mv.chars().nth(1).and_then(|c| c.to_digit(10)).ok_or(QFError::QFError)? as i32;
        let (oldx, oldy) = (p[*turn].1, p[*turn].3);

        let direction = match (newx - oldx, newy - oldy) {
            (1, 0) | (2, 0) => 2,
            (-1, 0) | (-2, 0) => 6,
            (0, 1) | (0, 2) => 0,
            (0, -1) | (0, -2) => 4,
            _ => return Err(QFError::QFError),
        };

        p[*turn] = ("x", newx, "y", newy);

        let direction_bin = format!("{:03b}", direction);
        qf_bin.push_str(&direction_bin);

        *turn = 1 - *turn;

        Ok(())
    }

    fn process_wall(mv: &str, qf_bin: &mut String, xalpha: &str, turn: &mut usize) -> Result<(), QFError> {
        qf_bin.push('1');

        let direction_bit = if let Some(c) = mv.chars().nth(2) {
            if c == 'h' { '0' } else { '1' }
        } else {
            return Err(QFError::QFError);
        };
        qf_bin.push(direction_bit);

        let x = (xalpha.find(mv.chars().nth(0).ok_or(QFError::QFError)?).ok_or(QFError::QFError)? + 1) as i32;
        let y = mv.chars().nth(1).and_then(|c| c.to_digit(10)).ok_or(QFError::QFError)? as i32;

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
                let index = usize::from_str_radix(chunk.iter().collect::<String>().as_str(), 2).map_err(|_| QFError::QFError)?;
                b64chars.chars().nth(index).ok_or(QFError::QFError)
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|chars| chars.into_iter().collect())?;
        Ok(qf_code)
    }
}

fn main() {
    let notation = "e2,e2v,f2h,e8,h2h,e7,d6h,f7,f6h,e7,d2,d7,c7v,d8,d3,d3h,c3,b3h,a2h,d9,b3,c9,a3,c8,c5v,c2v,h6h,c7,a4,c6,a5,a5h,b5,b6h,a7v,b1v,c5,c4,c6,b4,b6,a4,a6";
    match qf_encode::encode(notation) {
        Ok(qf_code) => println!("{}", qf_code),
        Err(err) => eprintln!("Error: {:?}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::qf_encode;

    #[test]
    fn test_qf_encode() {
        let notation = "e2,d2h,f2,e8,e7v,e7";
        let result = qf_encode::encode(notation);
        assert_eq!(result, Ok("QGCLJPRA".to_string()));
    }

    #[test]
    fn test_qf_encode2() {
        let notation = "e2,e2v,f2h,e8,h2h,e7,d6h,f7,f6h,e7,d2,d7,c7v,d8,d3,d3h,c3,b3h,a2h,d9,b3,c9,a3,c8,c5v,c2v,h6h,c7,a4,c6,a5,a5h,b5,b6h,a7v,b1v,c5,c4,c6,b4,b6,a4,a6";
        let result = qf_encode::encode(notation);
        assert_eq!(result, Ok("QrDMjUj0qyrWZvIAk2kYgGZk4sqvQECgKp8MEkBmZg".to_string()));
    }

    #[test]
    fn test_qf_encode3() {
        let notation = "e2,e2v,e3,d3h,e2,e8,d7v,e7,e6h,f7,d2,f8,e2,f9,e1,g9,d1,f9,e1,e9,f1,f9,e1,e9,f1,f9,e1,e9,f1,f9,e1h,f8,e1,f9,f1,e9,e1,f9,f1,e9,e1,f9,f1,e9,e1,f9,f1,e9,e1,f9,f1,e9,e1,f9,f1,e9,e1,f9,f1,e9,g1,f9,f1,e9,g1,f9,f1,e9,g1,f9,f1,e9,g1,f9,f1,f1v,e1,e9,d1,d9,c1,d8,c2,c8h,b2,b2h,a2,a8h,a3,a3v,a4,a4h,a3,d7,a2,d6,b2,d5,c2,d4,d2,c4,d3,c3,b3,d3,d2h,c3,b4,c4,d4,e4,f3v,e5,e4,f5,g4h,g6h,e5,g5,h1h,h5,f5,i5,g5,i4,g6,h4,h6,h3,i6,g1v,i7,g3,i8,g2,i9";
        let result = qf_encode::encode(notation);
        assert_eq!(result, Ok("SJDMCTRPNKwmAgQmYmImYiZiKERgJmImYiZiJmImYiZiJiJmImYiZiJsVmZmQLpolrgNAJhERCQkJgRii2ACLVAinq4ChyIiQGJCxgYEA".to_string()));
    }

    #[test]
    fn test_qf_encode4() {
        let notation = "aaa,aaaa";
        let result = qf_encode::encode(notation);
        assert_eq!(result, Err(qf_encode::QFError::QFError));
    }
}
