use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use karlo_rs::*;

#[pymodule]
fn karlo_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(gen_image, m)?)?;
    m.add_function(wrap_pyfunction!(gen_variation, m)?)?;
    Ok(())
}

// #[pyfunction]
// fn img_to_base64_string(img: &DynamicImage) -> PyResult<String> {
//     let mut buffer = Vec::new();
//     let mut cursor = Cursor::new(&mut buffer);

//     img.write_to(&mut cursor, image::ImageOutputFormat::Png)
//         .expect("Failed to write image to buffer");

//     Ok(general_purpose::STANDARD.encode(buffer))
// }

#[pyfunction]
pub fn gen_image<'py>(
    py: Python<'py>,
    prompt: String,
    output_prefix: String,
    api_key: String,
    batch_size: Option<usize>,
) -> PyResult<&'py PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let result = generate_image(&prompt, &output_prefix, &api_key, batch_size).await;
        match result {
            Ok(()) => Ok(Python::with_gil(|py| py.None())),
            Err(err) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Error: {}",
                err
            ))),
        }
    })
}

#[pyfunction]
pub fn gen_variation<'py>(
    py: Python<'py>,
    input_path: String,
    output_prefix: String,
    api_key: String,
    batch_size: Option<usize>,
) -> PyResult<&'py PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let result = generate_variations(&input_path, &output_prefix, &api_key, batch_size).await;
        match result {
            Ok(()) => Ok(Python::with_gil(|py| py.None())),
            Err(err) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Error: {}",
                err
            ))),
        }
    })
}
