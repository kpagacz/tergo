testthat::test_that("style_text validates the configuration", {
  testthat::expect_error(
    style_text(
      "1+1",
      configuration = list(line_length = 80),
      "Failed to style the text. Error: line_length configuration value must be an integer. Did you forget about L?"
    )
  )
  testthat::expect_error(
    style_text(
      "1+1",
      configuration = list(indent = 2),
      "Failed to style the text. Error: indent configuration value must be an integer. Did you forget about L?"
    )
  )
  testthat::expect_error(
    style_text(
      "1+1",
      configuration = list(embracing_op_no_nl = 2),
      "Failed to style the text. Error: embracing_op_no_nl configuration value must be a boolean."
    )
  )
})

