
# using a different form here as it makes more sense with R


updateInventory <- function(old, new) {
  res <- check_inventory(old, new)
  res[order(names(res))]
}

check_inventory <- function(x, y) {
  if (is.null(x)) {
    return(y)
  }

  if (is.null(y)) {
    return(x)
  }

  combined <- c(x, y)
  out <- tapply(combined, names(combined), sum)
  unlist(as.vector(out, "list"))
}


stopifnot(
  identical(
    updateInventory(
      c(`Bowling Ball` = 21, `Dirty Sock` = 2, `Hair Pin` = 1, `Microphone` = 5),
      c(`Hair Pin` = 2, `Half-Eaten Apple` = 3, `Bowling Ball` = 67, `Toothpaste` = 7)
    ),
    c(
      `Bowling Ball` = 88, `Dirty Sock` = 2, `Hair Pin` = 3,
      `Half-Eaten Apple` = 3, `Microphone` = 5, `Toothpaste` = 7
    )
  ),

  identical(
    updateInventory(
      NULL,
      c(`Hair Pin` = 2, `Half-Eaten Apple` = 3, `Bowling Ball` = 67, `Toothpaste` = 7)
    ),
    c(`Bowling Ball` = 67, `Hair Pin` = 2, `Half-Eaten Apple` = 3, `Toothpaste` = 7 )
  ),

  identical(
    updateInventory(
      c(`Bowling Ball` = 0, `Dirty Sock` = 0, `Hair Pin` = 0, `Microphone` = 0),
      c(`Hair Pin` = 1, `Half-Eaten Apple` = 1, `Bowling Ball` = 1, `Toothpaste` = 1)
    ),
    c(
      `Bowling Ball` = 1, `Dirty Sock` = 0, `Hair Pin` = 1,
      `Half-Eaten Apple` = 1, `Microphone` = 0, `Toothpaste` = 1
    )
  )
)
