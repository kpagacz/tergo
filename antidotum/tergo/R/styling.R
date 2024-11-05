#' Style a package
#'
#' @export
style <- function() {
  # TODO
  # 1. Read a configuration file
  # 2. List files
  # 3. Call formatting
  "styled"
}

#' Style a package
#'
#' @export
#' @param configuration (`list`) the configuration for formatting
style_pkg <- function(configuration = list()) {}

#' Style a file
#'
#' @param configuration (`list`) the configuration for formatting
#' @export
style_file <- function(configuration = list()) {}

#' Style text
#'
#' This function is vectorized.
#'
#' @param text (`character`) the text to format
#' @param configuration (`list`) the configuration for formatting
#' @export
style_text <- function(text, configuration = list()) {}
