use std::process::Output;

use pyo3::prelude::*;

/// calculates fibonacci sequence
#[pyfunction]
fn fib_rs(a: u32) -> u32 {
    if a <= 1 {
        return 1;
    } else {
        return fib_rs(a - 1) + fib_rs(a - 2);
    }
}

/// return a vector of two elements
#[pyfunction]
fn make_vec(a: u8, b: u8) -> Vec<u8> {
    vec![a, b]
}

/// return a generic vector of two elements
#[pyfunction]
fn make_generic(a: PyObject, b: PyObject) -> Vec<PyObject> {
    vec![a, b]
}

#[pyfunction]
fn sq(n: PyAny) -> PyResult<PyAny> {
    let val: Result<_, _> = n.try_into();
    match val {
        Ok(val) => square(val),
        Err(e) => Err(e),
    }
}

fn square<T>(n: T) -> T
where
    T: std::ops::Mul<Output = T>,
{
    n * n
}

/// A Python module implemented in Rust.
#[pymodule]
fn word_counter(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fib_rs, m)?)?;
    m.add_function(wrap_pyfunction!(make_vec, m)?)?;
    m.add_function(wrap_pyfunction!(make_generic, m)?)?;
    Ok(())
}

#[cfg(test)]
mod fib_tests {
    use super::fib_rs;

    #[test]
    fn test_fib_rs() {
        assert_eq!(fib_rs(0), 1);
        assert_eq!(fib_rs(1), 1);
        assert_eq!(fib_rs(2), 2);
        assert_eq!(fib_rs(3), 3);
        assert_eq!(fib_rs(4), 5);
        assert_eq!(fib_rs(5), 8);
        assert_eq!(fib_rs(6), 13);
        assert_eq!(fib_rs(10), 89);
    }
}
