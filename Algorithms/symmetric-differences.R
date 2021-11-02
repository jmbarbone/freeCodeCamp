sym_diff <- function(...) {
  n <- ...length()

  if (n < 2) {
    stop("... must have at least 2 vectors")
  }

  unique(sort.int(Reduce(do_set_diff, list(...))))
}

do_set_diff <- function(x, y) {
  unique(c(x[match(x, y, 0L) == 0L], y[match(y, x, 0L) == 0L]))
}

stopifnot(
  sym_diff(c(1, 2, 3),    c(5, 2, 1, 4))                   |> identical(c(3, 4, 5)),
  sym_diff(c(1, 2, 3, 3), c(5, 2, 1, 4))                   |> identical(c(3, 4, 5)),
  sym_diff(c(1, 2, 3),    c(5, 2, 1, 4, 5))                |> identical(c(3, 4, 5)),
  sym_diff(c(1, 2, 5),    c(2, 3, 5),       c(3, 4, 5))    |> identical(c(1, 4, 5)),
  sym_diff(c(1, 1, 2, 5), c(2, 2, 3, 5),    c(3, 4, 5, 5)) |> identical(c(1, 4, 5)),
  sym_diff(
    c(3, 3, 3, 2, 5),
    c(2, 1, 5, 7),
    c(3, 4, 6, 6),
    c(1, 2, 3),
    c(5, 3, 9, 8),
    c(1)
  ) |> identical(c(1, 2, 4, 5, 6, 7, 8, 9)),
  TRUE
)
