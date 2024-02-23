# CpusConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**boot_vcpus** | **i32** |  | [default to 1]
**max_vcpus** | **i32** |  | [default to 1]
**topology** | Option<[**crate::models::CpuTopology**](CpuTopology.md)> |  | [optional]
**kvm_hyperv** | Option<**bool**> |  | [optional][default to false]
**max_phys_bits** | Option<**i32**> |  | [optional]
**affinity** | Option<[**Vec<crate::models::CpuAffinity>**](CpuAffinity.md)> |  | [optional]
**features** | Option<[**crate::models::CpuFeatures**](CpuFeatures.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


