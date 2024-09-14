rows <- lapply(
  datanames,
  function(dataname) {
    parent <- teal.data::parent(joinkeys, dataname)

    # todo: what should we display for a parent dataset?
    #     - Obs and Subjects
    #     - Obs only
    #     - Subjects only
    # todo (for later): summary table should be displayed in a way that child datasets
    #       are indented under their parent dataset to form a tree structure
    subject_keys <- if (length(parent) > 0) {
      names(joinkeys[dataname, parent])
    } else {
      joinkeys[dataname, dataname]
    }
    get_object_filter_overview(
      filtered_data = filtered_data_objs[[dataname]],
      unfiltered_data = unfiltered_data_objs[[dataname]],
      dataname = dataname,
      subject_keys = subject_keys
    )
  }
)

unssuported_idx <- vapply(rows, function(x) all(is.na(x[-1])), logical(1)) # this is mainly for vectors
