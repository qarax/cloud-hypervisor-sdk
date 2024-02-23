# VmConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cpus** | Option<[**crate::models::CpusConfig**](CpusConfig.md)> |  | [optional]
**memory** | Option<[**crate::models::MemoryConfig**](MemoryConfig.md)> |  | [optional]
**payload** | [**crate::models::PayloadConfig**](PayloadConfig.md) |  | 
**rate_limit_groups** | Option<[**Vec<crate::models::RateLimitGroupConfig>**](RateLimitGroupConfig.md)> |  | [optional]
**disks** | Option<[**Vec<crate::models::DiskConfig>**](DiskConfig.md)> |  | [optional]
**net** | Option<[**Vec<crate::models::NetConfig>**](NetConfig.md)> |  | [optional]
**rng** | Option<[**crate::models::RngConfig**](RngConfig.md)> |  | [optional]
**balloon** | Option<[**crate::models::BalloonConfig**](BalloonConfig.md)> |  | [optional]
**fs** | Option<[**Vec<crate::models::FsConfig>**](FsConfig.md)> |  | [optional]
**pmem** | Option<[**Vec<crate::models::PmemConfig>**](PmemConfig.md)> |  | [optional]
**serial** | Option<[**crate::models::ConsoleConfig**](ConsoleConfig.md)> |  | [optional]
**console** | Option<[**crate::models::ConsoleConfig**](ConsoleConfig.md)> |  | [optional]
**debug_console** | Option<[**crate::models::DebugConsoleConfig**](DebugConsoleConfig.md)> |  | [optional]
**devices** | Option<[**Vec<crate::models::DeviceConfig>**](DeviceConfig.md)> |  | [optional]
**vdpa** | Option<[**Vec<crate::models::VdpaConfig>**](VdpaConfig.md)> |  | [optional]
**vsock** | Option<[**crate::models::VsockConfig**](VsockConfig.md)> |  | [optional]
**sgx_epc** | Option<[**Vec<crate::models::SgxEpcConfig>**](SgxEpcConfig.md)> |  | [optional]
**numa** | Option<[**Vec<crate::models::NumaConfig>**](NumaConfig.md)> |  | [optional]
**iommu** | Option<**bool**> |  | [optional][default to false]
**watchdog** | Option<**bool**> |  | [optional][default to false]
**platform** | Option<[**crate::models::PlatformConfig**](PlatformConfig.md)> |  | [optional]
**tpm** | Option<[**crate::models::TpmConfig**](TpmConfig.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


