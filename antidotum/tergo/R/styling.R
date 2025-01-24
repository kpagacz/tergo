#' Style a package
#'
#' @details
#' Configuration is read from a file named `tergo.toml` in the root of the
#' package. The precedence of the configuration is (from the highest to lowest):
#' 1. The configuration passed to the function.
#' 2. The configuration file.
#'
#' @param config_file (`character`) The path to the configuration file
#' @param configuration (`list`) The path to the configuration for formatting
#' @param ... additional parameters to [tergo::style_pkg()]
#'
#' @export
#' @examples
#' style()
#' style(config_file = "tergo.toml", configuration = list())
style <- function(config_file = "tergo.toml", configuration = list(), ...) {
  style_pkg(path = getwd(), config_file = config_file, configuration = configuration)
}

#' Style a package
#'
#' @inheritParams style
#' @param path (`character`) The path to the package.
#' @param force (`logical(1`) Whether to format the files even
#' if no package was found. `TRUE` - format the `.R` and `.r` files
#' found in the directory (recursive). `FALSE` exit without formatting
#' anything.
#' @param extensions (`character`) The extensions of the files to format.
#'
#' @export
style_pkg <- function(path = ".",
                      config_file = "tergo.toml",
                      configuration = list(),
                      force = FALSE,
                      extensions = c(".R", ".r")) {
  if (!is.character(path) || length(path) != 1) {
    stop("Path must be a character")
  }
  if (!is.character(config_file) || length(config_file) != 1) {
    stop("Config file must be a character")
  }
  if (!is.logical(force) || length(force) != 1) {
    stop("Force must be a logical")
  }
  if (!is.list(configuration)) {
    stop("Configuration must be a list")
  }

  # Read a configuration file
  wd <- path
  config <- NULL

  repeat {
    if (file.exists(file.path(wd, config_file))) {
      config <- file.path(wd, config_file)
      break
    }
    # OS agnostic file path splitting
    if (strsplit(wd, split = .Platform$file.sep) |> length() == 1) {
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

  # Find package root. If not found, then just list files in the current directory
  package_root <- getwd()
  while (!file.exists(file.path(package_root, "DESCRIPTION"))) {
    package_root <- dirname(package_root)
    if (strsplit(package_root, split = .Platform$file.sep) |> length() == 1) {
      package_root <- getwd()
      break
    }
  }

  if (!file.exists(file.path(package_root, "DESCRIPTION")) && !force) {
    message("No package detected. Exiting without formatting anything.")
    return(invisible())
  }

  files <- list.files(package_root, recursive = TRUE, full.names = TRUE)
  files <- Filter(function(file) any(endsWith(file, extensions)), files)
  # Format
  for (file in files) {
    tryCatch(
      expr = {
        style_file(file, configuration)
      },
      error = function(err) {
        cat(sprintf("Error formatting the file: %s", file))
        print(err)
      }
    )
  }
  message(sprintf("Sucessfully styled %i files.", length(files)))
}

#' Style a file
#'
#' @inheritParams style
#' @param file (`character`) the file to format
#'
#' @export
style_file <- function(file, configuration = list()) {
  if (!file.exists(file)) {
    stop("File " + file + " does not exist")
  }
  size <- file.info(file)$size
  code <- readChar(con = file, nchars = size)
  formatted <- format_code(code, configuration)
  write(x = formatted, file = file)
}

#' Style text
#'
#' @details
#' This function is vectorized.
#'
#' @inheritParams style
#' @param text (`character`) the text to style
#'
#' @return (`character`) the text formatted as R code
#' @export
#' @examples
#' code <- "function(){}"
#' style_text(code)
#'
#' code <- c("function(){}", "A<-7")
#' style_text(code)
style_text <- function(text, configuration = list()) {
  vapply(X = text, FUN = function(code) format_code(code, configuration), FUN.VALUE = character(1), USE.NAMES = FALSE)
}

