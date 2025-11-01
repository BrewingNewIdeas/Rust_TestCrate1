pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub fn test_mult(left:u64,right:u64)->u64{
    left*right
}

// use pyo3::prelude::*;


// #[pyfunction]
// fn hello() -> String {
//     "Hello from Rust!".to_string()
// }

// #[pymodule]
// fn pdb_reader(m: &Bound<'_, PyModule>) -> PyResult<()> {
    
//     m.add_function(wrap_pyfunction!(hello, m)?)?;
//     Ok(())
// }