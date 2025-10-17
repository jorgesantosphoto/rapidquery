from rapidquery import _lib
import pytest


def test_backend():
    for x in (_lib.SQLiteBackend, _lib.MySQLBackend, _lib.PostgreSQLBackend):
        ty = x()
        assert isinstance(ty, _lib.BackendMeta)

    try:
        _lib.BackendMeta()
    except NotImplementedError:
        pass
    else:
        pytest.fail()
