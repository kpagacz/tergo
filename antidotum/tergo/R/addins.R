#' Style the current package (RStudio addin)
#'
#' @keywords internal
style_pkg_addin <- function() {
  if (!requireNamespace("rstudioapi", quietly = TRUE)) {
    stop("The 'rstudioapi' package is required for this addin.")
  }

  # possibly detect current project directory
  project_path <- rstudioapi::getActiveProject()
  if (is.null(project_path)) {
    # fallback if not in a project
    project_path <- getwd()
  }

  # call your styling function
  tergo::style_pkg(path = project_path)

  rstudioapi::showDialog(
    title = "Package styling complete",
    message = paste0("Package at '", project_path, "' has been styled.")
  )

  invisible(NULL)
}

#' Style the active file (RStudio addin)
#'
#' This function will look for the currently active file in the
#' RStudio editor, style it, and then overwrite its contents
#' with the formatted code.
#'
#' @keywords internal
style_active_file_addin <- function() {
  if (!requireNamespace("rstudioapi", quietly = TRUE)) {
    stop("The 'rstudioapi' package is required for this addin.")
  }

  # Get the context of the source editor
  context <- rstudioapi::getSourceEditorContext()
  file_path <- context$path

  # If no path is available (e.g., an unsaved document), handle gracefully
  if (is.null(file_path) || file_path == "") {
    rstudioapi::showDialog(title = "Cannot style", message = "Please save the document before styling.")
    return(invisible(NULL))
  }

  # Call your package's styling function on this file
  # Adjust 'configuration' as needed, or supply your own config list
  tergo::style_file(file = file_path)

  # Optionally, if you prefer to do in-memory replacement (instead of rewriting the file),
  # you can read the newly styled code and set the document contents.
  # styled_code <- readLines(file_path)
  # rstudioapi::setDocumentContents(paste(styled_code, collapse = "\n"), id = context$id)

  # Provide user feedback
  rstudioapi::showDialog(title = "Styling complete", message = paste0("The file '", file_path, "' has been styled."))

  invisible(NULL)
}

#' Style the selected text (RStudio addin)
#'
#' This addin will retrieve the current text selection(s) in the
#' RStudio editor, run it through `style_text()`, and then replace
#' the selected text with the formatted version.
#'
#' @keywords internal
style_selection_addin <- function() {
  if (!requireNamespace("rstudioapi", quietly = TRUE)) {
    stop("The 'rstudioapi' package is required for this addin.")
  }

  context <- rstudioapi::getSourceEditorContext()

  # Handle cases where there is no selection or an empty selection
  if (length(context$selection) == 0) {
    rstudioapi::showDialog(title = "No selection found", message = "Please select some code before using this addin.")
    return(invisible(NULL))
  }

  # It’s possible there are multiple selection regions.
  # We'll loop through each region and style it.
  for (i in seq_along(context$selection)) {
    sel <- context$selection[[i]]

    # If the selection is empty, skip
    if (nzchar(sel$text)) {
      # Style the selected text using your styling function
      styled_text <- tergo::style_text(sel$text)
      # Replace that region with the styled text
      rstudioapi::modifyRange(sel$range, styled_text, id = context$id)
    }
  }

  rstudioapi::showDialog(title = "Styling Complete", message = "The selected text has been styled.")

  invisible(NULL)
}
