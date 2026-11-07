# BalloonStats

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**swap_in** | Option<**i64**> | Memory swapped into the guest in bytes | [optional]
**swap_out** | Option<**i64**> | Memory swapped out by the guest in bytes | [optional]
**major_faults** | Option<**i64**> | Number of major page faults | [optional]
**minor_faults** | Option<**i64**> | Number of minor page faults | [optional]
**free_memory** | Option<**i64**> | Unused guest memory in bytes | [optional]
**total_memory** | Option<**i64**> | Total guest memory in bytes | [optional]
**available_memory** | Option<**i64**> | Guest memory available without swapping in bytes | [optional]
**disk_caches** | Option<**i64**> | Reclaimable guest disk caches in bytes | [optional]
**hugetlb_allocations** | Option<**i64**> | Number of successful hugetlb allocations | [optional]
**hugetlb_failures** | Option<**i64**> | Number of failed hugetlb allocations | [optional]
**oom_kills** | Option<**i64**> | Number of guest OOM kills reported by Linux | [optional]
**alloc_stalls** | Option<**i64**> | Number of guest memory allocation stalls reported by Linux | [optional]
**async_scans** | Option<**i64**> | Memory scanned asynchronously by Linux in bytes | [optional]
**direct_scans** | Option<**i64**> | Memory scanned directly by Linux in bytes | [optional]
**async_reclaims** | Option<**i64**> | Memory reclaimed asynchronously by Linux in bytes | [optional]
**direct_reclaims** | Option<**i64**> | Memory reclaimed directly by Linux in bytes | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


