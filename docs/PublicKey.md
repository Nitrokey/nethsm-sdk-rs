# PublicKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mechanisms** | [**Vec<crate::models::KeyMechanism>**](KeyMechanism.md) |  | 
**r#type** | [**crate::models::KeyType**](KeyType.md) |  | 
**restrictions** | [**crate::models::KeyRestrictions**](KeyRestrictions.md) |  | 
**public** | Option<[**crate::models::KeyPublicData**](KeyPublicData.md)> |  | [optional]
**operations** | **i32** |  | 
**label** | Option<**String**> | A valid UTF-8 string. For interoperability with PKCS#11 its length shouldn't exceed 32 bytes. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


