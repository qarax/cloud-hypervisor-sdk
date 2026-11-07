# VsockConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cid** | **i64** | Guest Vsock CID | 
**socket** | **String** | Path to UNIX domain socket, used to proxy vsock connections. | 
**iommu** | Option<[**models::IommuType**](IommuType.md)> |  | [optional]
**pci_segment** | Option<**i16**> |  | [optional]
**pci_device_id** | Option<**u8**> |  | [optional]
**id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


