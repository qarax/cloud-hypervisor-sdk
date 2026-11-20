# SerialConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**file** | Option<**String**> | Required if `mode` is `File`. For `Pty`, it holds the path of the PTY once the device has been created.  | [optional]
**socket** | Option<**String**> | Required if `mode` is `Socket`. | [optional]
**tcp** | Option<[**models::TcpConsoleConfig**](TcpConsoleConfig.md)> | Required if `mode` is `Tcp`. | [optional]
**mode** | [**models::ConsoleMode**](ConsoleMode.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


