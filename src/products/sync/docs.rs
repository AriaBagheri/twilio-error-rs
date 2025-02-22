// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioSyncError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioSyncError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioSyncError::ErrorCode54011 => r#"## Error - 54011

### Sync: Invalid TTL
The TTL value in seconds must either be empty or an integer between 0 and 31 536 000 (1 year).

#### Possible Causes
* The value supplied for the TTL parameter does not fall into the valid range.

#### Possible Solutions
* Confirm a valid TTL parameter is being passed in your request."#,
            TwilioSyncError::ErrorCode54006 => r#"
## Error - 54006

### Sync: Request entity too large
The size of the specified 'Data' parameter exceeds the size limit.

#### Possible Causes
* The size of a serialized JSON payload stored in a Sync Document, List Item, or Map Item cannot exceed 16KB; attempting to submit a larger object will yield this error.

#### Possible Solutions
* Reduce the size of the submitted JSON object by removing or compressing data.
* Split your data across multiple Map Items, List Items, or Documents.
"#,
            TwilioSyncError::ErrorCode54201 => r#"
## Error - 54201

### Sync: Map Item not found
The specified Map Item does not exist.

#### Possible Causes
* The specified Sync Map item key is incorrect.
* The requested item of a Sync Map has been deleted.

#### Possible Solutions
Confirm a valid Sync Map key is being passed in your request.
"#,
            TwilioSyncError::ErrorCode54155 => r#"
## Error - 54155

### Sync: List Item revision mismatch
The actual revision of the List Item does not match the expected revision.

#### Possible Causes
* Your update to this Sync List Item has conflicted with a request submitted elsewhere.
* The value specified in the If-Match header is erroneous.

#### Possible Solutions
If your update conflicts with a concurrent update elsewhere, the correct solution is to read the latest data (on the client SDK: wait for an update) and then re-attempt your update until you succeed. Consider that, since the data has changed, you may not want to submit the exact same update a second time. On the JavaScript SDK, we suggest using the [mutate](https://www.twilio.com/docs/api/sync/mutation-and-conflict-resolution#javascript-sdk-conflict-walkthrough) convenience function.

The value you provide in the If-Match header should be a revision assigned by the Sync service when interacting with the object.

* In SDKs, this is the `revision` field, part of the object descriptor.
* When using REST (e.g. via the Twilio Helper libraries) this is the `ETag` header provided in responses.
"#,
            TwilioSyncError::ErrorCode54451 => r#"
## Error - 54451

### Sync: Invalid 'Order' query parameter
The 'Order' query parameter must be one of {'asc', 'desc'}.

#### Possible Causes
* The value supplied by the Order query string parameter does not match _asc_ or _desc_ designators.

#### Possible Solutions
Confirm a valid Order query parameter is being passed in request.
"#,
            TwilioSyncError::ErrorCode54351 => r#"
## Error - 54351

### Sync: Invalid identity
Identity must be a string with length 1-256.

#### Possible Causes
* The value supplied by the Identity parameter is an empty string.
* The value supplied by the Identity parameter exceeds 256 characters.

#### Possible Solutions
Confirm than an Identity string parameter of 1-256 characters is being passed in request.
"#,
            TwilioSyncError::ErrorCode54156 => r#"
## Error - 54156

### Sync: Invalid List Item Data
The 'Data' parameter cannot be null.

#### Possible Causes
* Your request does not include a Data parameter which is required for this operation.

#### Possible Solutions
Confirm that a valid Data parameter containing a serialized, utf-8 JSON object is being passed in your request.
"#,
            TwilioSyncError::ErrorCode54450 => r#"
## Error - 54450

### Sync: Invalid 'Direction' query parameter
The 'Direction' query parameter must be one of {'forward', 'backward'}.

#### Possible Causes
* The value supplied by the Direction query string parameter does not match _forward_ or _backward_ designators.

#### Possible Solutions
Confirm that a valid Direction query parameter is being passed in request.
"#,
            TwilioSyncError::ErrorCode54251 => r#"
## Error - 54251

### Sync: Invalid Stream Message data
The 'Data' parameter cannot be null.

#### Possible Causes
* Your request does not include a Data parameter which is required for this operation.

#### Possible Solutions
Confirm that a valid Data parameter containing a serialized, utf-8 JSON object is being passed in your request.
"#,
            TwilioSyncError::ErrorCode54100 => r#"
## Error - 54100

### Sync: Document not found
The specified Document does not exist.

#### Possible Causes
* The supplied Sync Document SID or Unique Name is erroneous.
* This Sync Document has been deleted.

#### Possible Solutions
Confirm that the document you're requesting exists, and that you are using the precise SID or unique name it is assigned.
"#,
            TwilioSyncError::ErrorCode54003 => r#"
## Error - 54003

### Sync: Invalid If-Match header
The specified If-Match header cannot be used for concurrency control.

#### Possible Causes
* The value specified in the If-Match header is not a valid Sync object revision.

#### Possible Solutions
The value you provide should be a revision assigned by the Sync service when interacting with an object.

* In SDKs, this is the `revision` field, part of the object descriptor.
* When using REST (e.g. via the Twilio Helper libraries) this is the `ETag` header provided in responses.
"#,
            TwilioSyncError::ErrorCode54509 => r#"## Error - 54509

### Sync: Query expression contains too many operators
The query expression must contain less than 30 operators.

#### Possible Causes
* The query expression contains 30 or more operators, probably a user-facing edge case or a programming error.

#### Possible Solutions
* Rewrite the query or the code generating it to use fewer operators or issue separate queries. You can use any combination of the following operators: `and`, `or`, `in`, `eq`, `not_in`, `not_eq `, `contains`, however overall number of operators in any query expression must be less than 30."#,
            TwilioSyncError::ErrorCode54103 => r#"
## Error - 54103

### Sync: Document revision mismatch
The actual revision of the Document does not match the expected revision.

#### Possible Causes
* Your update to this Sync Document has conflicted with a request submitted elsewhere.
* The value specified in the If-Match header is erroneous.

#### Possible Solutions
If your update conflicts with a concurrent update elsewhere, the correct solution is to read the latest revision (on the client SDK: wait for an update) and then re-attempt your update until you succeed. Consider that, since the data has changed, you may not want to submit the exact same update a second time. On the JavaScript SDK, we suggest using the [mutate](https://www.twilio.com/docs/api/sync/mutation-and-conflict-resolution#javascript-sdk-conflict-walkthrough) convenience function.

The value you provide in the If-Match header should be a revision assigned by the Sync service when interacting with an object.

* In SDKs, this is the `revision` field, part of the object descriptor.
* When using REST (e.g. via the Twilio Helper libraries) this is the `ETag` header provided in responses.
"#,
            TwilioSyncError::ErrorCode54101 => r#"
## Error - 54101

### Sync: Invalid Document data
The 'Data' parameter cannot be null.

#### Possible Causes
Your request does not include a Data parameter which is required for this operation.

#### Possible Solutions
Confirm that a valid Data parameter containing a serialized, utf-8 JSON object is being passed in your request.
"#,
            TwilioSyncError::ErrorCode54507 => r#"## Error - 54507

### Sync: Invalid query
Valid query expression is expected.

#### Possible Causes
* You made a typo.
* You've made a programming error that produces a corrupt expression (where usually the expression is valid).

#### Possible Solutions
* Check that you're adhering to the right format. Query expressions must be specified in infix notation in the form `field operator "value"`.
* Check that you're using supported operators. Please review https://www.twilio.com/docs/sync/live-query for a list of supported operators and query examples examples."#,
            TwilioSyncError::ErrorCode54053 => r#"
## Error - 54053

### Sync: Invalid friendly name
The specified 'FriendlyName' parameter is not valid.

#### Possible Causes
* The value supplied by the FriendlyName parameter is an empty string.
* The value supplied by the FriendlyName parameter exceeds 255 characters.

#### Possible Solutions
Confirm the FriendlyName string parameter is either unspecified (`null`) or a nonempty string of up to 1-255 Unicode characters.
"#,
            TwilioSyncError::ErrorCode54452 => r#"
## Error - 54452

### Sync: Invalid 'Bounds' query parameter
The 'Bounds' query parameter must be one of {'inclusive', 'exclusive'}.

#### Possible Causes
* The value supplied by the Bounds query string parameter does not match _inclusive_ or _exclusive_ designators.

#### Possible Solutions
Confirm a valid Bounds query parameter is being passed in request.
"#,
            TwilioSyncError::ErrorCode54301 => r#"
## Error - 54301

### Sync: Unique name already exists
An object with the specified unique name already exists.

#### Possible Causes
* Another object with the matching Unique Name already exists in this Sync Service Instance, causing a conflict.
* The provided UniqueName parameter is not set correctly.
* Your original Sync object creation request has been sent more than once due to a retrier error.

#### Possible Solutions
* Confirm a valid UniqueName parameter is being passed in request, matching your expectation.
* Delete a previous object with the matching Unique Name if you no longer need it.
* Pick a different Unique Name, or create an anonymous object addressed by a SID instead.
"#,
            TwilioSyncError::ErrorCode54009 => r#"
## Error - 54009

### Sync: Rate limit exceeded
The rate of access to this resource exceeds the prescribed limits.

#### Possible Causes
* Your application is opening, reading or querying a single Sync object or its sub-items too often.
* Your application is updating a single Sync object or its sub-items too often.
* Your application is creating new Sync objects or their sub-items too often.

#### Possible Solutions
* Analyze your application to ensure that the _sustained_ rate of reads and writes against a single Sync object, and the sustained rate of of new object creations, is below 20 per second during normal application activity. Note that this limit only applies to explicit reads or writes; delivered state updates are not limited.
* Analyze your application to ensure that bursts of activity are uncommon; Sync provides a ten-second burst window during which you can exceed the sustained request-rate limits.
* Make sure your application (client or backend) uses a good exponential back-off algorithm like [the one advocated by Amazon](https://www.awsarchitectureblog.com/2015/03/backoff.html) to retry on HTTP 429 (rate limiting) responses.
"#,
            TwilioSyncError::ErrorCode54051 => r#"## ERROR - 54051

### Invalid webhook URL

The specified 'WebhookUrl' parameter is not a valid URL or exceeds the allowed length.  


#### Possible Causes
* You did not pass a correctly-formatted URL.
* You are crafting HTTP requests yourself and passed a URL, but did not percent-encoded it.
* You passed a webhook URL that is longer than 512 characters.

#### Possible Solutions
Confirm that you are providing a fully-qualified, percent-encoded HTTP URL no longer than 512 characters."#,
            TwilioSyncError::ErrorCode54354 => r#"## Error - 54354

### Sync: Permission not found
The specified Permission does not exist.

#### Possible Causes
* Requested permission does not exist because the provided Identity parameter is incorrect.
* Permission for the specified Identity has been deleted.

#### Possible Solutions
Confirm that the Identity parameter being passed in your request answers to the correct identity in your app, and that it has permissions to access the given object."#,
            TwilioSyncError::ErrorCode54502 => r#"## Error - 54502

### Sync: Invalid index name
The specified index doesn't exist, therefore cannot be queried.

#### Possible Causes
* You may have made a typo.
* You may have made a programming error that yields the empty string in place of a valid index name.

#### Possible Solutions
* Please review https://www.twilio.com/docs/sync/live-query for the currently supported list of indices, and make sure your code consumes one of those. There is no facility to create your own index today."#,
            TwilioSyncError::ErrorCode54453 => r#"## Error - 54453

### Sync: Invalid 'PageToken' query parameter
The specified 'PageToken' query parameter is invalid.

#### Possible Causes
* The value supplied by the PageToken query string parameter is invalid or malformed.

#### Possible Solutions
Ensure that a valid PageToken parameter is supplied, as originally included in the previous/next page links under the _meta_ block of pagination response."#,
            TwilioSyncError::ErrorCode54300 => r#"
## Error - 54300

### Sync: Unique name not found
No object with the specified unique name exists.

#### Possible Causes
* The provided UniqueName parameter is incorrect.
* The requested Sync object with given unique name has been deleted.

#### Possible Solutions
Confirm that a valid UniqueName parameter is being passed in your request.
"#,
            TwilioSyncError::ErrorCode54206 => r#"
## Error - 54206

### Sync: Invalid Map Item data
The 'Data' parameter cannot be null.

#### Possible Causes
* Your request does not include a Data parameter which is required for this operation.

#### Possible Solutions
Confirm that a valid Data parameter containing a serialized, utf-8 JSON object is being passed in your request.
"#,
            TwilioSyncError::ErrorCode54200 => r#"
## Error - 54200

### Sync: Map not found
The specified Map does not exist.

#### Possible Causes
* The supplied Sync Map SID is erroneous.
* The requested Sync Map has been deleted.

#### Possible Solutions
Confirm that the map you're requesting exists, and that you are using the precise SID or unique name it is assigned.
"#,
            TwilioSyncError::ErrorCode54302 => r#"
## Error - 54302

### Sync: Invalid unique name
The unique name must be a string with length 1-256 and not match the SID pattern [A-Z]{2}[a-f0-9]{32}.

#### Possible Causes
* The value supplied by the UniqueName parameter is an empty string.
* The value supplied by the UniqueName parameter exceeds 256 characters.

#### Possible Solutions
Confirm that your UniqueName is a string of 1-256 characters not matching the SID pattern.
"#,
            TwilioSyncError::ErrorCode54208 => r#"
## Error - 54208

### Sync: Map Item already exists
A Map Item with given key already exists in the Map.

#### Possible Causes
* Another Item with the matching Key already exists in this Sync Map, causing a conflict.
* The provided Key parameter is not set correctly.
* Your original Map Item creation request has been sent more than once due to a retrier error.

#### Possible Solutions
* Confirm a valid Key parameter is being passed in request, matching your expectation.
* Delete a previous Map Item with the matching Key if you no longer need it.
* Pick a different Key. If your items should be unique, consider using a UUID.
"#,
            TwilioSyncError::ErrorCode54510 => r#"## Error - 54510

### Sync: Query expression contains an array with too many items
Any array within query expression must contain less than 30 items.

#### Possible Causes
* The query expression contains an array with 30 or more elements.

#### Possible Solutions
* Rewrite your queries (or the code generating them) such that arrays never contain more than 30 items."#,
            TwilioSyncError::ErrorCode54050 => r#"
## Error - 54050

### Sync: Instance not found
The specified Service Instance does not exist.

#### Possible Causes
* The supplied Sync Service Instance SID is erroneous.
* The requested Sync Service Instance has been deleted.

#### Possible Solutions
* Use the [Twilio Console](https://twilio.com/console) or the [REST API](https://www.twilio.com/docs/api/sync/rest/services) to confirm your Sync service exists.
* Make sure the Sync Service Instance SID passed in your request matches the SID of that service exactly.
"#,
            TwilioSyncError::ErrorCode54151 => r#"
## Error - 54151

### Sync: List Item not found
The specified List Item does not exist.

#### Possible Causes
* The specified Sync List item index is incorrect.
* The requested item of a Sync List has been deleted.

#### Possible Solutions
Confirm a valid Sync List index is being passed in your request.
"#,
            TwilioSyncError::ErrorCode54150 => r#"
## Error - 54150

### Sync: List not found
The specified List does not exist.

#### Possible Causes
* The supplied Sync List SID is erroneous.
* The requested Sync List has been deleted.

#### Possible Solutions
Confirm that the list you're requesting exists, and that you are using the precise SID or unique name it is assigned.
"#,
            TwilioSyncError::ErrorCode54007 => r#"
## Error - 54007

### Sync: Access forbidden for identity
The identity specified in the Access Token does not have permissions to perform this operation.

#### Possible Causes
* Sync's ACL control has been unintentionally enabled on your Sync service instance.
* An SDK endpoint trying to read or write a Sync object does not have the correct permission configured.
* The identity you specified in your SDK's token does not match the permissions that you intended for them.

#### Possible Solutions
* Disable the ACL feature on this Sync service. This will allow access from any SDK with a valid token.
Do this by either unchecking the AclEnabled feature in the [Twilio Console](https://www.twilio.com/console/sync/services), or setting the `acl_enabled` flag to `false` via the [Sync REST API](https://www.twilio.com/docs/api/sync/rest/services).
* Confirm a valid permission is configured via the [Sync Permissions REST API](https://www.twilio.com/docs/api/sync/rest/sync-rest-api-permissions), and that the configured identity matches the identity you provide in your FPA Access Tokens.
"#,
            TwilioSyncError::ErrorCode54008 => r#"
## Error - 54008

### Sync: Invalid JSON
The specified 'Data' parameter is not valid JSON.

#### Possible Causes
* The data you submitted is not JSON, or
* A missing or poorly-encoded character makes your JSON fail to parse.
* You crafted the HTTP request yourself, and did not percent-encode the submitted JSON data in the form parameters.

#### Possible Solutions
* Use a tool like [JSONLint](https://jsonlint.com/) to check the data you're sending for syntax errors.
* If you're crafting HTTP requests yourself, make sure all form data is percent-encoded.
"#,
            TwilioSyncError::ErrorCode54209 => r#"
## Error - 54209

### Sync: Invalid Map Item key
The Map Item key must be a string with length 1-256.

#### Possible Causes
* The value supplied by the Map Key parameter contains an empty string.
* The string supplied by the Map Key parameter exceeds 256 characters.

#### Possible Solutions
* Confirm a valid Map Key parameter is being passed in your request.
"#,
            TwilioSyncError::ErrorCode54458 => r#"
## Error - 54458

### Sync: Invalid List Item Index
The List Item index must be a non-negative integer.

#### Possible Causes
* The value supplied by the List Index parameter contains a non-integer or a negative integer.

#### Possible Solutions
Confirm that a valid List Index parameter is being passed in your request.
"#,
            TwilioSyncError::ErrorCode54205 => r#"
## Error - 54205

### Sync: Map Item revision mismatch
The actual revision of the Map Item does not match the expected revision.

#### Possible Causes
* Your update to this Sync Map Item has conflicted with a request submitted elsewhere.
* The value specified in the If-Match header is erroneous.

#### Possible Solutions
If your update conflicts with a concurrent update elsewhere, the correct solution is to read the latest data (on the client SDK: wait for an update) and then re-attempt your update until you succeed. Consider that, since the data has changed, you may not want to submit the exact same update a second time. On the JavaScript SDK, we suggest using the [mutate](https://www.twilio.com/docs/api/sync/mutation-and-conflict-resolution#javascript-sdk-conflict-walkthrough) convenience function.

The value you provide in the If-Match header should be a revision assigned by the Sync service when interacting with the object.

* In SDKs, this is the `revision` field, part of the object descriptor.
* When using REST (e.g. via the Twilio Helper libraries) this is the `ETag` header provided in responses.
"#,
            TwilioSyncError::ErrorCode54056 => r#"
## Error - 54056

### Sync: Account cannot access requested Service Instance
Account cannot access requested Service Instance.

#### Possible Causes
* The requested Sync Service does not belong to this account.
* You have multiple accounts, and this Sync Service does not belong to the account whose credentials you used.

#### Possible Solutions
Confirm that you're using the correct credentials for this instance's account. This could mean:

* Make sure the Access Token specifies the correct Account SID
* Make sure that the Account SID + AuthToken pair you're using (for REST access) matches the correct account
* Make sure the API Key + API secret you're using (for REST access) belongs to the correct account.
"#,
            TwilioSyncError::ErrorCode54010 => r#"## Error - 54010

### Sync: No parameters specified
Expected at least 1 optional parameter, but none were provided.

#### Possible Causes
* The request expected at least 1 optional parameter, but none was provided.

#### Possible Solutions
* Add a parameter or confirm that the existing parameters are not misspelled."#,
            TwilioSyncError::ErrorCode54454 => r#"## ERROR - 54454

### Sync: Invalid 'PageSize' query parameter

The requested page size must be in the range [1-1000]. 


#### Possible Causes
The value supplied by the PageSize query string parameter is not an integer in the required range.

#### Possible Solutions
Confirm that a valid PageSize query parameter is being passed in request."#,
            TwilioSyncError::ErrorCode54419 => r#"## Error - 54419

### Sync: Number of subscriptions per connection is over the limit
An SDK attempted to open and listen for updates for more than 2000 objects.

#### Possible Causes
* A programming error has generated many queries in place of one.
* Your app has created too many LiveQueries over time (often a problem in single-page apps).

#### Possible Solutions
* Make sure you're only issuing queries for information that you need.
* When your UI no longer needs a query, use the `close()` method on the LiveQuery object to unsubscribe and release the corresponding Sync resources. See the [SDK Documentation](https://www.twilio.com/docs/sync/javascript-sdk-changelog) for details."#,
            TwilioSyncError::ErrorCode54250 => r#"
## Error - 54250

### Sync: Message Stream not found
The specified Message Stream does not exist.

#### Possible Causes
* The supplied Message Stream SID is erroneous.
* The requested Message Stream has been deleted.

#### Possible Solutions
Confirm a valid Message Stream SID is being passed in your request.
"#
        }
    }
}
