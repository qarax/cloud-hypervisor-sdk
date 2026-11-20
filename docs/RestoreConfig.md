# RestoreConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_url** | **String** |  | 
**prefault** | Option<**bool**> |  | [optional]
**memory_restore_mode** | Option<[**models::MemoryRestoreMode**](MemoryRestoreMode.md)> |  | [optional]
**net_fds** | Option<[**Vec<models::RestoredNetConfig>**](RestoredNetConfig.md)> | Deprecated: use external_fds with Net targets instead.  | [optional]
**vfio_fds** | Option<[**Vec<models::RestoredVfioConfig>**](RestoredVfioConfig.md)> | Deprecated: use external_fds with Vfio targets instead.  | [optional]
**iommufd_fd** | Option<**i32**> | Deprecated: use external_fds with an Iommu target instead.  | [optional]
**resume** | Option<**bool**> |  | [optional]
**zone_updates** | Option<[**Vec<models::VmMemoryZoneUpdateData>**](VmMemoryZoneUpdateData.md)> |  | [optional]
**serial** | Option<[**models::SerialConfig**](SerialConfig.md)> | New output of the serial console, in the mode of the snapshot. | [optional]
**console** | Option<[**models::SerialConfig**](SerialConfig.md)> | New output of the virtio console, in the mode of the snapshot. | [optional]
**external_fds** | Option<[**Vec<models::RestoreConfigExternalFdsInner>**](RestoreConfigExternalFdsInner.md)> | File descriptor entries and counts. The corresponding descriptors must be supplied out of band via SCM_RIGHTS on the API UNIX socket. Do not combine external_fds with legacy FD fields.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


