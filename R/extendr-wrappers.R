# nolint start

# This file was created with the following call:
#   .Call("wrap__make_helloextendr_wrappers", use_symbols = TRUE, package_name = "helloextendr")

#' @docType package
#' @usage NULL
#' @useDynLib helloextendr, .registration = TRUE
NULL

#' Return string `"Hello world!"` to R.
#' @export
namedPipe <- function(x, y) .Call(wrap__namedPipe, x, y)
readMemoryMappedFile <- function(x) .Call(wrap__readMemoryMappedFile, x)
readMemoryMappedFileFloat <- function(x) .Call(wrap__readMemoryMappedFileFloat, x)
writeMemoryMappedFile <- function(x, y) .Call(wrap__writeMemoryMappedFile, x, y)
writeMemoryMappedFileFloat <- function(x, y) .Call(wrap__writeMemoryMappedFileFloat, x, y)