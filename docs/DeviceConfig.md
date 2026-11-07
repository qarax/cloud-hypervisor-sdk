# DeviceConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**path** | Option<**String**> | Sysfs path of the VFIO device. Exactly one of `path` or an externally-opened cdev FD must be supplied; an FD is passed out of band via SCM_RIGHTS on the UNIX domain socket, never in this body.  | [optional]
**iommu** | Option<[**models::IommuType**](IommuType.md)> |  | [optional]
**pci_segment** | Option<**i16**> |  | [optional]
**pci_device_id** | Option<**u8**> |  | [optional]
**id** | Option<**String**> |  | [optional]
**x_nv_gpudirect_clique** | Option<**i8**> |  | [optional]
**x_exclude_mmap_bars** | Option<**Vec<i64>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


