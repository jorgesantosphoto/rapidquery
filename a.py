from rapidquery import _lib
import pypika
import timeit


c = timeit.timeit(
    """
str(pypika.Query.from_("users").limit(2).where(pypika.Field("id") > 20).delete())
""",
    globals=globals(),
    number=100000,
)
print(c)

c = timeit.timeit(
    """
_lib.Delete().from_table("users").where(_lib.Expr.col("id") > 20).limit(2).to_sql("postgresql")
""",
    globals=globals(),
    number=100000,
)

print(c)
