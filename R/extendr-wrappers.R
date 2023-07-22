# nolint start

# This file was created with the following call:
#   .Call("wrap__make_helloextendr_wrappers", use_symbols = TRUE, package_name = "helloextendr")

#' @docType package
#' @usage NULL
#' @useDynLib helloextendr, .registration = TRUE
NULL

#' Return string `"Hello world!"` to R.
#' @export
named_pipe <- function(x, y) .Call(wrap__named_pipe, x, y)
memory_mapped_file <- function(x) .Call(wrap__memory_mapped_file, x)
read_memory_mapped_file <- function(x) .Call(wrap__read_memory_mapped_file, x)
write_memory_mapped_file <- function(x, y) .Call(wrap__write_memory_mapped_file, x, y)