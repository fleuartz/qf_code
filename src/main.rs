mod qf_encode;
mod qf_decode;

fn main() {
    let notation = "e2,e2v,f2h,e8,h2h,e7,d6h,f7,f6h,e7,d2,d7,c7v,d8,d3,d3h,c3,b3h,a2h,d9,b3,c9,a3,c8,c5v,c2v,h6h,c7,a4,c6,a5,a5h,b5,b6h,a7v,b1v,c5,c4,c6,b4,b6,a4,a6";
    match qf_encode::encode(notation) {
        Ok(qf_code) => println!("Encode: ({}) -> {}", notation, qf_code),
        Err(err) => eprintln!("Error: {:?}", err),
    }

    let qf_code = "QrDMjUj0qyrWZvIAk2kYgGZk4sqvQECgKp8MEkBmZg";
    match qf_decode::decode(qf_code) {
        Ok(notation) => println!("Decode: ({}) -> {}", qf_code, notation),
        Err(err) => eprintln!("Error: {:?}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qf_encode1() {
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
        assert_eq!(result, Err(qf_encode::QFError::QFEncodeError));
    }

    #[test]
    fn test_qf_decode1() {
        let notation = "e2,d2h,f2,e8,e7v,e7";
        let result = qf_encode::encode(notation);
        let notation_decoded = qf_decode::decode(&result.unwrap());
        assert_eq!(notation_decoded, Ok(notation.to_string()));
    }

    #[test]
    fn test_qf_decode2() {
        let notation = "e2,e2v,f2h,e8,h2h,e7,d6h,f7,f6h,e7,d2,d7,c7v,d8,d3,d3h,c3,b3h,a2h,d9,b3,c9,a3,c8,c5v,c2v,h6h,c7,a4,c6,a5,a5h,b5,b6h,a7v,b1v,c5,c4,c6,b4,b6,a4,a6";
        let result = qf_encode::encode(notation);
        let notation_decoded = qf_decode::decode(&result.unwrap());
        assert_eq!(notation_decoded, Ok(notation.to_string()));
    }

    #[test]
    fn test_qf_decode3() {
        let notation = "e2,e2v,e3,d3h,e2,e8,d7v,e7,e6h,f7,d2,f8,e2,f9,e1,g9,d1,f9,e1,e9,f1,f9,e1,e9,f1,f9,e1,e9,f1,f9,e1h,f8,e1,f9,f1,e9,e1,f9,f1,e9,e1,f9,f1,e9,e1,f9,f1,e9,e1,f9,f1,e9,e1,f9,f1,e9,e1,f9,f1,e9,g1,f9,f1,e9,g1,f9,f1,e9,g1,f9,f1,e9,g1,f9,f1,f1v,e1,e9,d1,d9,c1,d8,c2,c8h,b2,b2h,a2,a8h,a3,a3v,a4,a4h,a3,d7,a2,d6,b2,d5,c2,d4,d2,c4,d3,c3,b3,d3,d2h,c3,b4,c4,d4,e4,f3v,e5,e4,f5,g4h,g6h,e5,g5,h1h,h5,f5,i5,g5,i4,g6,h4,h6,h3,i6,g1v,i7,g3,i8,g2,i9";
        let result = qf_encode::encode(notation);
        let notation_decoded = qf_decode::decode(&result.unwrap());
        assert_eq!(notation_decoded, Ok(notation.to_string()));
    }

    #[test]
    fn test_qf_decode4() {
        let result = "AAAAAAA".to_string();
        let notation_decoded = qf_decode::decode(&result);
        assert_eq!(notation_decoded, Err(qf_decode::QFError::QFDecodeError));
    }
}
