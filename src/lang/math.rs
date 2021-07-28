use super::datatype::BoxInt;

pub fn modulo<T: BoxInt>(a: T, b: T) -> Result<T, String> {
    if b.is_zero() {
        return Err("Cannot use 0 as a modulus".to_string());
    }

    if a * b < T::zero() {
        Ok(b + a % b)
    } else {
        Ok(a % b)
    }
}

pub fn inv_modulo<T: BoxInt>(a: T, b: T) -> Result<T, String> {
    let x = modulo(a, b)?;
    let mut n = T::one();
    while n < b {
        let mod_result = modulo(n * x, b);
        if mod_result.is_ok() && mod_result.unwrap().is_one() {
            return Ok(n);
        }

        n = n + T::one();
    }

    return Err(format!("{} is not invertible", a));
}

pub fn divide<T: BoxInt>(a: T, b: T) -> Result<T, String> {
    if b.is_zero() {
        Err("Cannot use 0 as a divisor".to_string())
    } else {
        Ok(a / b)
    }
}
