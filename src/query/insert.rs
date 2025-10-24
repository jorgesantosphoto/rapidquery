#[derive(Debug, Default)]
pub enum InsertValueSource {
    #[default]
    None,
    // Select(pyo3::Py<pyo3::PyAny>),
    Single(Vec<pyo3::Py<pyo3::PyAny>>),
    Many(Vec<Vec<pyo3::Py<pyo3::PyAny>>>),
}

#[derive(Default)]
pub struct InsertInner {
    pub replace: bool,
    pub table: Option<pyo3::Py<pyo3::PyAny>>,
    pub columns: Vec<String>,
    pub source: InsertValueSource,
    // pub on_conflict: Option<pyo3::Py<pyo3::PyAny>>,
    // pub returning: Option<pyo3::Py<pyo3::PyAny>>,
    pub default_values: Option<u32>,
    // pub with: Option<pyo3::Py<pyo3::PyAny>>,
}

#[pyo3::pyclass(module = "rapidquery._lib", name = "Insert", frozen)]
pub struct PyInsert {
    inner: parking_lot::Mutex<InsertInner>,
}

#[pyo3::pymethods]
impl PyInsert {
    #[new]
    fn new() -> Self {
        Self {
            inner: parking_lot::Mutex::new(Default::default()),
        }
    }

    fn replace(slf: pyo3::PyRef<'_, Self>) -> pyo3::PyRef<'_, Self> {
        {
            let mut lock = slf.inner.lock();
            lock.replace = true;
        }

        slf
    }
}
