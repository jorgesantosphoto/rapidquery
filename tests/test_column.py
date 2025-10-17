from rapidquery import _lib
import pytest


def test_different_types():
    # Simple
    ty = _lib.IntegerType()
    assert ty == _lib.IntegerType()
    assert repr(ty) == "<IntegerType >"

    # Length
    ty = _lib.StringType(None)
    assert ty == _lib.StringType()
    assert ty.length is None
    assert repr(ty) == "<StringType length=None>"

    ty = _lib.StringType(20)
    assert ty != _lib.StringType(30)
    assert ty != _lib.StringType(None)
    assert ty == _lib.StringType(20)
    assert ty.length == 20
    assert repr(ty) == "<StringType length=20>"

    # Percision Scale
    ty = _lib.MoneyType()
    assert ty == _lib.MoneyType()
    assert ty.precision_scale is None
    assert repr(ty) == "<MoneyType precision_scale=None>"

    ty = _lib.MoneyType((10, 8))
    assert ty != _lib.MoneyType((4, 6))
    assert ty != _lib.MoneyType(None)
    assert ty == _lib.MoneyType((10, 8))
    assert ty.precision_scale == (10, 8)
    assert repr(ty) == "<MoneyType precision_scale=(10, 8)>"

    # Enum
    ty = _lib.EnumType(
        "priority", ["low", "medium"]
    )
    assert ty.name == "priority"
    assert ty.variants == ["low", "medium"]

    assert ty == _lib.EnumType(
        "priority", ["low", "medium"]
    )
    assert ty != _lib.EnumType(
        "priority", ["low", "medium", "high"]
    )

    # Array
    try:
        ty = _lib.ArrayType(str)
    except Exception:
        pass
    else:
        pytest.fail()
    
    ty = _lib.ArrayType(_lib.TextType())
    assert ty.element == _lib.TextType()

    # Interval
    try:
        ty = _lib.IntervalType(5983)
    except Exception:
        pass
    else:
        pytest.fail()

    ty = _lib.IntervalType(_lib.INTERVAL_DAY_TO_MINUTE)
    assert ty.fields == _lib.INTERVAL_DAY_TO_MINUTE
    assert ty.precision is None

    ty = _lib.IntervalType(_lib.INTERVAL_HOUR, 5)
    assert ty.fields == _lib.INTERVAL_HOUR
    assert ty.precision == 5
