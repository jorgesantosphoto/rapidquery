from rapidquery import _lib
import timeit


backend = _lib.PostgreSQLBackend()

tb = _lib.Table(
    "users",
    columns=[
        _lib.Column("id", _lib.BigIntegerType(), primary_key=True, auto_increment=True),
        _lib.Column("name", _lib.CharType(64), nullable=False),
        _lib.Column("file_id", _lib.BigIntegerType(), nullable=True),
    ],
    indexes=[
        _lib.Index(["name"]),
    ],
    foreign_keys=[
        _lib.ForeignKeySpec(from_columns=["file_id"], to_columns=["id"], to_table="files")
    ],
    checks=[_lib.Expr.col("name") == _lib.Expr("admin")],
    if_not_exists=True,
)

c = timeit.timeit(
"""
expr = _lib.all(
    tb.c.id == _lib.Expr(1),
    tb.c.name.like("%ali%"),
    tb.c.file_id.is_not_null(),
)
expr.build(backend)
""",
globals=globals(),
number=100000,
)
print(c)
