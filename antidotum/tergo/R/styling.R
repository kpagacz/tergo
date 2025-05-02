#' Style a package
#'
#' @details
#' Configuration is read from a file named `tergo.toml` in the root of the
#' package. The precedence of the configuration is (from the highest to lowest):
#' 1. The configuration passed to the function.
#' 2. The configuration file.
#'
#' To see possible configuration options, see [get_default_config()].
#'
#' @param config_file (`character`) The path to the configuration file. Default `"tergo.toml"`.
#' @param configuration (`list`) Configuration for formatting. Default `list()`.
#' @param ... additional parameters to [tergo::style_pkg()]
#' @return No return value, called for side effects.
#'
#' @export
#' @examples
#' style()
#' style(config_file = "tergo.toml", configuration = list())
#'
style <- function(config_file = "tergo.toml", configuration = list(), ...) {
  style_pkg(path = getwd(), config_file = config_file, configuration = configuration)
  invisible(NULL)
}

#' Style a package
#'
#' @details
#' Configuration is read from a file named `tergo.toml` in the root of the
#' package. The precedence of the configuration is (from the highest to lowest):
#' 1. The configuration passed to the function.
#' 2. The configuration file.
#'
#' To see possible configuration options, see [get_default_config()].
#'
#' @inheritParams style
#' @param path (`character`) The path to the package. Default `"."`.
#' @param force (`logical(1)`) Whether to format the files even
#' if no package was found. `TRUE` - format the `.R` and `.r` files
#' found in the directory (recursive). `FALSE` exit without formatting
#' anything. Default `FALSE`.
#' @param extensions (`character`) The extensions of the files to format. Default `c(".R", ".r")`.
#' @param verbose (`logical(1)`) Whether per file status and run statistics should be printed. Default `interactive()`.
#' @return No return values, called for side effects.
#'
#' @export
#' @examples
#' style_pkg()
#' style_pkg(path = "./tergo", config_file = "custom_tergo.toml", verbose = TRUE)
#'
style_pkg <- function(path = ".",
                      config_file = "tergo.toml",
                      configuration = list(),
                      force = FALSE,
                      extensions = c(".R", ".r"),
                      verbose = interactive()) {
  if (!is.character(path) || length(path) != 1) {
    stop("Path must be a single character string.")
  }
  if (!is.character(config_file) || length(config_file) != 1) {
    stop("Config file must be a single character string.")
  }
  if (!is.logical(force) || length(force) != 1) {
    stop("Force must be a single logical value.")
  }
  if (!is.list(configuration)) {
    stop("Configuration must be a list.")
  }
  if (!is.logical(verbose) || length(verbose) != 1) {
    stop("verbose must be a single logical value.")
  }

  # Read Configuration File
  wd <- path
  config <- NULL

  repeat {
    config_path <- file.path(wd, config_file)
    if (file.exists(config_path)) {
      config <- config_path
      break
    }
    # Stop if at the root directory
    if (dirname(wd) == wd) {
      break
    }
    wd <- dirname(wd)
  }

  if (!is.null(config)) {
    config <- get_config(config)
  } else {
    config <- get_default_config()
  }
  config[names(configuration)] <- configuration

  # Find Package Root
  package_root <- path
  while (!file.exists(file.path(package_root, "DESCRIPTION"))) {
    parent_dir <- dirname(package_root)
    if (parent_dir == package_root) { # Reached root without finding DESCRIPTION
      package_root <- getwd()
      break
    }
    package_root <- parent_dir
  }

  if (!file.exists(file.path(package_root, "DESCRIPTION")) && !force) {
    message("No package detected. Exiting without formatting anything.")
    return(invisible())
  }

  # List Relevant Files
  files <- list.files(package_root, recursive = TRUE, full.names = TRUE)
  files <- Filter(function(file) any(endsWith(file, extensions)), files)

  # Define ANSI Color Codes and Unicode Symbols
  green_tick <- get_printed_symbol(symbol = "\u001B[32m\u2714\u001B[0m", fallback_symbol = "v")
  red_cross <- get_printed_symbol(symbol = "\u001B[31m\u274C\u001B[0m", fallback_symbol = "x")
  # Define ANSI Color Codes and Unicode Symbols for a yellow dot
  yellow_dot <- get_printed_symbol(symbol = "\u001B[33m\u2022\u001B[0m", fallback_symbol = "*")

  ignored_paths <- vapply(
    config$exclusion_list,
    function(ignored_path) {
      if (startsWith(ignored_path, "./")) {
        ignored_path <- substr(ignored_path, 3, nchar(ignored_path))
      }
      file.path(path, ignored_path)
    },
    FUN.VALUE = character(1),
    USE.NAMES = FALSE
  )
  success_count <- 0
  skipped_count <- 0
  for (file in files) {
    tryCatch(
      {
        succes <- style_file_internal(file, config, ignored_paths)
        if (succes) {
          success_count <- success_count + 1
          if (verbose) cat(sprintf("%s %s\n", file, green_tick))
        } else {
          skipped_count <- skipped_count + 1
          if (verbose) cat(sprintf("%s %s\n", file, yellow_dot))
        }
      },
      error = function(err) {
        # Print File Path, Red Cross, and Error Message
        if (verbose) cat(sprintf("%s %s : %s\n", basename(file), red_cross, truncate_error(err$message)))
      }
    )
  }

  if (verbose) {
    summary_bullet <- get_printed_symbol(
      symbol = "\u25B6", # Black Right-Pointing Triangle
      fallback_symbol = ">"
    )
    cat("\nSummary:\n")
    cat(sprintf("  %s Files processed : %d\n", summary_bullet, length(files)))
    cat(sprintf("  %s Successful      : %d\n", green_tick, success_count))
    cat(sprintf("  %s Skipped         : %d\n", yellow_dot, skipped_count))
    cat(sprintf("  %s Failed          : %d\n", red_cross, length(files) - success_count - skipped_count))
  }

  invisible(NULL)
}

#' Style a file
#'
#' @details
#' To see possible configuration options, see [get_default_config()].
#'
#' @inheritParams style
#' @param file (`character`) path to the file to format.
#' @return (`logical`) whether the file was formatted successfully
#' or skipped. `TRUE` - formatted successfully, `FALSE` - skipped.
#'
#' @export
#' @examples
#' tmp <- tempfile()
#' file_conn <- file(tmp)
#' writeLines(c("function(){}", "A<-7"), file_conn)
#' close(file_conn)
#' style_file(file = tmp, configuration = list())
#' unlink(tmp)
#'
style_file <- function(file, configuration = list()) {
  ignored_paths <- configuration$exclusion_list
  if (!is.null(ignored_paths)) {
    if (any(Map(function(ignored_path) startsWith(file, ignored_path), ignored_paths))) {
      return(FALSE)
    }
  }
  if (!file.exists(file)) {
    stop("File " + file + " does not exist")
  }
  size <- file.info(file)$size
  code <- readChar(con = file, nchars = size)
  formatted <- format_code(code, configuration)
  if (formatted[[1]] == "success") {
    formatted[[2]]
  } else {
    stop("Failed to style the file. Error: ", truncate_error(formatted[[2]]))
  }
  write(x = formatted[[2]], file = file)
  TRUE
}

#' Check whether a path is in ignored paths
#' @return (`logical`) whether the path is in the ignored paths.
#' @keywords internal
is_in_ignored_paths <- function(path, ignored_paths) {
  if (!is.null(ignored_paths)) {
    if (
      any(
        vapply(
          ignored_paths,
          FUN = function(ignored_path) startsWith(path, ignored_path),
          FUN.VALUE = logical(1),
          USE.NAMES = FALSE
        )
      )
    ) {
      return(TRUE)
    }
  }
  FALSE
}

#' Style a file internal
#' @keywords internal
style_file_internal <- function(file, configuration, ignored_paths) {
  if (is_in_ignored_paths(file, ignored_paths)) {
    return(FALSE)
  }
  if (!file.exists(file)) {
    stop("File " + file + " does not exist")
  }
  size <- file.info(file)$size
  code <- readChar(con = file, nchars = size)
  formatted <- format_code(code, configuration)
  if (formatted[[1]] == "success") {
    formatted[[2]]
  } else {
    stop("Failed to style the file.")
  }
  write(x = formatted[[2]], file = file)
  TRUE
}

#' Style text
#'
#' @details
#' This function is vectorized.
#' To see possible configuration options, see [get_default_config()].
#'
#' @inheritParams style
#' @param text (`character`) the text to style
#' @return (`character`) The text formatted as R code.
#'
#' @export
#' @examples
#' code <- "a+b"
#' styled <- style_text(code)
#' code <- c("a+b", "A<-7")
#' styled <- style_text(code)
#'
style_text <- function(text, configuration = list()) {
  vapply(
    X = text,
    FUN = function(code) {
      formatted <- format_code(code, configuration)
      if (formatted[[1]] == "success") {
        formatted[[2]]
      } else {
        stop("Failed to style the text. Error: ", truncate_error(formatted[[2]]))
      }
    },
    FUN.VALUE = character(1),
    USE.NAMES = FALSE
  )
}

#' Truncate the error message
#'
#' @keywords internal
truncate_error <- function(err) {
  ifelse(nchar(err) > 80, sprintf("%s...", substr(err, 1, 77)), err)
}

#' Fallback to simpler symbol if console does not support Unicode characters
#' @param symbol (`character`) to print by default
#' @param fallback_symbol (`character`) to print if the option `tergo.unicode`
#' is `FALSE`.
#' @keywords internal
get_printed_symbol <- function(symbol, fallback_symbol) {
  ifelse(isFALSE(getOption("tergo.unicode")), fallback_symbol, symbol)
}

