import rapidquery as rq


stmt = (
    rq.Select(rq.Expr.col("id"))
        .from_table("users")
        .lock("exclusive", tables=["tb"], behavior="skip")
        .group_by(rq.Expr.col("name"))
)

print(stmt.to_sql("postgresql"))
