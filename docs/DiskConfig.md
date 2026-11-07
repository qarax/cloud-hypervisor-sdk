# DiskConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**path** | Option<**String**> |  | [optional]
**readonly** | Option<**bool**> |  | [optional][default to false]
**direct** | Option<**bool**> |  | [optional][default to false]
**iommu** | Option<[**models::IommuType**](IommuType.md)> |  | [optional]
**num_queues** | Option<**i32**> |  | [optional][default to 1]
**queue_size** | Option<**i32**> |  | [optional][default to 128]
**vhost_user** | Option<**bool**> |  | [optional][default to false]
**vhost_socket** | Option<**String**> |  | [optional]
**rate_limiter_config** | Option<[**models::RateLimiterConfig**](RateLimiterConfig.md)> |  | [optional]
**pci_segment** | Option<**i16**> |  | [optional]
**pci_device_id** | Option<**u8**> |  | [optional]
**id** | Option<**String**> |  | [optional]
**serial** | Option<**String**> |  | [optional]
**rate_limit_group** | Option<**String**> |  | [optional]
**queue_affinity** | Option<[**Vec<models::VirtQueueAffinity>**](VirtQueueAffinity.md)> |  | [optional]
**backing_files** | Option<**bool**> |  | [optional][default to false]
**sparse** | Option<**bool**> |  | [optional][default to true]
**image_type** | Option<[**models::ImageType**](ImageType.md)> |  | [optional]
**lock_granularity** | Option<[**models::LockGranularity**](LockGranularity.md)> |  | [optional]
**guest_block_size** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


