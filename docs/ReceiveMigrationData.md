# ReceiveMigrationData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**receiver_url** | **String** |  | 
**vfio_fds** | Option<[**Vec<models::RestoredVfioConfig>**](RestoredVfioConfig.md)> | Deprecated: use external_fds with Vfio targets instead.  | [optional]
**iommufd_fd** | Option<**i32**> | Deprecated: use external_fds with an Iommu target instead.  | [optional]
**tls_dir** | Option<**String**> | Directory containing the TLS server certificate (server-cert.pem), the TLS server key (server-key.pem), and the client TLS root CA certificate (ca-cert.pem). TLS is only supported with tcp:<host>:<port> receiver URLs. Recommended for all TCP migrations. Omitting it is only intended for development and testing, as TCP migration is then unencrypted and unauthenticated.  | [optional]
**zone_updates** | Option<[**Vec<models::VmMemoryZoneUpdateData>**](VmMemoryZoneUpdateData.md)> |  | [optional]
**background_fetch_pages** | Option<**bool**> | Enable the background thread for faulting all remaining pages. | [optional][default to true]
**serial** | Option<[**models::SerialConfig**](SerialConfig.md)> | New output of the serial console, in the mode of the source VM. | [optional]
**console** | Option<[**models::SerialConfig**](SerialConfig.md)> | New output of the virtio console, in the mode of the source VM. | [optional]
**external_fds** | Option<[**Vec<models::RestoreConfigExternalFdsInner>**](RestoreConfigExternalFdsInner.md)> | File descriptor entries and counts. The corresponding descriptors must be supplied out of band via SCM_RIGHTS on the API UNIX socket. Do not combine external_fds with legacy FD fields.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


