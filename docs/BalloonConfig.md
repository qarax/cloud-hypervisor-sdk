# BalloonConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**pci_segment** | Option<**i16**> |  | [optional]
**pci_device_id** | Option<**u8**> |  | [optional]
**iommu** | Option<[**models::IommuType**](IommuType.md)> |  | [optional]
**size** | **i64** |  | 
**deflate_on_oom** | Option<**bool**> | Deflate balloon when the guest is under memory pressure. | [optional][default to false]
**free_page_reporting** | Option<**bool**> | Enable guest to report free pages. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


