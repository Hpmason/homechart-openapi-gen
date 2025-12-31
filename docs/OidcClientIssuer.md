# OidcClientIssuer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auth_url** | Option<**String**> | OPTIONAL AuthURL is the endpoint for OAuth.  Can be discovered from OIDCIssuerURL. | [optional]
**auth_url_params** | Option<**std::collections::HashMap<String, String>**> | OPTIONAL AuthURLParams to send with OAuth. | [optional]
**ca_certificate_base64** | Option<**String**> | OPTIONAL CA certificate to use with requests. | [optional]
**client_id** | Option<**String**> | REQUIRED ClientID is the ID shared with clients for OAuth requests. | [optional]
**client_secret** | Option<**String**> | REQUIRED ClientSecret is used to retrieve tokens from OAuth responses. | [optional]
**client_secret_jwt_claims_custom** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | OPTIONAL Custom claims to be added to JWT. | [optional]
**client_secret_jwt_claims_reg** | Option<[**models::JwtRegisteredClaims**](jwt.RegisteredClaims.md)> | OPTIONAL Registered claims to be added to JWT. | [optional]
**client_secret_jwt_key** | Option<[**models::CryptolibKeyCryptolibKeyProviderPrivate**](cryptolib.Key-cryptolib_KeyProviderPrivate.md)> | OPTIONAL Key that can be used to generate a JWT for the clientSecret. | [optional]
**code_challenge_methods_supported** | Option<**Vec<String>**> | OPTIONAL CodeChallengeMethodsSupported is an optional list of supported.  Can be discovered from OIDCIssuerURL | [optional]
**display_name** | Option<**String**> | OPTIONAL Display name of the ClientIssuer. | [optional]
**filters** | Option<[**Vec<models::TypesFilter>**](types.Filter.md)> | A list of filters to test the returned token with. | [optional]
**icon** | Option<**String**> | OPTIONAL Icon of the ClientIssuer. | [optional]
**issuer_url** | Option<**String**> | OPTIONAL Issuer URL of the Issuer.  Must match aud for token.  Can be discovered from OIDCIssuerURL. | [optional]
**jwks_keys** | Option<[**Vec<models::CryptolibKeyCryptolibKeyProviderPublic>**](cryptolib.Key-cryptolib_KeyProviderPublic.md)> | OPTIONAL List of public keys for the Issuer.  Can be discovered from JWKSURI | [optional]
**jwks_uri** | Option<**String**> | OPTIONAL URL to retrieve JWKSKeys.  Can be discovered from OIDCIssuerURL | [optional]
**oidc_issuer_url** | Option<**String**> | OPTIONAL URL to retrieve issuer configurations. | [optional]
**scopes** | Option<**Vec<String>**> | OPTIONAL List of scopes to request.  Should include openid for OIDC issuers. | [optional]
**token_url** | Option<**String**> | OPTIONAL URL to retrieve token from.  Can be discovered from OIDCIssuerURL. | [optional]
**user_info_endpoint** | Option<**String**> | OPTIONAL URL to retrieve user info.  Can be discovered from OIDCIssuerURL. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


