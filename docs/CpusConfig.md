# CpusConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**boot_vcpus** | **i32** |  | 
**max_vcpus** | **i32** |  | 
**topology** | Option<[**models::CpuTopology**](CpuTopology.md)> |  | [optional]
**kvm_hyperv** | Option<**bool**> |  | [optional][default to false]
**kvm_hint_realtime** | Option<**bool**> | Advertise KVM_HINTS_REALTIME to the guest (x86-64 with KVM only) | [optional][default to false]
**max_phys_bits** | Option<**i32**> |  | [optional]
**nested** | Option<**bool**> |  | [optional][default to false]
**affinity** | Option<[**Vec<models::CpuAffinity>**](CpuAffinity.md)> |  | [optional]
**features** | Option<[**models::CpuFeatures**](CpuFeatures.md)> |  | [optional]
**core_scheduling** | Option<[**models::CoreSchedulingMode**](CoreSchedulingMode.md)> |  | [optional]
**profile** | Option<[**models::CpuProfileConfig**](CpuProfileConfig.md)> |  | [optional]
**x_cpuid_overrides** | Option<[**Vec<models::CpuidOverride>**](CpuidOverride.md)> | Experimental x86-64 common CPUID register overrides, applied in order after the selected CPU profile and before per-vCPU adjustments. The caller must ensure host compatibility.  | [optional][default to []]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


