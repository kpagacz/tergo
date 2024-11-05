rows <- lapply(
  datanames,
  function(dataname) {
    get_object_filter_overview(
      filtered_data = filtered_data_objs[[dataname]],
      unfiltered_data = unfiltered_data_objs[[dataname]],
      dataname = dataname,
      subject_keys = subject_keys
    )
  }
)
