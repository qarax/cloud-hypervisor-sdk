# VmConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cpus** | Option<[**models::CpusConfig**](CpusConfig.md)> |  | [optional]
**memory** | Option<[**models::MemoryConfig**](MemoryConfig.md)> |  | [optional]
**payload** | [**models::PayloadConfig**](PayloadConfig.md) |  | 
**rate_limit_groups** | Option<[**Vec<models::RateLimitGroupConfig>**](RateLimitGroupConfig.md)> |  | [optional]
**disks** | Option<[**Vec<models::DiskConfig>**](DiskConfig.md)> |  | [optional]
**net** | Option<[**Vec<models::NetConfig>**](NetConfig.md)> |  | [optional]
**rng** | Option<[**models::RngConfig**](RngConfig.md)> |  | [optional]
**balloon** | Option<[**models::BalloonConfig**](BalloonConfig.md)> |  | [optional]
**fs** | Option<[**Vec<models::FsConfig>**](FsConfig.md)> |  | [optional]
**generic_vhost_user** | Option<[**Vec<models::GenericVhostUserConfig>**](GenericVhostUserConfig.md)> |  | [optional]
**pmem** | Option<[**Vec<models::PmemConfig>**](PmemConfig.md)> |  | [optional]
**serial** | Option<[**models::SerialConfig**](SerialConfig.md)> |  | [optional]
**console** | Option<[**models::ConsoleConfig**](ConsoleConfig.md)> |  | [optional]
**debug_console** | Option<[**models::DebugConsoleConfig**](DebugConsoleConfig.md)> |  | [optional]
**devices** | Option<[**Vec<models::DeviceConfig>**](DeviceConfig.md)> |  | [optional]
**user_devices** | Option<[**Vec<models::UserDeviceConfig>**](UserDeviceConfig.md)> |  | [optional]
**vdpa** | Option<[**Vec<models::VdpaConfig>**](VdpaConfig.md)> |  | [optional]
**vsock** | Option<[**models::VsockConfig**](VsockConfig.md)> |  | [optional]
**numa** | Option<[**Vec<models::NumaConfig>**](NumaConfig.md)> |  | [optional]
**watchdog** | Option<**bool**> |  | [optional][default to false]
**rtc** | Option<[**models::RtcConfig**](RtcConfig.md)> |  | [optional]
**pvpanic** | Option<**bool**> |  | [optional][default to false]
**pci_segments** | Option<[**Vec<models::PciSegmentConfig>**](PciSegmentConfig.md)> |  | [optional]
**platform** | Option<[**models::PlatformConfig**](PlatformConfig.md)> |  | [optional]
**tpm** | Option<[**models::TpmConfig**](TpmConfig.md)> |  | [optional]
**landlock_enable** | Option<**bool**> |  | [optional][default to false]
**landlock_rules** | Option<[**Vec<models::LandlockConfig>**](LandlockConfig.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


