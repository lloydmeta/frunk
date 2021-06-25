window.BENCHMARK_DATA = {
  "lastUpdate": 1624586234668,
  "repoUrl": "https://github.com/lloydmeta/frunk",
  "entries": {
    "Frunk Benchmarks": [
      {
        "commit": {
          "author": {
            "email": "lloydmeta@users.noreply.github.com",
            "name": "Lloyd",
            "username": "lloydmeta"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8eb9ccb08f9060c4aa008a4f1786bc441c97ee27",
          "message": "[CI] Switch to GH Actions from Travis (#172)\n\n\r\nSigned-off-by: lloydmeta <lloydmeta@gmail.com>",
          "timestamp": "2021-04-02T12:07:23+09:00",
          "tree_id": "0bd79f80ad03cf074ecf391057f75a95645f89bf",
          "url": "https://github.com/lloydmeta/frunk/commit/8eb9ccb08f9060c4aa008a4f1786bc441c97ee27"
        },
        "date": 1617333130088,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "generic_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "creating_hlist",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "creating_tuple2",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_append",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_hlist_pat_match",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2_match",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_consuming",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_non_consuming",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_24fields",
            "value": 63,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 82,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 65,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 80,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "name",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sculpted_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 29,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_all_i32",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_mut",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_ref",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_value",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_mut",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_ref",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_value",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_i32",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 52,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_i32",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_option_string",
            "value": 72,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 819,
            "range": "± 266",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 732,
            "range": "± 122",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 339,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 45,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 156,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 25,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 39,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "ok_result_into_validated",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "validated_to_result",
            "value": 4,
            "range": "± 1",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "lloydmeta@users.noreply.github.com",
            "name": "Lloyd",
            "username": "lloydmeta"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7fa860ca8523ac4fff3096b327276d01b28a26a3",
          "message": "[Docs] Update 2021-04-02 (#173)\n\n- Remove travis badge\r\n- Add link to benches\r\n- Link to benchmarks in ToC\r\n\r\nSigned-off-by: lloydmeta <lloydmeta@gmail.com>",
          "timestamp": "2021-04-02T12:21:22+09:00",
          "tree_id": "72ee7be13d7d55253ffc9a66b892f90c0863c843",
          "url": "https://github.com/lloydmeta/frunk/commit/7fa860ca8523ac4fff3096b327276d01b28a26a3"
        },
        "date": 1617333844797,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "generic_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "creating_hlist",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "creating_tuple2",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_append",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_hlist_pat_match",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2_match",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_consuming",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_non_consuming",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_24fields",
            "value": 46,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 58,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 46,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 59,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "name",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sculpted_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 20,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_all_i32",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_mut",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_ref",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_value",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_mut",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_ref",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_value",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_i32",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 38,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_i32",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_option_string",
            "value": 50,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 595,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 528,
            "range": "± 94",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 250,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 32,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 112,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 19,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 29,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "ok_result_into_validated",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "validated_to_result",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "lloydmeta@users.noreply.github.com",
            "name": "Lloyd",
            "username": "lloydmeta"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "486c1a2f1648515baa30e0a6463744ceda151c7c",
          "message": "Rename benchmark results (#174)\n\nSigned-off-by: lloydmeta <lloydmeta@gmail.com>",
          "timestamp": "2021-04-02T15:37:41+09:00",
          "tree_id": "9b2b0729cee11fa51991dda668fcacc13843a3f0",
          "url": "https://github.com/lloydmeta/frunk/commit/486c1a2f1648515baa30e0a6463744ceda151c7c"
        },
        "date": 1617345702090,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "generic_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "creating_hlist",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "creating_tuple2",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_append",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_hlist_pat_match",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2_match",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_consuming",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_non_consuming",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_24fields",
            "value": 69,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 83,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 71,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 87,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "name",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sculpted_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 24,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_all_i32",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_mut",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_ref",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_value",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_mut",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_ref",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_value",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_i32",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 49,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_i32",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_option_string",
            "value": 65,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 767,
            "range": "± 91",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 693,
            "range": "± 97",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 306,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 42,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 137,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 25,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 39,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "ok_result_into_validated",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "validated_to_result",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "lloydmeta@users.noreply.github.com",
            "name": "Lloyd",
            "username": "lloydmeta"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5cc5511109778bba30012b5dcf1272ca574d3f33",
          "message": "Link CI badge to filtered page (#175)\n\nSigned-off-by: lloydmeta <lloydmeta@gmail.com>",
          "timestamp": "2021-04-02T16:18:52+09:00",
          "tree_id": "b58699084cd0ea7e43d433f0a24f62bd1daaa54d",
          "url": "https://github.com/lloydmeta/frunk/commit/5cc5511109778bba30012b5dcf1272ca574d3f33"
        },
        "date": 1617348214737,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "generic_conversion",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "creating_hlist",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "creating_tuple2",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_append",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_hlist_pat_match",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2_match",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_consuming",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_non_consuming",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_24fields",
            "value": 60,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 77,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 61,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 85,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "name",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sculpted_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 29,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_all_i32",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_mut",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_ref",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_value",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_mut",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_ref",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_value",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_i32",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 50,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_i32",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_option_string",
            "value": 67,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 773,
            "range": "± 95",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 725,
            "range": "± 232",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 329,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 45,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 142,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 24,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 37,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "ok_result_into_validated",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "validated_to_result",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "somewhat.fluffy@gmail.com",
            "name": "ImmConCon",
            "username": "ImmemorConsultrixContrarie"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e91bcc82b49bb542b711c58987feaafabfee7238",
          "message": "Allow folding hlist with a single `Poly` (#170)",
          "timestamp": "2021-04-16T14:29:19+09:00",
          "tree_id": "65c1bcc5d83925a041011bd4e9e6de05594728bd",
          "url": "https://github.com/lloydmeta/frunk/commit/e91bcc82b49bb542b711c58987feaafabfee7238"
        },
        "date": 1618551143777,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "generic_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "creating_hlist",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "creating_tuple2",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_append",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_hlist_pat_match",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2_match",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_consuming",
            "value": 2,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_non_consuming",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_24fields",
            "value": 70,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 94,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 76,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 85,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "name",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sculpted_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 25,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_all_i32",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_mut",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_ref",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_value",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_mut",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_ref",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_value",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_i32",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 49,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_i32",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_option_string",
            "value": 68,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 826,
            "range": "± 81",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 751,
            "range": "± 106",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 331,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 44,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 139,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 26,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 40,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "ok_result_into_validated",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "validated_to_result",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "lloydmeta@users.noreply.github.com",
            "name": "Lloyd",
            "username": "lloydmeta"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a1f4383ba13e1139010f65c3b397ed09e846b5f4",
          "message": "[2021 04 16 release] 0.3.2 (#176)\n\n* Add mention in changelog\r\n* Capture fact that `laws` depends on main `frunk`\r\n* Bump versions and release",
          "timestamp": "2021-04-16T15:00:58+09:00",
          "tree_id": "6564fad325d61c224c8725edb74db2935749bbd4",
          "url": "https://github.com/lloydmeta/frunk/commit/a1f4383ba13e1139010f65c3b397ed09e846b5f4"
        },
        "date": 1618553147581,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "generic_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "creating_hlist",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "creating_tuple2",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_append",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_hlist_pat_match",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2_match",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_consuming",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_non_consuming",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_24fields",
            "value": 58,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 77,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 60,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 78,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "name",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sculpted_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 26,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_all_i32",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_mut",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_ref",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_value",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_mut",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_ref",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_value",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_i32",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 50,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_i32",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_option_string",
            "value": 67,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 757,
            "range": "± 112",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 674,
            "range": "± 130",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 321,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 45,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 146,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 25,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 38,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "ok_result_into_validated",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "validated_to_result",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "diagonaldevice@gmail.com",
            "name": "Michael Lamparski",
            "username": "ExpHP"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d586e6239944147f9d1425f9fc7360aad16305ad",
          "message": "Merge pull request #177 from ImmemorConsultrixContrarie/foldr_doc_fix\n\nFoldr doc fix",
          "timestamp": "2021-04-21T23:22:20-04:00",
          "tree_id": "378858650b7cefef64086b9931dc2c10a37a0463",
          "url": "https://github.com/lloydmeta/frunk/commit/d586e6239944147f9d1425f9fc7360aad16305ad"
        },
        "date": 1619061870434,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "generic_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "creating_hlist",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "creating_tuple2",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_append",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_hlist_pat_match",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2_match",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_consuming",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_non_consuming",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_24fields",
            "value": 62,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 79,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 63,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 75,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "name",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sculpted_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 21,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_all_i32",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_mut",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_ref",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_value",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_mut",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_ref",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_value",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_i32",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 44,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_i32",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_option_string",
            "value": 58,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 701,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 637,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 281,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 125,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 22,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 35,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "ok_result_into_validated",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "validated_to_result",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "lloydmeta@users.noreply.github.com",
            "name": "Lloyd",
            "username": "lloydmeta"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "15e5ba16903e47227fe6c16b902f61d41c575326",
          "message": "0.4 cleanup: rename HList type macro, remove HList#length() (#179)\n\n* Rename `Hlist!` type macro to `HList!`\r\n\r\n* - Remove deprecated `HList#length()` method\r\n\r\n* - Update readme\r\n  - Mention ExpHP as maintainer\r\n\r\n* Update code with clippy recommendation",
          "timestamp": "2021-05-07T12:02:43+09:00",
          "tree_id": "3e4ef793b8cf0a323c0540f29a30ae273db1aa49",
          "url": "https://github.com/lloydmeta/frunk/commit/15e5ba16903e47227fe6c16b902f61d41c575326"
        },
        "date": 1620356784470,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "generic_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "creating_hlist",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "creating_tuple2",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_append",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_hlist_pat_match",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2_match",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_consuming",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_non_consuming",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_24fields",
            "value": 74,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 88,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 73,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 88,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "name",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sculpted_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 26,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_all_i32",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_mut",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_ref",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_value",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_mut",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_ref",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_value",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_i32",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 53,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_i32",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_option_string",
            "value": 70,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 936,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 960,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 338,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 155,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 35,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 42,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "ok_result_into_validated",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "validated_to_result",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "somewhat.fluffy@gmail.com",
            "name": "ImmConCon",
            "username": "ImmemorConsultrixContrarie"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ad73d4a6182076363ed84f5f7efdc2c00cd01cd5",
          "message": "Make `foldl` and `foldr` take exactly same arguments (#178)",
          "timestamp": "2021-05-12T08:11:54+09:00",
          "tree_id": "18e6714287b3e29c42b9828758840a16be3444f3",
          "url": "https://github.com/lloydmeta/frunk/commit/ad73d4a6182076363ed84f5f7efdc2c00cd01cd5"
        },
        "date": 1620774861117,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "generic_conversion",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "creating_hlist",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "creating_tuple2",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_append",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_hlist_pat_match",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2_match",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_consuming",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_non_consuming",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_24fields",
            "value": 61,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 78,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 62,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 79,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "name",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sculpted_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 27,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_all_i32",
            "value": 3,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_mut",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_ref",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_value",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_mut",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_ref",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_value",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_i32",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 51,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_i32",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_option_string",
            "value": 68,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 826,
            "range": "± 209",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 834,
            "range": "± 179",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 331,
            "range": "± 194",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 53,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 148,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 33,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 38,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "ok_result_into_validated",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "validated_to_result",
            "value": 5,
            "range": "± 1",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "lloydmeta@users.noreply.github.com",
            "name": "Lloyd",
            "username": "lloydmeta"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b8fa84639e7376ad171ba963842f05f3c453aac9",
          "message": "Fix unicode identifiers support (#186)\n\n* [Fix] Unicode support\r\n\r\n1.53 brought unicode support; apparently, we were missing the unicode bookend types.\r\n\r\nThis fixes that and adds a regression test.\r\n\r\n* * Disable clippy self convention for a method.",
          "timestamp": "2021-06-19T12:09:23+09:00",
          "tree_id": "e2c9664375a0f7833918046ed2e7bf7cf57237c3",
          "url": "https://github.com/lloydmeta/frunk/commit/b8fa84639e7376ad171ba963842f05f3c453aac9"
        },
        "date": 1624072372410,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "generic_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "creating_hlist",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "creating_tuple2",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_append",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_hlist_pat_match",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2_match",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_consuming",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_non_consuming",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_24fields",
            "value": 67,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 89,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 67,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 88,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "name",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sculpted_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 26,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_all_i32",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_mut",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_ref",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_value",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_mut",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_ref",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_value",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_i32",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 59,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_i32",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_option_string",
            "value": 74,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 871,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 868,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 312,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 55,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 146,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 33,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 40,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "ok_result_into_validated",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "validated_to_result",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "m1brobbel@gmail.com",
            "name": "Matthijs Brobbel",
            "username": "mbrobbel"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0a21dc4033e8bec5ed666eb12291521795341dd3",
          "message": "Bump quote, syn and proc-macro2 to 1 (#183)\n\nBumps `quote`, `syn` and `proc-macro2` dependencies of Frunk's proc macro crates to `1`.\r\n\r\nCo-authored-by: Lloyd <lloydmeta@users.noreply.github.com>",
          "timestamp": "2021-06-19T12:12:59+09:00",
          "tree_id": "a65a1cefce9b2b8b10a366fd35f22313a59b9eff",
          "url": "https://github.com/lloydmeta/frunk/commit/0a21dc4033e8bec5ed666eb12291521795341dd3"
        },
        "date": 1624072498774,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "generic_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "creating_hlist",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "creating_tuple2",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_append",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_hlist_pat_match",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2_match",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_consuming",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_non_consuming",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_24fields",
            "value": 70,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 91,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 70,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 92,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "name",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sculpted_conversion",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 26,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_all_i32",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_mut",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_ref",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_value",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_mut",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_ref",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_value",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_i32",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 61,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_i32",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_option_string",
            "value": 78,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 903,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 918,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 331,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 55,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 153,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 35,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 42,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "ok_result_into_validated",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "validated_to_result",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "lloydmeta@users.noreply.github.com",
            "name": "Lloyd",
            "username": "lloydmeta"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f83e1d8c2e4e6d080cc646ffc9456eb90f71960a",
          "message": "[Docs] Changelog updates (#188)",
          "timestamp": "2021-06-25T10:42:50+09:00",
          "tree_id": "90cb575202d7843fbbf191508ef4cfadaa8c6357",
          "url": "https://github.com/lloydmeta/frunk/commit/f83e1d8c2e4e6d080cc646ffc9456eb90f71960a"
        },
        "date": 1624585509383,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "generic_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "creating_hlist",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "creating_tuple2",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_append",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_hlist_pat_match",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2_match",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_consuming",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_non_consuming",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_24fields",
            "value": 61,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 74,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 61,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 74,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "name",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sculpted_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 22,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_all_i32",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_mut",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_ref",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_value",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_mut",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_ref",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_value",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_i32",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 51,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_i32",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_option_string",
            "value": 64,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 757,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 765,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 287,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 45,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 128,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 36,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "ok_result_into_validated",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "validated_to_result",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "lloydmeta@users.noreply.github.com",
            "name": "Lloyd",
            "username": "lloydmeta"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "44a0fefecee1a0e1b327721c8ab3495ee42b595d",
          "message": "[CI] Add Rust cache (#189)",
          "timestamp": "2021-06-25T10:52:18+09:00",
          "tree_id": "a1c1f948cbe141bc9ae9349a3b6e6902807c093a",
          "url": "https://github.com/lloydmeta/frunk/commit/44a0fefecee1a0e1b327721c8ab3495ee42b595d"
        },
        "date": 1624586232791,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "generic_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "creating_hlist",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "creating_tuple2",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_append",
            "value": 4,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_hlist_pat_match",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2_match",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_consuming",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_non_consuming",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_24fields",
            "value": 66,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 78,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 59,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 82,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "name",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sculpted_conversion",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 28,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_all_i32",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_mut",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_ref",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_value",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_mut",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_ref",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_value",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_i32",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 57,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_i32",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_option_string",
            "value": 70,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 816,
            "range": "± 133",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 769,
            "range": "± 375",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 319,
            "range": "± 74",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 46,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 140,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 33,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 39,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "ok_result_into_validated",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "validated_to_result",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}