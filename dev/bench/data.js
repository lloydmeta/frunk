window.BENCHMARK_DATA = {
  "lastUpdate": 1765090249044,
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
          "id": "1bb2b7d593fe3fe2ff1c60f6456df70cec7e02de",
          "message": "[Release] 0.4.0 (#190)",
          "timestamp": "2021-06-25T11:10:49+09:00",
          "tree_id": "7929a99f52e36714f8721f36833d966a07951cf2",
          "url": "https://github.com/lloydmeta/frunk/commit/1bb2b7d593fe3fe2ff1c60f6456df70cec7e02de"
        },
        "date": 1624587268977,
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
            "value": 67,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 82,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 68,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 82,
            "range": "± 11",
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
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 29,
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
            "value": 61,
            "range": "± 6",
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
            "value": 83,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 899,
            "range": "± 108",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 876,
            "range": "± 96",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 335,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 61,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 160,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 36,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 40,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "ok_result_into_validated",
            "value": 3,
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
          "id": "2b16d2cabdf0ce6ef40f92562ba5265f33eb72fa",
          "message": "[Build] Update to 2018 edition (#191)",
          "timestamp": "2021-06-25T16:23:26+09:00",
          "tree_id": "603da54905d0c0479611c36e8526ac20063817b6",
          "url": "https://github.com/lloydmeta/frunk/commit/2b16d2cabdf0ce6ef40f92562ba5265f33eb72fa"
        },
        "date": 1624606001079,
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
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_24fields",
            "value": 54,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 66,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 53,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 67,
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
            "value": 24,
            "range": "± 5",
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
            "range": "± 9",
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
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 759,
            "range": "± 383",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 816,
            "range": "± 287",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 285,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 45,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 124,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 29,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 34,
            "range": "± 6",
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
          "id": "8d82a7b3dd28f73b2382a81b6f73c1cb346abf93",
          "message": "[Examples] Add examples of how to map over generic and labelled representations (#192)\n\n* [Examples] Add examples of how to map over generic and labelled representations\r\n\r\n```\r\n❯ cargo run --example generic\r\n    Finished dev [unoptimized + debuginfo] target(s) in 0.02s\r\n     Running `target/debug/examples/generic`\r\nJoe\r\nFirst name: bo\r\nLast name: peep\r\nage: 30\r\n❯ cargo run --example labelled\r\n    Finished dev [unoptimized + debuginfo] target(s) in 0.02s\r\n     Running `target/debug/examples/labelled`\r\nBlow\r\nExternalPerson {\r\n    age: 10,\r\n    address: ExternalAddress {\r\n        name: \"somewhere out there\",\r\n        phone: ExternalPhoneNumber {\r\n            main: 1234,\r\n        },\r\n    },\r\n    name: \"John\",\r\n}\r\nfirst_name: bo\r\nlast_name: peep\r\nage: 30\r\n```",
          "timestamp": "2021-07-17T16:26:47+09:00",
          "tree_id": "0a072ad3b953d6383a28ea631de662b1e9b879f4",
          "url": "https://github.com/lloydmeta/frunk/commit/8d82a7b3dd28f73b2382a81b6f73c1cb346abf93"
        },
        "date": 1626506958698,
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
            "value": 74,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 89,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 73,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 90,
            "range": "± 3",
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
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 61,
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
            "value": 77,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 910,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 928,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 333,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 54,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 154,
            "range": "± 1",
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
          "id": "246741f51fe19c2056fb2cc75e31a4e3afbac6a8",
          "message": "Clippy fixes (#195)\n\nSigned-off-by: lloydmeta <lloydmeta@gmail.com>",
          "timestamp": "2021-09-15T19:03:27+09:00",
          "tree_id": "a4aedf5f512a729af11d56d5432eba2f660510b9",
          "url": "https://github.com/lloydmeta/frunk/commit/246741f51fe19c2056fb2cc75e31a4e3afbac6a8"
        },
        "date": 1631700362033,
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
            "value": 57,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 75,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 57,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 75,
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
            "value": 23,
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
            "value": 65,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 607,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 636,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 275,
            "range": "± 1",
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
            "value": 129,
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
            "value": 34,
            "range": "± 1",
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
            "email": "fdsteffahn@gmail.com",
            "name": "Frank Steffahn",
            "username": "steffahn"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f1926d75a328aa23209175f067ec3ea1a2689628",
          "message": "Fix typo in documentation of `HCons::sculpt` (#194)\n\nCo-authored-by: Lloyd <lloydmeta@users.noreply.github.com>",
          "timestamp": "2021-09-15T19:07:42+09:00",
          "tree_id": "3632fd7883057ab7158a1aae3c30dcd301140256",
          "url": "https://github.com/lloydmeta/frunk/commit/f1926d75a328aa23209175f067ec3ea1a2689628"
        },
        "date": 1631700563208,
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
            "value": 57,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 75,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 57,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 75,
            "range": "± 2",
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
            "value": 23,
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
            "value": 608,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 636,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 279,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 44,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 132,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 34,
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
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "r.proskuryakoff@gmail.com",
            "name": "Roman",
            "username": "kpp"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "09a3d4f45f7b2ac5b996fcdaa7c85173f0533ab1",
          "message": "Optimize Semigroup for HashSet and HashMap (#196)\n\n* Speedup Semigroup for HashSet\r\n\r\n* Speedup Semigroup for HashMap\r\n\r\n* Prettify code in Semigroup for Option\r\n\r\n* Fix clippy warning",
          "timestamp": "2021-12-17T18:23:08+09:00",
          "tree_id": "2e96de9842bd268b30c399422e78e105135db08b",
          "url": "https://github.com/lloydmeta/frunk/commit/09a3d4f45f7b2ac5b996fcdaa7c85173f0533ab1"
        },
        "date": 1639733119481,
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
            "range": "± 1",
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
            "value": 68,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 75,
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
            "value": 23,
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
            "value": 77,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 615,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 645,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 273,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 44,
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
            "range": "± 3",
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
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "gluszak.andrzej@gmail.com",
            "name": "Andrzej Głuszak",
            "username": "agluszak"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "58a399dc2d8935e8ccd4b94c6c7931f86b0c8434",
          "message": "[Build] Update to 2021 edition (#200)\n\nCo-authored-by: andrzej.gluszak <andrzej.gluszak@mpi-sp.org>",
          "timestamp": "2022-09-18T11:05:36+09:00",
          "tree_id": "b3310afc7a7dd7d2a85af7243fd553e351bf9973",
          "url": "https://github.com/lloydmeta/frunk/commit/58a399dc2d8935e8ccd4b94c6c7931f86b0c8434"
        },
        "date": 1663466903684,
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
            "value": 81,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 98,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 1782,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 1800,
            "range": "± 31",
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
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 18,
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
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 55,
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
            "value": 76,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 740,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 720,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 318,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 53,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 141,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 42,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 47,
            "range": "± 2",
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
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "gluszak.andrzej@gmail.com",
            "name": "Andrzej Głuszak",
            "username": "agluszak"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "58a399dc2d8935e8ccd4b94c6c7931f86b0c8434",
          "message": "[Build] Update to 2021 edition (#200)\n\nCo-authored-by: andrzej.gluszak <andrzej.gluszak@mpi-sp.org>",
          "timestamp": "2022-09-18T11:05:36+09:00",
          "tree_id": "b3310afc7a7dd7d2a85af7243fd553e351bf9973",
          "url": "https://github.com/lloydmeta/frunk/commit/58a399dc2d8935e8ccd4b94c6c7931f86b0c8434"
        },
        "date": 1663468360252,
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
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 82,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 1489,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 1499,
            "range": "± 6",
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
            "value": 15,
            "range": "± 0",
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
            "value": 46,
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
            "value": 63,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 625,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 613,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 266,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 44,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 120,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 40,
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
            "value": 2,
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
          "id": "573f44f8e61ed461ea3e0214a2dd1b3b7a0b7402",
          "message": "Fix needless borrow (#202)\n\nSigned-off-by: lloydmeta <lloydmeta@gmail.com>",
          "timestamp": "2022-10-02T13:46:23+09:00",
          "tree_id": "19e65945808014f48041a7d838f327c19c4a0ada",
          "url": "https://github.com/lloydmeta/frunk/commit/573f44f8e61ed461ea3e0214a2dd1b3b7a0b7402"
        },
        "date": 1664686115005,
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
            "value": 0,
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
            "value": 2,
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
            "value": 65,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 76,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 1594,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 1582,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 0,
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
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_all_i32",
            "value": 4,
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
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 41,
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
            "value": 57,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 578,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 591,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 250,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 51,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 108,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 42,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 37,
            "range": "± 1",
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
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "rose@krx.sh",
            "name": "Rose Hudson",
            "username": "rosefromthedead"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "bb4e8d51f0edb582e56731e2482850b80febab22",
          "message": "add `extract` for getting the value out of a 1-coproduct (#201)",
          "timestamp": "2022-10-02T15:03:51+09:00",
          "tree_id": "f2e76a3b25470e3b9036212b81bd40714a8165ab",
          "url": "https://github.com/lloydmeta/frunk/commit/bb4e8d51f0edb582e56731e2482850b80febab22"
        },
        "date": 1664690816324,
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
            "value": 80,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 97,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 1749,
            "range": "± 180",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 1684,
            "range": "± 104",
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
            "value": 18,
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
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 53,
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
            "value": 70,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 709,
            "range": "± 94",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 712,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 320,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 54,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 147,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 41,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 47,
            "range": "± 7",
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
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "benwolverine2019@gmail.com",
            "name": "Ben Reeves",
            "username": "BGR360"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "166940a52d0e50553b75568cba10438079f9d844",
          "message": "Add `Coproduct::map()` method (#204)\n\n* Add `Coproduct::map()` method.\r\n* Allow multi-line closures in `poly_fn!`.",
          "timestamp": "2022-11-03T15:33:41+09:00",
          "tree_id": "c18abc3bafef10044d89faedaa40c81bd959868a",
          "url": "https://github.com/lloydmeta/frunk/commit/166940a52d0e50553b75568cba10438079f9d844"
        },
        "date": 1667457367722,
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
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 81,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 1462,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 1509,
            "range": "± 7",
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
            "value": 15,
            "range": "± 0",
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
            "value": 45,
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
            "value": 62,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 615,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 616,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 280,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 44,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 118,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 40,
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
            "value": 2,
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
          "id": "ff5dafc0d5693b34b6af869e062158e4a442f894",
          "message": "[Release] 0.4.1 (#205)\n\n* Add changelog\r\n* Bump versions and release.\r\n\r\nSigned-off-by: lloydmeta <lloydmeta@gmail.com>",
          "timestamp": "2022-11-03T15:52:26+09:00",
          "tree_id": "2c643b8b692d734b7b2489a1405ddce45d41b1ed",
          "url": "https://github.com/lloydmeta/frunk/commit/ff5dafc0d5693b34b6af869e062158e4a442f894"
        },
        "date": 1667458426201,
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
            "value": 82,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 97,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 1767,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 1814,
            "range": "± 103",
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
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 18,
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
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 54,
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
            "value": 75,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 741,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 743,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 311,
            "range": "± 12",
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
            "value": 142,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 41,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 48,
            "range": "± 1",
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
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "charles@computer.surgery",
            "name": "Charles Hall",
            "username": "CobaltCause"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "70d17a79e6c458e36860b22c4c4272d4c54c2df5",
          "message": "turn links into hyperlinks (#210)",
          "timestamp": "2023-05-06T12:50:04+09:00",
          "tree_id": "4e269864e7eacd14098aa2218a17f3fc02f8a816",
          "url": "https://github.com/lloydmeta/frunk/commit/70d17a79e6c458e36860b22c4c4272d4c54c2df5"
        },
        "date": 1683345110420,
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
            "value": 2,
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
            "value": 71,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 90,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 76,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 86,
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
            "value": 14,
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
            "value": 39,
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
            "value": 57,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 704,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 572,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 282,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 116,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 33,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 38,
            "range": "± 1",
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
            "value": 0,
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
          "id": "a891af803577c1f45306f78ccc424da9e10238b8",
          "message": "Add RUSTDOCFLAGS for running docs check (#211)\n\nSigned-off-by: lloydmeta <lloydmeta@gmail.com>",
          "timestamp": "2023-05-06T12:54:20+09:00",
          "tree_id": "48868e015a1af5d5e3b0607742a22c0631b6c69c",
          "url": "https://github.com/lloydmeta/frunk/commit/a891af803577c1f45306f78ccc424da9e10238b8"
        },
        "date": 1683345320737,
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
            "value": 72,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 89,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 74,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 88,
            "range": "± 2",
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
            "value": 15,
            "range": "± 0",
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
            "value": 45,
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
            "value": 62,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 689,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 569,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 297,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 27,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 123,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 25,
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
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "validated_to_result",
            "value": 0,
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
          "id": "341df3e921fee3e5891724abf4906869fa18284f",
          "message": "Bump time crate (#212)\n\nSigned-off-by: lloydmeta <lloydmeta@gmail.com>",
          "timestamp": "2023-05-06T13:07:26+09:00",
          "tree_id": "f2f5a90a589dd29fbfbf33888a36810e1712cb6e",
          "url": "https://github.com/lloydmeta/frunk/commit/341df3e921fee3e5891724abf4906869fa18284f"
        },
        "date": 1683346127540,
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
            "range": "± 1",
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
            "range": "± 1",
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
            "value": 84,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 103,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 86,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 102,
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
            "value": 19,
            "range": "± 1",
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
            "value": 53,
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
            "value": 72,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 838,
            "range": "± 129",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 671,
            "range": "± 231",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 351,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 31,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 143,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 29,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 48,
            "range": "± 6",
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
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "charles@computer.surgery",
            "name": "Charles Hall",
            "username": "CobaltCause"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ebcb73024c870935112fffb0d41415c4568ef3f7",
          "message": "add function for explicitly extending an hlist (#209)",
          "timestamp": "2023-05-06T21:37:39+09:00",
          "tree_id": "e87a97eb1b8f52a2dc19ed922fc8b79088ed7d1b",
          "url": "https://github.com/lloydmeta/frunk/commit/ebcb73024c870935112fffb0d41415c4568ef3f7"
        },
        "date": 1683376723578,
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
            "value": 2,
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
            "value": 66,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 90,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 74,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 89,
            "range": "± 3",
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
            "value": 14,
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
            "value": 39,
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
            "value": 56,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 701,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 552,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 283,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 115,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 33,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 39,
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
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "charles@computer.surgery",
            "name": "Charles Hall",
            "username": "CobaltCause"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3d15dd9c0d908a55de510ccccd2000c5a3ce33f2",
          "message": "Drop `proc-macro-hack`, upgrade `syn` to 2 (#214)",
          "timestamp": "2023-06-16T22:19:36+09:00",
          "tree_id": "baf9c131eba92d65e12ab35e719c74ba446729c2",
          "url": "https://github.com/lloydmeta/frunk/commit/3d15dd9c0d908a55de510ccccd2000c5a3ce33f2"
        },
        "date": 1686921697021,
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
            "value": 2,
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
            "value": 73,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 90,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 68,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 86,
            "range": "± 2",
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
            "value": 15,
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
            "value": 43,
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
            "value": 56,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 599,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 568,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 291,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 24,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 116,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 33,
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
            "value": 0,
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
          "id": "df2d6ea6242c853ea16fca944b2b1ef2af939fdf",
          "message": "0.4.2 release (#215)\n\n* Append changelog\r\n* Fix up dependency-graph.puml\r\n\r\nSigned-off-by: lloydmeta <lloydmeta@gmail.com>",
          "timestamp": "2023-06-17T22:54:39+09:00",
          "tree_id": "1a63ca171c8d91c6b7441d2893ca6fb7849c8d85",
          "url": "https://github.com/lloydmeta/frunk/commit/df2d6ea6242c853ea16fca944b2b1ef2af939fdf"
        },
        "date": 1687010295855,
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
            "value": 84,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 113,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 85,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 100,
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
            "value": 18,
            "range": "± 2",
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
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 56,
            "range": "± 6",
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
            "value": 73,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 759,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 729,
            "range": "± 157",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 371,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 32,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 147,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 29,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 49,
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
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "figsoda@pm.me",
            "name": "figsoda",
            "username": "figsoda"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8543fa8c4f8c5ca1146ae36ddbd1bcf48e28593f",
          "message": "avoid clippy::unneeded_field_pattern (#216)\n\n* avoid clippy::unneeded_field_pattern\r\n\r\n* run clippy with --tests in CI",
          "timestamp": "2023-06-25T07:44:45+09:00",
          "tree_id": "603c22b5ffb619b5755402e6ebc59bc876529a9e",
          "url": "https://github.com/lloydmeta/frunk/commit/8543fa8c4f8c5ca1146ae36ddbd1bcf48e28593f"
        },
        "date": 1687646765662,
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
            "value": 2,
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
            "value": 66,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 91,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 67,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 86,
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
            "value": 14,
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
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 43,
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
            "value": 57,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 599,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 569,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 287,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 24,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 112,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 34,
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
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "lloydmeta@gmail.com",
            "name": "lloydmeta",
            "username": "lloydmeta"
          },
          "committer": {
            "email": "lloydmeta@gmail.com",
            "name": "lloydmeta",
            "username": "lloydmeta"
          },
          "distinct": true,
          "id": "34ba13d8162d276b50dfcb8a6c46d4445c2409a5",
          "message": "Update changelog\n\nSigned-off-by: lloydmeta <lloydmeta@gmail.com>",
          "timestamp": "2023-06-25T07:46:09+09:00",
          "tree_id": "3353fb72732746b86fa680d030502dacfecae6a2",
          "url": "https://github.com/lloydmeta/frunk/commit/34ba13d8162d276b50dfcb8a6c46d4445c2409a5"
        },
        "date": 1687646894613,
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
            "value": 2,
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
            "value": 65,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 92,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 67,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 85,
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
            "value": 14,
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
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 43,
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
            "value": 56,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 617,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 565,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 287,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 24,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 113,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 33,
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
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "d.starosud@gmail.com",
            "name": "dima-starosud",
            "username": "dima-starosud"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "83b26a1467359d0aeff50e8dd6eb7a558829f42a",
          "message": "Document frunk_core dependency requirement for [Labelled]Generic (#219)",
          "timestamp": "2023-07-04T20:46:18+09:00",
          "tree_id": "caa3a45551f54eb39b5ef3049c92f902cb230062",
          "url": "https://github.com/lloydmeta/frunk/commit/83b26a1467359d0aeff50e8dd6eb7a558829f42a"
        },
        "date": 1688471268996,
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
            "value": 74,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 111,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 78,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 90,
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
            "value": 15,
            "range": "± 0",
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
            "value": 71,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 663,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 606,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 313,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 27,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 127,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 25,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 38,
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
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "taranov.vv@gmail.com",
            "name": "Vasiliy Taranov",
            "username": "oriontvv"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e24c63dffd69ba6ba56c2b1d3688267c69b06f41",
          "message": "Add dependabot (#225)",
          "timestamp": "2024-02-10T22:54:07+09:00",
          "tree_id": "8e684378b9ded4a88ddc41c8d9139c5275e4fca4",
          "url": "https://github.com/lloydmeta/frunk/commit/e24c63dffd69ba6ba56c2b1d3688267c69b06f41"
        },
        "date": 1707573331650,
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
            "value": 0,
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
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_hlist_pat_match",
            "value": 0,
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
            "value": 0,
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
            "value": 60,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 73,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 56,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 69,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 0,
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
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 12,
            "range": "± 0",
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
            "value": 38,
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
            "value": 52,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 647,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 450,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 223,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 35,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 102,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 29,
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
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "784efdd214cddfb0fd8686e3567a3a595119190a",
          "message": "Bump Swatinem/rust-cache from 1 to 2 (#226)\n\nBumps [Swatinem/rust-cache](https://github.com/swatinem/rust-cache) from 1 to 2.\r\n- [Release notes](https://github.com/swatinem/rust-cache/releases)\r\n- [Changelog](https://github.com/Swatinem/rust-cache/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/swatinem/rust-cache/compare/v1...v2)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: Swatinem/rust-cache\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2024-02-11T17:42:42+09:00",
          "tree_id": "e48b99b5dad733cee9843c12b480ca835e4534f0",
          "url": "https://github.com/lloydmeta/frunk/commit/784efdd214cddfb0fd8686e3567a3a595119190a"
        },
        "date": 1707641098269,
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
            "value": 0,
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
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_hlist_pat_match",
            "value": 0,
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
            "value": 0,
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
            "value": 57,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 70,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 54,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 66,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 0,
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
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 12,
            "range": "± 0",
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
            "value": 37,
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
            "value": 50,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 629,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 423,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 220,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 35,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 96,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 30,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 27,
            "range": "± 2",
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
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1d8fbc22ba433b284f1cb2664944b52e0e18afa0",
          "message": "Bump actions/checkout from 1 to 4 (#228)\n\nBumps [actions/checkout](https://github.com/actions/checkout) from 1 to 4.\r\n- [Release notes](https://github.com/actions/checkout/releases)\r\n- [Changelog](https://github.com/actions/checkout/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/actions/checkout/compare/v1...v4)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: actions/checkout\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-major\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>\r\nCo-authored-by: Lloyd <lloydmeta@users.noreply.github.com>",
          "timestamp": "2024-02-11T17:44:36+09:00",
          "tree_id": "6820266945babb9fa64a82abb5249031d95526d2",
          "url": "https://github.com/lloydmeta/frunk/commit/1d8fbc22ba433b284f1cb2664944b52e0e18afa0"
        },
        "date": 1707641161334,
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
            "value": 0,
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
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_hlist_pat_match",
            "value": 0,
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
            "value": 0,
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
            "value": 61,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 73,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 57,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 70,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 0,
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
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 12,
            "range": "± 0",
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
            "value": 39,
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
            "value": 52,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 644,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 438,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 224,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 35,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 101,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 29,
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
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0559a868fc978b4ce7773ee08804edb03b15306b",
          "message": "Update quickcheck requirement from 0.6.1 to 1.0.3 (#227)\n\n* Update quickcheck requirement from 0.6.1 to 1.0.3\r\n\r\nUpdates the requirements on [quickcheck](https://github.com/BurntSushi/quickcheck) to permit the latest version.\r\n- [Commits](https://github.com/BurntSushi/quickcheck/compare/quickcheck_macros-0.6.1...1.0.3)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: quickcheck\r\n  dependency-type: direct:production\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\n* Fixup wrapper implementations to mach updated trait\r\n\r\nSigned-off-by: lloydmeta <lloydmeta@gmail.com>\r\n\r\n---------\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nSigned-off-by: lloydmeta <lloydmeta@gmail.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>\r\nCo-authored-by: Lloyd <lloydmeta@users.noreply.github.com>\r\nCo-authored-by: lloydmeta <lloydmeta@gmail.com>",
          "timestamp": "2024-02-11T17:52:44+09:00",
          "tree_id": "bac288d3903f972660301aa4a933d8db16db652b",
          "url": "https://github.com/lloydmeta/frunk/commit/0559a868fc978b4ce7773ee08804edb03b15306b"
        },
        "date": 1707641652935,
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
            "value": 0,
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
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_hlist_pat_match",
            "value": 0,
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
            "value": 0,
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
            "value": 60,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 78,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 57,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 69,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 0,
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
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 12,
            "range": "± 0",
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
            "value": 38,
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
            "value": 52,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 649,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 443,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 226,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 35,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 101,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 29,
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
            "value": 1,
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
          "id": "c93709ddbfba577f330b71f533fe1b36663f931c",
          "message": "New Frunk laws release due to bumped quickcheck (#229)\n\nSigned-off-by: lloydmeta <lloydmeta@gmail.com>",
          "timestamp": "2024-02-11T18:01:05+09:00",
          "tree_id": "cdc48b9db30b104485d127d55d23b6bced5567c4",
          "url": "https://github.com/lloydmeta/frunk/commit/c93709ddbfba577f330b71f533fe1b36663f931c"
        },
        "date": 1707642135774,
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
            "value": 0,
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
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_hlist_pat_match",
            "value": 0,
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
            "value": 0,
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
            "value": 61,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 73,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 57,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 70,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 0,
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
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 12,
            "range": "± 0",
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
            "value": 38,
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
            "value": 52,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 655,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 437,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 224,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 35,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 101,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 30,
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
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1ca9c995b966ef204accd6b992401acfec302500",
          "message": "Bump peaceiris/actions-gh-pages from 3 to 4 (#230)",
          "timestamp": "2024-05-03T19:40:36+09:00",
          "tree_id": "fc3521b27454af03bf777150a07de351165d5f83",
          "url": "https://github.com/lloydmeta/frunk/commit/1ca9c995b966ef204accd6b992401acfec302500"
        },
        "date": 1714732905051,
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
            "value": 0,
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
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_hlist_pat_match",
            "value": 0,
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
            "value": 0,
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
            "value": 65,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 78,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 3718,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 3719,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 0,
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
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 12,
            "range": "± 0",
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
            "value": 36,
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
            "value": 55,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 691,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 474,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 217,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 101,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 28,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 30,
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
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "np@nathanperry.dev",
            "name": "Nathan Perry",
            "username": "mammothbane"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "93c4db74fac719764e3ce841f5169550efd1bb60",
          "message": "rework #![no_std] support, switch on `alloc` (#236)\n\n* rework #![no_std] support, switch on `alloc`\r\n\r\nMost of the uses of `std` in the crate are of `Vec`, `String`, and\r\n`Box`, which are in `alloc`, which is available on `#![no_std]`\r\nplatforms via `extern crate alloc;`. This commit adjusts these uses to\r\nuse `alloc` directly and adds an `\"alloc\"` feature flag to control them.\r\nThe notable exceptions still requiring a `std` flag are `HashMap` and\r\n`HashSet` (I'm working on a separate change to add `BTree{Map,Set}`\r\nsupport for `#![no_std]` envs). `frunk_core` no longer needs a `std`\r\nflag: it has been removed.\r\n\r\nThe reimport of `core` as `std` (see #220 for historical explanation) is\r\nconverted to direct usage of `core` everywhere.\r\n\r\nRemoved `#[cfg(feature = \"std\")]` condition from `frunk_laws` and made\r\nit depend on `frunk/std` -- it depends on `quickcheck`, which requires\r\n`std`.\r\n\r\nAdded `#[cfg(test)] extern crate std;` to `#![no_std]` crates, as `std`\r\nis required to run the libtest harness.\r\n\r\nTested working against all the combinations of feature flags I could\r\nthink of.\r\n\r\n* bump minor version, unremove frunk_core/std and mark deprecated\r\n\r\n* fix rustfmt",
          "timestamp": "2024-08-31T22:05:50+09:00",
          "tree_id": "27d2b7335bc5fc2290e617b578decfb3a65a3671",
          "url": "https://github.com/lloydmeta/frunk/commit/93c4db74fac719764e3ce841f5169550efd1bb60"
        },
        "date": 1725109628193,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty",
            "value": 0.31,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "generic_conversion",
            "value": 0.77,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "creating_hlist",
            "value": 1.55,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "creating_tuple2",
            "value": 1.55,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_append",
            "value": 2.47,
            "range": "± 0.02",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_hlist_pat_match",
            "value": 0.77,
            "range": "± 0.13",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2",
            "value": 1.55,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2_match",
            "value": 0.77,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_consuming",
            "value": 1.55,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_non_consuming",
            "value": 1.55,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_24fields",
            "value": 60.5,
            "range": "± 1.22",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 74.18,
            "range": "± 0.86",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 2615.34,
            "range": "± 34.57",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 2691.97,
            "range": "± 29.75",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 0.77,
            "range": "± 0.67",
            "unit": "ns/iter"
          },
          {
            "name": "name",
            "value": 0.31,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "sculpted_conversion",
            "value": 0.77,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 12.99,
            "range": "± 0.10",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_all_i32",
            "value": 2.35,
            "range": "± 0.04",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_mut",
            "value": 0.31,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_ref",
            "value": 0.31,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_value",
            "value": 0.31,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_mut",
            "value": 0.31,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_ref",
            "value": 0.31,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_value",
            "value": 0.31,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "combine_i32",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 35.09,
            "range": "± 0.26",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_i32",
            "value": 0.31,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_option_string",
            "value": 52.06,
            "range": "± 1.04",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 604.52,
            "range": "± 5.87",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 440.17,
            "range": "± 4.77",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 247.28,
            "range": "± 4.53",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 34.66,
            "range": "± 0.92",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 112.82,
            "range": "± 3.61",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 27.85,
            "range": "± 0.14",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 30.25,
            "range": "± 0.17",
            "unit": "ns/iter"
          },
          {
            "name": "ok_result_into_validated",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "validated_to_result",
            "value": 1.24,
            "range": "± 0.01",
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
          "id": "f413e1d4a4b73a7880c7405d42512ac005f6b5b0",
          "message": "0.4.3 release (#237)\n\n* 0.4.3 release\r\n* Changelog\r\n\r\nSigned-off-by: lloydmeta <lloydmeta@gmail.com>",
          "timestamp": "2024-08-31T22:22:25+09:00",
          "tree_id": "0c6494b7545c6e7465b79c4c1d1594d8db9e0043",
          "url": "https://github.com/lloydmeta/frunk/commit/f413e1d4a4b73a7880c7405d42512ac005f6b5b0"
        },
        "date": 1725110625532,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty",
            "value": 0.31,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "generic_conversion",
            "value": 0.77,
            "range": "± 0.02",
            "unit": "ns/iter"
          },
          {
            "name": "creating_hlist",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "creating_tuple2",
            "value": 1.55,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_append",
            "value": 2.47,
            "range": "± 0.03",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_hlist_pat_match",
            "value": 0.77,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2",
            "value": 1.55,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2_match",
            "value": 0.77,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_consuming",
            "value": 1.55,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_non_consuming",
            "value": 1.55,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_24fields",
            "value": 60.65,
            "range": "± 1.30",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 74.22,
            "range": "± 1.23",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 2670.19,
            "range": "± 47.78",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 2781.4,
            "range": "± 53.29",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 0.77,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "name",
            "value": 0.62,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "sculpted_conversion",
            "value": 0.77,
            "range": "± 0.02",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 12.99,
            "range": "± 0.19",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_all_i32",
            "value": 2.35,
            "range": "± 0.08",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_mut",
            "value": 0.31,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_ref",
            "value": 0.31,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_value",
            "value": 0.31,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_mut",
            "value": 0.31,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_ref",
            "value": 0.31,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_value",
            "value": 0.31,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "combine_i32",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 35.22,
            "range": "± 0.41",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_i32",
            "value": 0.31,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_option_string",
            "value": 52.3,
            "range": "± 0.93",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 611.4,
            "range": "± 9.82",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 480.48,
            "range": "± 3.21",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 226.65,
            "range": "± 2.25",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 34.88,
            "range": "± 0.64",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 101.62,
            "range": "± 1.36",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 27.8,
            "range": "± 0.19",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 29.98,
            "range": "± 0.16",
            "unit": "ns/iter"
          },
          {
            "name": "ok_result_into_validated",
            "value": 1.08,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "validated_to_result",
            "value": 1.24,
            "range": "± 0.01",
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
          "id": "ffba8cfa144b8ed02942271a3ea45006100cd029",
          "message": "Add a test to check that LabelledGeneric works with keyword fields (#238)\n\nChecks that derives works and transmogrifying works.\r\n\r\nSigned-off-by: lloydmeta <lloydmeta@gmail.com>",
          "timestamp": "2024-11-09T23:27:46+09:00",
          "tree_id": "0eb4ae4948dab78157d4d055240c6f49b1fd05db",
          "url": "https://github.com/lloydmeta/frunk/commit/ffba8cfa144b8ed02942271a3ea45006100cd029"
        },
        "date": 1731162518208,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty",
            "value": 0.31,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "generic_conversion",
            "value": 0.77,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "creating_hlist",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "creating_tuple2",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_append",
            "value": 2.47,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_hlist_pat_match",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2_match",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_consuming",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_non_consuming",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_24fields",
            "value": 61,
            "range": "± 0.56",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 75.11,
            "range": "± 0.76",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 2609.23,
            "range": "± 29.34",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 2562.98,
            "range": "± 27.33",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 0.77,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "name",
            "value": 0.62,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "sculpted_conversion",
            "value": 0.77,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 13.3,
            "range": "± 0.03",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_all_i32",
            "value": 2.35,
            "range": "± 0.07",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_mut",
            "value": 0.31,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_ref",
            "value": 0.31,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_value",
            "value": 0.31,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_mut",
            "value": 0.31,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_ref",
            "value": 0.31,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_value",
            "value": 0.31,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "combine_i32",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 35.18,
            "range": "± 0.26",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_i32",
            "value": 0.31,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_option_string",
            "value": 53.66,
            "range": "± 0.60",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 618.24,
            "range": "± 11.16",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 442.53,
            "range": "± 4.70",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 225.54,
            "range": "± 120.80",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 36.87,
            "range": "± 6.04",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 101.47,
            "range": "± 29.73",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 27.89,
            "range": "± 1.05",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 30.31,
            "range": "± 0.53",
            "unit": "ns/iter"
          },
          {
            "name": "ok_result_into_validated",
            "value": 1.54,
            "range": "± 0.05",
            "unit": "ns/iter"
          },
          {
            "name": "validated_to_result",
            "value": 1.24,
            "range": "± 0.01",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "account@kgtkr.net",
            "name": "kgtkr",
            "username": "kgtkr"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6130a385cbb75136097144e31d041b732bec5b3d",
          "message": "Implemented a reference version of Plucker/ByNameFieldPlucker (#240)\n\n* Implemented a reference version of Plucker/ByNameFieldPlucker\n\n* fixup! Implemented a reference version of Plucker/ByNameFieldPlucker\n\n* fix clippy\n\n* update CHANGELOG",
          "timestamp": "2025-03-06T19:52:48+09:00",
          "tree_id": "86205658dd85dd2815861874559555d255c91868",
          "url": "https://github.com/lloydmeta/frunk/commit/6130a385cbb75136097144e31d041b732bec5b3d"
        },
        "date": 1741258431899,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty",
            "value": 0.31,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "generic_conversion",
            "value": 0.78,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "creating_hlist",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "creating_tuple2",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_append",
            "value": 2.49,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_hlist_pat_match",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2",
            "value": 1.55,
            "range": "± 0.03",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2_match",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_consuming",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_non_consuming",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_24fields",
            "value": 58.54,
            "range": "± 0.69",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 74.3,
            "range": "± 1.72",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 2630.06,
            "range": "± 37.99",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 2585.78,
            "range": "± 32.64",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 0.78,
            "range": "± 0.02",
            "unit": "ns/iter"
          },
          {
            "name": "name",
            "value": 0.31,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "sculpted_conversion",
            "value": 0.78,
            "range": "± 0.02",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 13.06,
            "range": "± 0.06",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_all_i32",
            "value": 2.37,
            "range": "± 0.42",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_mut",
            "value": 0.31,
            "range": "± 0.02",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_ref",
            "value": 0.31,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_value",
            "value": 0.31,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_mut",
            "value": 0.31,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_ref",
            "value": 0.31,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_value",
            "value": 0.31,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "combine_i32",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 31.79,
            "range": "± 1.13",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_i32",
            "value": 0.31,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_option_string",
            "value": 49.83,
            "range": "± 0.73",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 602.71,
            "range": "± 7.92",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 443.41,
            "range": "± 11.14",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 227.49,
            "range": "± 3.45",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 34.83,
            "range": "± 0.84",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 104.68,
            "range": "± 1.38",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 27.98,
            "range": "± 1.26",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 29.21,
            "range": "± 0.72",
            "unit": "ns/iter"
          },
          {
            "name": "ok_result_into_validated",
            "value": 1.55,
            "range": "± 0.02",
            "unit": "ns/iter"
          },
          {
            "name": "validated_to_result",
            "value": 1.24,
            "range": "± 0.00",
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
          "id": "a20698ae018fceccdf99325a3b1e758ac53a729d",
          "message": "Release 0.4.4 (#241)\n\nSigned-off-by: lloydmeta <lloydmeta@gmail.com>",
          "timestamp": "2025-06-14T20:07:55+09:00",
          "tree_id": "625d18a4fe95242967e2e6e0392ab9d2b9af132b",
          "url": "https://github.com/lloydmeta/frunk/commit/a20698ae018fceccdf99325a3b1e758ac53a729d"
        },
        "date": 1749899323788,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty",
            "value": 0.31,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "generic_conversion",
            "value": 0.78,
            "range": "± 0.03",
            "unit": "ns/iter"
          },
          {
            "name": "creating_hlist",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "creating_tuple2",
            "value": 1.55,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_append",
            "value": 2.67,
            "range": "± 0.02",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_hlist_pat_match",
            "value": 1.55,
            "range": "± 0.02",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2_match",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_consuming",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_non_consuming",
            "value": 1.55,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_24fields",
            "value": 59.52,
            "range": "± 1.97",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 72.92,
            "range": "± 1.04",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 2816.67,
            "range": "± 30.83",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 2741.88,
            "range": "± 17.53",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 0.78,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "name",
            "value": 0.62,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "sculpted_conversion",
            "value": 0.78,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 13.35,
            "range": "± 0.12",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_all_i32",
            "value": 2.48,
            "range": "± 0.02",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_mut",
            "value": 0.62,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_ref",
            "value": 0.62,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_value",
            "value": 0.62,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_mut",
            "value": 0.62,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_ref",
            "value": 0.62,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_value",
            "value": 0.62,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "combine_i32",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 32.43,
            "range": "± 0.51",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_i32",
            "value": 0.31,
            "range": "± 0.00",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_option_string",
            "value": 49.99,
            "range": "± 0.59",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 605.33,
            "range": "± 133.24",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 425.37,
            "range": "± 28.71",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 223.79,
            "range": "± 4.14",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 36.97,
            "range": "± 2.52",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 100.7,
            "range": "± 1.21",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 27.98,
            "range": "± 1.12",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 29.22,
            "range": "± 27.42",
            "unit": "ns/iter"
          },
          {
            "name": "ok_result_into_validated",
            "value": 1.55,
            "range": "± 0.02",
            "unit": "ns/iter"
          },
          {
            "name": "validated_to_result",
            "value": 1.24,
            "range": "± 0.01",
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
          "id": "b085ceb6a92b050842a636fb44e8961f448d5bcf",
          "message": "[clippy-fix] Add dead code allow for things clippy can't find (#247)\n\nSigned-off-by: lloydmeta <lloydmeta@gmail.com>",
          "timestamp": "2025-12-07T15:45:51+09:00",
          "tree_id": "d214dd50a68538125e82b851b3d811c339488c6e",
          "url": "https://github.com/lloydmeta/frunk/commit/b085ceb6a92b050842a636fb44e8961f448d5bcf"
        },
        "date": 1765090005129,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty",
            "value": 0.31,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "generic_conversion",
            "value": 0.78,
            "range": "± 0.02",
            "unit": "ns/iter"
          },
          {
            "name": "creating_hlist",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "creating_tuple2",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_append",
            "value": 2.49,
            "range": "± 0.02",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_hlist_pat_match",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2_match",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_consuming",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_non_consuming",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_24fields",
            "value": 60.05,
            "range": "± 0.9",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 74.67,
            "range": "± 1.96",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 2571.11,
            "range": "± 25.79",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 2638.66,
            "range": "± 27.87",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 0.78,
            "range": "± 0.07",
            "unit": "ns/iter"
          },
          {
            "name": "name",
            "value": 0.31,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "sculpted_conversion",
            "value": 0.78,
            "range": "± 0.06",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 13.05,
            "range": "± 0.12",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_all_i32",
            "value": 2.36,
            "range": "± 0.04",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_mut",
            "value": 0.31,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_ref",
            "value": 0.31,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_value",
            "value": 0.31,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_mut",
            "value": 0.31,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_ref",
            "value": 0.31,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_value",
            "value": 0.31,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_i32",
            "value": 1.55,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 32.73,
            "range": "± 0.39",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_i32",
            "value": 0.31,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_option_string",
            "value": 50.87,
            "range": "± 0.47",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 645.52,
            "range": "± 10.37",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 476.87,
            "range": "± 384.11",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 233.45,
            "range": "± 3.79",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 25.43,
            "range": "± 0.18",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 107.44,
            "range": "± 1.61",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 15.53,
            "range": "± 0.07",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 32,
            "range": "± 0.21",
            "unit": "ns/iter"
          },
          {
            "name": "ok_result_into_validated",
            "value": 1.24,
            "range": "± 0.02",
            "unit": "ns/iter"
          },
          {
            "name": "validated_to_result",
            "value": 0.62,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1efcf31fbdfa99d8972b632ecf9df46858d2925c",
          "message": "Bump actions/checkout from 4 to 6 (#246)\n\nBumps [actions/checkout](https://github.com/actions/checkout) from 4 to 6.\n- [Release notes](https://github.com/actions/checkout/releases)\n- [Changelog](https://github.com/actions/checkout/blob/main/CHANGELOG.md)\n- [Commits](https://github.com/actions/checkout/compare/v4...v6)\n\n---\nupdated-dependencies:\n- dependency-name: actions/checkout\n  dependency-version: '6'\n  dependency-type: direct:production\n  update-type: version-update:semver-major\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2025-12-07T15:49:57+09:00",
          "tree_id": "f43c061bf3e4a9c2d92daba0da183af775698617",
          "url": "https://github.com/lloydmeta/frunk/commit/1efcf31fbdfa99d8972b632ecf9df46858d2925c"
        },
        "date": 1765090246310,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty",
            "value": 0.31,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "generic_conversion",
            "value": 0.78,
            "range": "± 0.02",
            "unit": "ns/iter"
          },
          {
            "name": "creating_hlist",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "creating_tuple2",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_append",
            "value": 2.48,
            "range": "± 0.02",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_hlist_pat_match",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_into_tuple2_match",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_consuming",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "hlist_mapping_non_consuming",
            "value": 1.55,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_24fields",
            "value": 60.16,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "big_from_25fields",
            "value": 74.93,
            "range": "± 1.1",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_24fields",
            "value": 2680.43,
            "range": "± 29.66",
            "unit": "ns/iter"
          },
          {
            "name": "big_transform_from_25fields",
            "value": 2826.44,
            "range": "± 24.52",
            "unit": "ns/iter"
          },
          {
            "name": "labelled_conversion",
            "value": 0.78,
            "range": "± 0.03",
            "unit": "ns/iter"
          },
          {
            "name": "name",
            "value": 0.31,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sculpted_conversion",
            "value": 0.78,
            "range": "± 0.02",
            "unit": "ns/iter"
          },
          {
            "name": "combine_all_i32",
            "value": 13.04,
            "range": "± 0.1",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_all_i32",
            "value": 2.36,
            "range": "± 0.03",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_mut",
            "value": 0.31,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_ref",
            "value": 0.31,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "lens_path_read_value",
            "value": 0.31,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_mut",
            "value": 0.31,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_ref",
            "value": 0.31,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "normal_path_read_value",
            "value": 0.31,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_i32",
            "value": 1.55,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "combine_option_string",
            "value": 32.51,
            "range": "± 0.52",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_i32",
            "value": 0.31,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "std_add_option_string",
            "value": 50.98,
            "range": "± 0.68",
            "unit": "ns/iter"
          },
          {
            "name": "manual_deep_from",
            "value": 622.51,
            "range": "± 7.19",
            "unit": "ns/iter"
          },
          {
            "name": "transmogrify_deep",
            "value": 477.4,
            "range": "± 10.82",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_bad",
            "value": 247.45,
            "range": "± 4.21",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_all_good",
            "value": 25.45,
            "range": "± 0.09",
            "unit": "ns/iter"
          },
          {
            "name": "adding_result_to_validated_mixed",
            "value": 104.6,
            "range": "± 1.28",
            "unit": "ns/iter"
          },
          {
            "name": "adding_validateds",
            "value": 15.55,
            "range": "± 0.09",
            "unit": "ns/iter"
          },
          {
            "name": "error_result_into_validated",
            "value": 31.99,
            "range": "± 0.36",
            "unit": "ns/iter"
          },
          {
            "name": "ok_result_into_validated",
            "value": 1.24,
            "range": "± 0.02",
            "unit": "ns/iter"
          },
          {
            "name": "validated_to_result",
            "value": 0.62,
            "range": "± 0.03",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}