
pairwise <- function(x, n) {
  # x <- as.integer(x)
  # n <- as.integer(n)
  if (!length(x)) {
    return(0L)
  }
  
  xi <- seq_along(x) - 1L
  combx <- combn(x, 2)
  combi <- combn(xi, 2)
  w <- which(apply(combx, 2, sum) == n)
  inds <- as.integer(combi[, w, drop = FALSE])
  
  while ((d <- anyDuplicated(inds, incomparables = NA_integer_)) > 0) {
    d <- c(d, if (d %% 2 == 0) d - 1L else d + 1L)
    inds[d] <- NA_integer_
  }
  
  sum(apply(matrix(inds, nrow = 2), 2, sum), na.rm = TRUE)
}

stopifnot(
  identical(pairwise(c(1, 4, 2, 3, 0, 5), 7), 11L),
  identical(pairwise(c(1, 3, 2, 4), 4), 1L),
  identical(pairwise(c(1, 1, 1), 2), 1L),
  identical(pairwise(c(0, 0, 0, 0, 1, 1), 1), 10L),
  identical(pairwise(c(), 100), 0L)
)
