if (isTRUE(grepl("^t_", output_function))) {
  show_plot_rv(FALSE)
  shinyjs::show("mmrm_table")
} else if (isTRUE(grepl("^g_", output_function))) {
  shinyjs::hide("mmrm_table")
  show_plot_rv(TRUE)
} else {
  stop("unknown output type")
}
