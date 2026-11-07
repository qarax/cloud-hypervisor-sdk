# SendMigrationData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**destination_url** | **String** |  | 
**local** | Option<**bool**> | Deprecated: set memory_mode to \"MemFDs\" instead. If true, the request is treated as if memory_mode were \"MemFDs\".  | [optional][default to false]
**preserve_source** | Option<**bool**> | Keep the source VM alive in a paused state after migration completes. This is only supported when memory_mode is \"MemFDs\" or the deprecated local property is true.  | [optional][default to false]
**downtime_ms** | Option<**i64**> | The maximum downtime the migration aims for, in milliseconds. Defaults to 300ms.  | [optional][default to 300]
**timeout_s** | Option<**i64**> | The timeout for the migration (maximum total duration), in seconds. Defaults to 3600s (one hour).  | [optional][default to 3600]
**timeout_strategy** | Option<[**models::TimeoutStrategy**](TimeoutStrategy.md)> |  | [optional]
**connections** | Option<**i64**> | The number of parallel TCP connections to use for migration. Must be between 1 and 128. Multiple connections are not supported with UNIX domain socket migration.  | [optional][default to 1]
**tls_dir** | Option<**String**> | Directory containing the TLS root CA certificate (ca-cert.pem), the TLS client certificate (client-cert.pem), and TLS client key (client-key.pem). TLS is only supported with tcp:<host>:<port> destination URLs. Recommended for all TCP migrations. Omitting it is only intended for development and testing, as TCP migration is then unencrypted and unauthenticated.  | [optional]
**memory_mode** | Option<[**models::MigrationMode**](MigrationMode.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


