use pyo3::ffi::c_str;
use pyo3::prelude::*;
use pyo3::py_run;
use std::ffi::CString;

use python_to_rust::init_python_venv;

fn main() -> PyResult<()> {
    display_py_version()?;
    import_builtin_module()?;
    import_numpy()?;
    eval_expression()?;
    run_py_statement()?;
    from_py_code_file()?;
    Ok(())
}

// ------------- Directly Run String ---------------

pub fn display_py_version() -> PyResult<()> {
    println!("\n[Display Python Version]");
    Python::attach(|py| {
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;

        println!("Python version: {}", version);
        Ok(())
    })
}

pub fn import_builtin_module() -> PyResult<()> {
    println!("\n[Import Builtin Module]");
    Python::attach(|py| {
        let builtins = py.import("builtins")?;
        let total: i32 = builtins
            .getattr("sum")?
            .call1((vec![1, 2, 3],))?
            .extract()?;
        assert_eq!(total, 6);
        println!("builtins.sum(1, 2, 3) = {}", total);
        Ok(())
    })
}

pub fn import_numpy() -> PyResult<()> {
    println!("\n[Import numpy]");
    Python::attach(|py| {
        init_python_venv(py);

        let np = py.import("numpy")?;
        let array = np.call_method1("array", (vec![1, 2, 3],))?;
        let sum1: i32 = np.getattr("sum")?.call1((&array,))?.extract()?;
        assert_eq!(sum1, 6);
        println!("numpy.sum(numpy.array([1, 2, 3])) = {}", sum1);
        let sum2: i32 = array.call_method0("sum")?.extract()?;
        assert_eq!(sum2, 6);
        println!("numpy.array([1, 2, 3]).sum() = {}", sum2);
        Ok(())
    })
}

pub fn eval_expression() -> PyResult<()> {
    println!("\n[Eval Expression]");
    let expr = "[i * 10 for i in range(5)]";
    Python::attach(|py| {
        let cstr_expr = CString::new(expr).unwrap();
        let result: Vec<i64> = py.eval(cstr_expr.as_c_str(), None, None)?.extract()?;
        println!("Evaluated expression {}: {:?}", expr, result);
        Ok(())
    })
}

#[pyclass]
struct UserData {
    id: u32,
    name: String,
}

#[pymethods]
impl UserData {
    fn as_tuple(&self) -> (u32, String) {
        (self.id, self.name.clone())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("User {}(id: {})", self.name, self.id))
    }
}

pub fn run_py_statement() -> PyResult<()> {
    println!("\n[Run Python Statement]");
    Python::attach(|py| {
        let userdata = UserData {
            id: 34,
            name: "Yu".to_string(),
        };
        let userdata = Py::new(py, userdata).unwrap();
        let userdata_as_tuple = (34, "Yu");
        py_run!(py, userdata userdata_as_tuple, r#"
assert repr(userdata) == "User Yu(id: 34)"
assert userdata.as_tuple() == userdata_as_tuple
print(f"User data from Rust: {userdata}")
    "#);
        Ok(())
    })
}

// ------------- Run from Python File ---------------

pub fn from_py_code_file() -> PyResult<()> {
    println!("\n[From Python Code File]");

    Python::attach(|py| {
        let code_path = std::env::current_dir()?.join("src/example.py");
        let code = std::fs::read_to_string(code_path)?;

        let python_file = PyModule::from_code(
            py,
            CString::new(code)?.as_c_str(),
            c_str!("example.py"),
            c_str!("example"),
        )?;

        let add_result: i32 = python_file.getattr("add")?.call1((3, 5))?.extract()?;
        assert_eq!(add_result, 8);

        let factorial_5: u64 = python_file.getattr("factorial")?.call1((5,))?.extract()?;
        assert_eq!(factorial_5, 120);

        let fib_10: u64 = python_file.getattr("fibonacci")?.call1((10,))?.extract()?;
        assert_eq!(fib_10, 55);

        println!(
            "add(3, 5) = {} , factorial(5) = {}, fibonacci(10) = {}",
            add_result, factorial_5, fib_10
        );
        Ok(())
    })
}
