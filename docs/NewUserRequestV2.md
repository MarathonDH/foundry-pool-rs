# NewUserRequestV2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email_address** | **String** | Email Address of user | 
**full_name** | **String** | Full Name of new user - blank if adding an existing user to a group. | 
**preferred_name** | **String** | Preferred Name for new user - blank if adding an existing user to a group. | 
**groups** | [**Vec<models::UserGroupRequest>**](UserGroupRequest.md) | List of UserGroupRequest objects | 
**api_user** | Option<**bool**> | True for API-Only users. | [optional][default to false]
**ips** | Option<**Vec<String>**> | List of API User's IPs. Only valid if apiUser equals true. | [optional]
**sub_account_names** | Option<**Vec<String>**> | List of sub-accounts to add the user to. If null, user will be added to all sub-accounts | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


