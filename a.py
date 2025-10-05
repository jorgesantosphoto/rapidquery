from rapidquery import _lib
import timeit


c = timeit.timeit(
    "_lib.AdaptedValue(2)",
    globals=globals(),
    number=100000,
)
print(c)
