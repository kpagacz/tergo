#' Check for required RStudio API package
#'
#' Ensures the `rstudioapi` package is available, as it is required for addin functionality.
check_rstudioapi <- function() {
  if (!requireNamespace("rstudioapi", quietly = TRUE)) {
    stop("The 'rstudioapi' package is required for this addin.")
  }
}

#' Style the current package (RStudio addin)
#'
#' Automatically styles all R code in the current project/package using
#' \code{\link[tergo]{style_pkg}}. If not called within a project, it
#' defaults to the current working directory.
#'
#' @keywords internal
style_pkg_addin <- function() {
  check_rstudioapi()

  # Detect current project directory (fallback to working directory)
  project_path <- rstudioapi::getActiveProject()
  if (is.null(project_path)) {
    project_path <- getwd()
  }

  # Attempt styling silently
  result <- try(tergo::style_pkg(path = project_path), silent = TRUE)

  if (inherits(result, "try-error")) {
    stop("tergo::style_pkg failed to style the package.")
  }

  invisible(NULL)
}

#' Style the active file (RStudio addin)
#'
#' Styles the currently active file in the RStudio editor and saves the formatted code.
#'
#' @keywords internal
style_active_file_addin <- function() {
  check_rstudioapi()

  # Get the source editor context
  context <- rstudioapi::getActiveDocumentContext()
  file_path <- context$path

  # Ensure the file is saved before attempting to style
  if (is.null(file_path) || file_path == "") {
    rstudioapi::showDialog(title = "Cannot style", message = "Please save the document before styling.")
    return(invisible(NULL))
  }

  # Attempt styling silently
  result <- try(tergo::style_file(file_path), silent = TRUE)

  if (inherits(result, "try-error")) {
    stop(sprintf("tergo::style_file failed to style the %s file.", file_path))
  }

  invisible(NULL)
}

#' Style the selected text (RStudio addin)
#'
#' Styles the selected text in the RStudio editor, replacing it with the formatted version.
#'
#' @keywords internal
style_selection_addin <- function() {
  check_rstudioapi()

  # Get the source editor context
  context <- rstudioapi::getSourceEditorContext()

  # Check if there are any selections
  if (length(context$selection) == 0) {
    rstudioapi::showDialog(title = "No selection found", message = "Please select some code before using this addin.")
    return(invisible(NULL))
  }

  text <- context$selection[[1L]]$text
  range <- context$selection[[1L]]$range

  result <- try(
    expr = {
      styled_text <- tergo::style_text(text)
      rstudioapi::modifyRange(range, styled_text, id = context$id)
    },
    silent = TRUE
  )

  # Show dialog only if all selections were successfully styled
  if (inherits(result, "try-error")) {
    stop("tergo::style_text failed to style the text.")
  }

  invisible(rstudioapi::documentSave(context$id))
}

