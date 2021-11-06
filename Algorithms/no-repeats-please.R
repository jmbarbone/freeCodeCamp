

perm_alone <- function(x) {
    letters <- strsplit(x, "")[[1]]
    n <- length(letters)
    
    if (n == 1L) {
        return(1L)
    }

    if (length(unique(letters)) == 1L) {
        return(0L)
    }

    # returns a matrix of all permuations 
    perms <- do.call(rbind, combinat::permn(letters))
    # rle gets the run length -- so counts when values are repeat
    sum(apply(perms, 1, function(i) all(rle(i)$lengths == 1L)))
}

stopifnot(
    identical(perm_alone("aab"), 2L),
    identical(perm_alone("aaa"), 0L),
    identical(perm_alone("aabb"), 8L),
    identical(perm_alone("abcdefa"), 3600L),
    identical(perm_alone("zzzzzzzz"), 0L),
    identical(perm_alone("a"), 1L),
    identical(perm_alone("aaab"), 0L),
    identical(perm_alone("aaabb"), 12L)
)
