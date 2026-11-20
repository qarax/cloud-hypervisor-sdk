# TokenBucket

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**size** | **i64** | The total number of tokens this bucket can hold. | 
**one_time_burst** | Option<**i64**> | The initial size of a token bucket. | [optional]
**refill_time** | **i64** | The amount of milliseconds it takes for the bucket to refill. | 
**burst_rate** | Option<**i64**> | The peak rate in tokens per second (bytes/s for bandwidth, operations/s for ops). Enables bounded bursts. | [optional]
**burst_seconds** | Option<**i64**> | The burst length in seconds, with depth equal to burst_rate times burst_seconds. Requires burst_rate and defaults to 1 when burst_rate is set. | [optional]
**burst_recharge_seconds** | Option<**i64**> | The seconds to regain full burst credit from empty while idle. Requires burst_rate and defaults to burst_rate times burst_seconds divided by the sustained rate. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


