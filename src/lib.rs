mod c_mx;

pub fn transpose(a: &mut [f64], side: i32) -> Vec<f64> {
    let output: &mut [f64] = &mut vec![0.0; side as usize * side as usize];
    unsafe {
        c_mx::transpose(a.as_mut_ptr(), output.as_mut_ptr(), side);
    }

    return output.to_vec();
}

pub fn multiply(a: &mut [f64], b: &mut [f64]) -> Vec<f64> {
    let side = (a.len() as f64).sqrt() as usize;
    let output: &mut [f64] = &mut vec![0.0; side * side];
    unsafe {
        c_mx::multiply(a.as_mut_ptr(), b.as_mut_ptr(), output.as_mut_ptr(), side as i32);
    }

    return output.to_vec();
}

pub fn multiply_l(a: &mut [f64], b: &mut [f64]) -> Vec<f64> {
    let side = (a.len() as f64).sqrt() as usize;
    let output: &mut [f64] = &mut vec![0.0; side * side];
    unsafe {
        c_mx::multiply_l(a.as_mut_ptr(), b.as_mut_ptr(), output.as_mut_ptr(), side as i32);
    }

    return output.to_vec();
}

pub fn multiply_pf(a: &mut [f64], b: &mut [f64]) -> Vec<f64> {
    let side = (a.len() as f64).sqrt() as usize;
    let output: &mut [f64] = &mut vec![0.0; side * side];
    unsafe {
        c_mx::multiply_pf(a.as_mut_ptr(), b.as_mut_ptr(), output.as_mut_ptr(), side as i32);
    }

    return output.to_vec();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        let mut a = [2.0, 3.0, 4.0, 5.0];
	let mut b = [6.0, 7.0, 8.0, 9.0];

        let expected = [36.0, 41.0, 64.0, 73.0].to_vec();
        let actual = multiply(&mut a, &mut b);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_multiply_l() {
        let mut a = [2.0, 3.0, 4.0, 5.0];
	let mut b = [6.0, 7.0, 8.0, 9.0];

        let expected = [36.0, 41.0, 64.0, 73.0].to_vec();
        let actual = multiply_l(&mut a, &mut b);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_multiply_pf() {
        let mut a = [2.0, 3.0, 4.0, 5.0];
	let mut b = [6.0, 7.0, 8.0, 9.0];

        let expected = [36.0, 41.0, 64.0, 73.0].to_vec();
        let actual = multiply_pf(&mut a, &mut b);

        assert_eq!(expected, actual);
    }

}
