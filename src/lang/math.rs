use super::interpreter::BoxInt;

pub fn modulo<T: BoxInt>(a: T, b: T) -> Result<T, String> {
    if b.is_zero() {
        return Err("Modulo caused invalid value".to_string());
    }

    if a.checked_mul(&b).ok_or("Modulo caused invalid value")? < T::zero() {
        Ok(b.checked_add(&(a % b))
            .ok_or("Modulo caused invalid value")?)
    } else {
        Ok(a % b)
    }
}

pub fn inv_modulo<T: BoxInt>(a: T, b: T) -> Result<T, String> {
    let x = modulo(a, b)?;
    let mut n = T::one();
    while n < b {
        let mod_result = modulo(
            n.checked_mul(&x)
                .ok_or("Inverse modulo caused invalid value")?,
            b,
        );
        if mod_result.is_ok() && mod_result.unwrap().is_one() {
            return Ok(n);
        }

        n = n + T::one();
    }

    return Err(format!("{} is not invertible", a));
}
