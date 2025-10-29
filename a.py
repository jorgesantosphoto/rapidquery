from rapidquery import _lib
import pypika
import timeit


# c = timeit.timeit(
#     """
# str(pypika.Query.into('customers').columns('id', 'fname', 'lname').insert(1, 'Jane', 'Doe'))
# """,
#     globals=globals(),
#     number=100000,
# )
# print(c)

# c = timeit.timeit(
#     """
# _lib.Insert().into("customers").values(id=1, fname="Jane", lname="Doe").to_sql(_lib.PostgreSQLBackend())
# """,
#     globals=globals(),
#     number=100000,
# )
# print(c)

# users = _lib.Table(
#     "users",
#     [
#         _lib.Column("id", _lib.IntegerType(), primary_key=True),
#         _lib.Column("name", _lib.StringType(255)),
#     ],
# )

stmt = (
    _lib.Delete()
        .from_table("users")
        .where(
            _lib.any(
                _lib.Expr.col("id") == 1,
                _lib.Expr.col("name") == "Ali",
            )
        )
        .returning("id")
)
print(stmt.to_sql(_lib.PostgreSQLBackend()))
