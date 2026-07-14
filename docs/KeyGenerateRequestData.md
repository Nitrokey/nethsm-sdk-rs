# KeyGenerateRequestData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mechanisms** | [**Vec<crate::models::KeyMechanism>**](KeyMechanism.md) |  | 
**r#type** | [**crate::models::KeyType**](KeyType.md) |  | 
**length** | Option<**i32**> |  | [optional]
**id** | Option<**String**> |  | [optional]
**restrictions** | Option<[**crate::models::KeyRestrictions**](KeyRestrictions.md)> |  | [optional]
**label** | Option<**String**> | A valid UTF-8 string. For interoperability with PKCS#11 its length shouldn't exceed 32 bytes. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


