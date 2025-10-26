use pyo3::types::{PyAnyMethods, PyDictMethods};

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

    fn into<'a>(
        slf: pyo3::PyRef<'a, Self>,
        table: &'a pyo3::Bound<'_, pyo3::PyAny>,
    ) -> pyo3::PyResult<pyo3::PyRef<'a, Self>> {
        let table = {
            if let Ok(x) = table.cast_exact::<crate::table::PyTable>() {
                let guard = x.get().inner.lock();
                guard.name.clone_ref(slf.py())
            } else {
                crate::common::PyTableName::from_pyobject(table)?
            }
        };

        {
            let mut lock = slf.inner.lock();
            lock.table = Some(table);
        }

        Ok(slf)
    }

    #[pyo3(signature=(**kwds))]
    fn values<'a>(
        slf: pyo3::PyRef<'a, Self>,
        kwds: Option<&'a pyo3::Bound<'_, pyo3::types::PyDict>>,
    ) -> pyo3::PyResult<pyo3::PyRef<'a, Self>> {
        let kwds = kwds.ok_or_else(|| {
            pyo3::PyErr::new::<pyo3::exceptions::PyTypeError, _>("expected at list one paremeter")
        })?;

        {
            let lock = slf.inner.lock();

            if !lock.columns.is_empty() && lock.columns.len() != kwds.len() {
                return Err(
                    pyo3::PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        "values length isn't equal to columns length - this occurres when you're calling `.values()` method multiple times with different columns."
                    )
                );
            }
        }

        let mut cols = Vec::<String>::new();
        let mut vals = Vec::<pyo3::Py<pyo3::PyAny>>::new();

        unsafe {
            for (key, value) in kwds.iter() {
                let key = key.extract::<String>().unwrap_unchecked();
                cols.push(key.clone());

                let value_type_ptr = pyo3::ffi::Py_TYPE(value.as_ptr());

                if value_type_ptr == crate::typeref::EXPR_TYPE {
                    // Fast path for PyExpr type
                    vals.push(value.unbind());
                    continue;
                }

                let value = crate::expression::PyExpr::try_from(value)?;
                vals.push(pyo3::Py::new(slf.py(), value).unwrap().into_any());
            }
        }

        {
            let mut lock = slf.inner.lock();

            match std::mem::take(&mut lock.source) {
                InsertValueSource::None => {
                    lock.source = InsertValueSource::Single(vals);
                    lock.columns = cols;
                }
                InsertValueSource::Single(oldvals) => {
                    lock.source = InsertValueSource::Many(vec![oldvals, vals]);
                }
                InsertValueSource::Many(mut arr_of_vals) => {
                    arr_of_vals.push(vals);
                    lock.source = InsertValueSource::Many(arr_of_vals);
                }
            }

            println!("{:?}\n{:?}", lock.columns, lock.source)
        }

        Ok(slf)
    }
}
