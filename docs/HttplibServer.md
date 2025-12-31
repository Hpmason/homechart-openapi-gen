# HttplibServer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**listen_address** | Option<**String**> |  | [optional]
**rate_limit_key** | Option<**String**> |  | [optional]
**rate_limit_patterns** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**timeout_read** | Option<[**models::TypesDuration**](types.Duration.md)> |  | [optional]
**timeout_read_header** | Option<[**models::TypesDuration**](types.Duration.md)> |  | [optional]
**timeout_write** | Option<[**models::TypesDuration**](types.Duration.md)> |  | [optional]
**tls_certificate_base64** | Option<**String**> |  | [optional]
**tls_certificate_path** | Option<**String**> |  | [optional]
**tls_client_auth_ca_base64** | Option<**String**> |  | [optional]
**tls_client_auth_ca_path** | Option<**String**> |  | [optional]
**tls_key_base64** | Option<**String**> |  | [optional]
**tls_key_path** | Option<**String**> |  | [optional]
**x_forwarded_trusted_cidrs** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


