# nolint start

# This file was created with the following call:
#   .Call("wrap__make_helloextendr_wrappers", use_symbols = TRUE, package_name = "helloextendr")

#' @docType package
#' @usage NULL
#' @useDynLib helloextendr, .registration = TRUE
NULL

#' Return string `"Hello world!"` to R.
#' @export
namedPipe <- function(path, value) .Call(wrap__namedPipe, path, value)
namedPipeAction <- function(path, action, payload) {
  dst <- jsonlite::toJSON( list(type = jsonlite::unbox(action), payload = payload) )
  .Call(wrap__namedPipe, path, dst)
}

readMemoryMappedFile <- function(path) .Call(wrap__readMemoryMappedFile, path)
readMemoryMappedFileFloat <- function(path) .Call(wrap__readMemoryMappedFileFloat, path)
writeMemoryMappedFile <- function(path, index) .Call(wrap__writeMemoryMappedFile, path, index)
writeMemoryMappedFileFloat <- function(path, index) .Call(wrap__writeMemoryMappedFileFloat, path, index)

ggsave2svg <- function(...) .Call(wrap__ggsave2svg, eval(substitute(alist(...))))
as.image.data.frame <- function(src, width, height) .Call(wrap__asImageDataFrame, src, width, height)