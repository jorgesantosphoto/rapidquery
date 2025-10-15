use pyo3::types::PyAnyMethods;
use std::collections::HashMap;
use std::sync::Arc;

type ColumnMap = HashMap<String, pyo3::Py<pyo3::PyAny>>;

pub struct TableInner {
    pub name: pyo3::Py<pyo3::PyAny>,
    pub columns: ColumnMap,
    pub indexes: Vec<pyo3::Py<pyo3::PyAny>>,
    pub foreign_keys: Vec<pyo3::Py<pyo3::PyAny>>,
    pub checks: Vec<pyo3::Py<pyo3::PyAny>>,
    pub if_not_exists: bool,
    pub temporary: bool,
    pub comment: Option<String>,
    pub engine: Option<String>,
    pub collate: Option<String>,
    pub character_set: Option<String>,
    pub extra: Option<String>,
}

#[pyo3::pyclass(module = "rapidquery._lib", name = "Table", frozen)]
pub struct PyTable {
    pub inner: Arc<parking_lot::Mutex<TableInner>>,
}

#[pyo3::pymethods]
impl PyTable {
    #[new]
    #[pyo3(
        signature=(
            name,
            columns,
            indexes=Vec::new(),
            foreign_keys=Vec::new(),
            checks=Vec::new(),
            if_not_exists=false,
            temporary=false,
            comment=None,
            engine=None,
            collate=None,
            character_set=None,
            extra=None
        )
    )]
    fn new(
        name: &pyo3::Bound<'_, pyo3::PyAny>,
        columns: Vec<pyo3::Py<pyo3::PyAny>>,
        indexes: Vec<pyo3::Py<pyo3::PyAny>>,
        foreign_keys: Vec<pyo3::Py<pyo3::PyAny>>,
        checks: Vec<pyo3::Py<pyo3::PyAny>>,
        if_not_exists: bool,
        temporary: bool,
        comment: Option<String>,
        engine: Option<String>,
        collate: Option<String>,
        character_set: Option<String>,
        extra: Option<String>,
    ) -> pyo3::PyResult<Self> {
        let py = name.py();

        let name = crate::common::PyTableName::from_pyobject(name)?;
        let name_as_str = unsafe {
            let bound = name.cast_bound_unchecked::<crate::common::PyTableName>(py);

            bound.get().name.clone()
        };

        let mut cols = ColumnMap::with_capacity(columns.len());
        for col in columns {
            if unsafe {
                std::hint::unlikely(pyo3::ffi::Py_TYPE(col.as_ptr()) != crate::typeref::COLUMN_TYPE)
            } {
                return Err(typeerror!("expected Column, got {:?}", py, col.as_ptr()));
            }

            let colbound = unsafe { col.bind(py).cast_unchecked::<crate::column::PyColumn>() };
            let mut collock = colbound.get().inner.lock();

            collock.table = Some(name_as_str.clone());

            let colname = collock.name.clone();
            drop(collock);

            cols.insert(colname, col);
        }

        let mut indexes_vec = Vec::with_capacity(indexes.capacity());
        for ix in indexes {
            if std::hint::unlikely(!ix.bind(py).is_instance_of::<crate::index::PyIndex>()) {
                return Err(typeerror!("expected Index, got {:?}", py, ix.as_ptr()));
            }

            let ixbound = unsafe { ix.bind(py).cast_unchecked::<crate::index::PyIndex>() };
            let mut ixlock = ixbound.get().inner.lock();

            ixlock.table = Some(name.clone_ref(py));
            drop(ixlock);

            indexes_vec.push(ix);
        }

        let mut foreign_keys_vec = Vec::with_capacity(foreign_keys.capacity());
        for fk in foreign_keys {
            if std::hint::unlikely(
                !fk.bind(py)
                    .is_instance_of::<crate::foreign_key::PyForeignKeySpec>(),
            ) {
                return Err(typeerror!(
                    "expected ForeignKeySpec, got {:?}",
                    py,
                    fk.as_ptr()
                ));
            }

            foreign_keys_vec.push(fk);
        }

        let mut checks_vec = Vec::with_capacity(checks.capacity());
        for expr in checks {
            if unsafe { pyo3::ffi::Py_TYPE(expr.as_ptr()) != crate::typeref::EXPR_TYPE } {
                return Err(typeerror!("expected Expr, got {:?}", py, expr.as_ptr()));
            }

            checks_vec.push(expr);
        }

        let inner = TableInner {
            name,
            columns: cols,
            indexes: indexes_vec,
            foreign_keys: foreign_keys_vec,
            checks: checks_vec,
            if_not_exists,
            temporary,
            comment,
            engine,
            collate,
            character_set,
            extra,
        };

        Ok(Self {
            inner: Arc::new(parking_lot::Mutex::new(inner)),
        })
    }

    

    fn __repr__(&self) -> String {
        use std::io::Write;

        let lock = self.inner.lock();
        let mut s = Vec::with_capacity(50);

        write!(s, "<Table name={} columns=[", lock.name).unwrap();

        let n = lock.columns.len() - 1;
        for (index, col) in lock.columns.values().enumerate() {
            if index == n {
                write!(s, "{col}").unwrap();
            } else {
                write!(s, "{col}, ").unwrap();
            }
        }

        write!(s, "] indexes=[").unwrap();

        let n = lock.indexes.len() - 1;
        for (index, ix) in lock.indexes.iter().enumerate() {
            if index == n {
                write!(s, "{ix}").unwrap();
            } else {
                write!(s, "{ix}, ").unwrap();
            }
        }

        write!(s, "] foreign_keys=[").unwrap();

        let n = lock.foreign_keys.len() - 1;
        for (index, fk) in lock.foreign_keys.iter().enumerate() {
            if index == n {
                write!(s, "{fk}").unwrap();
            } else {
                write!(s, "{fk}, ").unwrap();
            }
        }

        write!(s, "]").unwrap();

        if lock.if_not_exists {
            write!(s, " if_not_exists=True").unwrap();
        }
        if lock.temporary {
            write!(s, " temporary=True").unwrap();
        }

        if let Some(x) = &lock.comment {
            write!(s, " comment={x}").unwrap();
        }
        if let Some(x) = &lock.engine {
            write!(s, " engine={x}").unwrap();
        }
        if let Some(x) = &lock.collate {
            write!(s, " collate={x}").unwrap();
        }
        if let Some(x) = &lock.character_set {
            write!(s, " character_set={x}").unwrap();
        }

        write!(s, " checks=[").unwrap();

        let n = lock.checks.len() - 1;
        for (index, ix) in lock.checks.iter().enumerate() {
            if index == n {
                write!(s, "{ix}").unwrap();
            } else {
                write!(s, "{ix}, ").unwrap();
            }
        }

        write!(s, "]>").unwrap();

        unsafe { String::from_utf8_unchecked(s) }
    }
}
