        (utils::capture.output(Rd2txt(rd, fragment = TRUE))
            |> paste(collapse = "\n")
            |> trimws())
