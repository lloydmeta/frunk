window.BENCHMARK_DATA = {
  "lastUpdate": 1617348215632,
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
      }
    ]
  }
}