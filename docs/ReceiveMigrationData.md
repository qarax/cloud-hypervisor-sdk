# ReceiveMigrationData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**receiver_url** | **String** |  | 
**tls_dir** | Option<**String**> | Directory containing the TLS server certificate (server-cert.pem), the TLS server key (server-key.pem), and the client TLS root CA certificate (ca-cert.pem). TLS is only supported with tcp:<host>:<port> receiver URLs. Recommended for all TCP migrations. Omitting it is only intended for development and testing, as TCP migration is then unencrypted and unauthenticated.  | [optional]
**zone_updates** | Option<[**Vec<models::VmMemoryZoneUpdateData>**](VmMemoryZoneUpdateData.md)> |  | [optional]
**background_fetch_pages** | Option<**bool**> | Enable the background thread for faulting all remaining pages. | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


