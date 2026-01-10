# NetConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tap** | Option<**String**> |  | [optional]
**ip** | Option<**String**> | IPv4 or IPv6 address | [optional][default to 192.168.249.1]
**mask** | Option<**String**> | Must be a valid IPv4 netmask if ip is an IPv4 address or a valid IPv6 netmask if ip is an IPv6 address. | [optional][default to 255.255.255.0]
**mac** | Option<**String**> |  | [optional]
**host_mac** | Option<**String**> |  | [optional]
**mtu** | Option<**i32**> |  | [optional]
**iommu** | Option<**bool**> |  | [optional][default to false]
**num_queues** | Option<**i32**> |  | [optional][default to 2]
**queue_size** | Option<**i32**> |  | [optional][default to 256]
**vhost_user** | Option<**bool**> |  | [optional][default to false]
**vhost_socket** | Option<**String**> |  | [optional]
**vhost_mode** | Option<**String**> |  | [optional][default to Client]
**id** | Option<**String**> |  | [optional]
**pci_segment** | Option<**i32**> |  | [optional]
**rate_limiter_config** | Option<[**models::RateLimiterConfig**](RateLimiterConfig.md)> |  | [optional]
**offload_tso** | Option<**bool**> |  | [optional][default to true]
**offload_ufo** | Option<**bool**> |  | [optional][default to true]
**offload_csum** | Option<**bool**> |  | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


