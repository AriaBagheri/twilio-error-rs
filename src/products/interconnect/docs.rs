// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioInterconnectError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioInterconnectError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioInterconnectError::ErrorCode62034 => r#"## Error - 62034 

#### Possible Causes 
*  No useful parameters provided

#### Possible Solutions
*  The request you made expected at least 1 optional parameter, but none was provided. Please add a parameter and/or confirm that is was not misspelled."#,
            TwilioInterconnectError::ErrorCode62015 => r#"## Error - 62015

### Connection in transition

The connection SID in the request is currently in a state of transition. A connection cannot be transitioned from inactive to active unless if the connection is currently in an inactive state. To transition the connection to an active state, please wait for the connection to be inactive again. "#,
            TwilioInterconnectError::ErrorCode32303 => r#"#### Possible Causes 
*   Multiple SIP outbound dials with TNX parameter


#### Possible Solutions
*  We don't support multiple SIP outbound dials with TNX parameter, please ensure you Response only includes one.
"#,
            TwilioInterconnectError::ErrorCode32301 => r#"#### Possible Causes 
*  The Interconnect Connection (TNX) SID is invalid.

#### Possible Solutions
*  Please verify your Interconnect Connection SID, visit [Interconnect Connections](https://www.twilio.com/console/sip-trunking/interconnect/connections)."#,
            TwilioInterconnectError::ErrorCode62028 => r#"## Error - 62028

#### Possible Causes 
* Connection expired.

#### Possible Solutions
*  The connection SID in the request has expired, and may no longer be used. Please create a new connection. "#,
            TwilioInterconnectError::ErrorCode62200 => r#"## ERROR - 62200

### Provisioning failure - Network-API is unavailable!

Network API is not available to perform the provisioning task Network provisioning calls are traced by network API which at the moment are not available.

#### Possible Causes
network connectivity, or the NSO server down

#### Possible Solutions
check the status of the NSO if the network is fine."#,
            TwilioInterconnectError::ErrorCode62001 => r#"## Error - 62001

### Invalid SID

A SID that was provided in the request was not valid. Ensure that any SIDs
are complete and valid."#,
            TwilioInterconnectError::ErrorCode62005 => r#"## Error - 62005

### Bandwidth reserve not found

The bandwidth reserve SID in the request could not be found.

"#,
            TwilioInterconnectError::ErrorCode62017 => r#"## Error - 62017

### No IP route specified

The request did not identify an IP route. This may indicate that the IP Route parameter was misspelled."#,
            TwilioInterconnectError::ErrorCode62006 => r#"## Error - 62006

### MPLS carrier is not associated with the exchange

The MPLS carrier SID in the request is not associated with the exchange SID
in the request. Choose an MPLS carrier SID that is associated with the
exchange SID, or choose an exchange SID that is associated with the MPLS
carrier SID."#,
            TwilioInterconnectError::ErrorCode62003 => r#"## Error - 62003

### MPLS carrier not found

The MPLS carrier SID in the request could not be found."#,
            TwilioInterconnectError::ErrorCode62025 => r#"## Error - 62025

#### Possible Causes 
* Invalid connection type

#### Possible Solutions
*  The request specified an invalid connection type. Please add a valid type-cloudconnect, crossconnect, mpls or vpn."#,
            TwilioInterconnectError::ErrorCode62012 => r#"## Error - 62012

### Connection not found

The connection SID in the request could not be found."#,
            TwilioInterconnectError::ErrorCode62053 => r#"#### Possible Causes 
*  The subaccount is not authorized to access this connection.
 
 
#### Possible Solutions
*  Please verify your Interconnect Connection permission settings, visit [Interconnect Connections](https://www.twilio.com/console/sip-trunking/interconnect/connections).
 
"#,
            TwilioInterconnectError::ErrorCode62020 => r#"## Error - 62020

### Connection pending deletion.

The connection SID in the request is pending deletion. No changes can be made to it. "#,
            TwilioInterconnectError::ErrorCode62021 => r#"## Error - 62021

### IP Gateway Invalid

The request provided an invalid gateway IP address. Request must provide a valid IP address not in the RFC 1918 range."#,
            TwilioInterconnectError::ErrorCode62009 => r#"## Error - 62009

### Account SID was not found

The account SID in the request could not be found in the Twilio Interconnect
directory. This does not mean that the account is not valid, only that it
is not associated with any Twilio Interconnect entries."#,
            TwilioInterconnectError::ErrorCode62024 => r#"## Error - 62024

### Missing connection type

The request did not specify a connection type. Valid connection types are vpn, crossconnect, mpls."#,
            TwilioInterconnectError::ErrorCode62027 => r#"## Error - 62027

#### Possible Causes 
* Extra MPLS parameter

#### Possible Solutions
*  An MPLS carrier does not relate to a connection of the specified type, please update the parameter and try again."#,
            TwilioInterconnectError::ErrorCode62008 => r#"## Error - 62008

### Bandwidth reserve is not associated with the exchange

The bandwidth reserve SID in the request is not associated with the
exchange SID in the request. Choose a bandwidth reserve that is associated
with the exchange SID, or choose an exchange SID that is associated with
the bandwidth reserve SID."#,
            TwilioInterconnectError::ErrorCode62100 => r#"## ERROR - 62100

### IP address(es) already linked to another connection

An IP address can be associated to one Interconnect connection only. The IP address is already associated with another Interconnect connection.

#### Possible Causes
IP conflict

#### Possible Solutions
New valid IP address"#,
            TwilioInterconnectError::ErrorCode62007 => r#"## Error - 62007

### Direct connect is not associated with the exchange

The direct connect SID in the request is not associated with the exchange
SID in the request. Choose a direct connect SID that is associated with the
exchange SID, or choose an exchange SID associated with the direct connect
SID."#,
            TwilioInterconnectError::ErrorCode62019 => r#"## Error - 62019

### IP route exceeds permitted IP range

The request provided an IP route that would exceed permitted IP range. IP addresses are limited to the 256 address range."#,
            TwilioInterconnectError::ErrorCode32304 => r#"#### Possible Causes 
*  The Interconnect Connection (TNX) SID is not Active.


#### Possible Solutions
*  Please verify the status of your Interconnect Connection SID, visit [Interconnect Connections](https://www.twilio.com/console/sip-trunking/interconnect/connections).


"#,
            TwilioInterconnectError::ErrorCode62014 => r#"## Error - 62014

### Connection not ready

The connection SID in the request is not fully configured.  Please provide additional configuration info.  For a VPN connection, please provide IP Gateway and at least one IP route. "#,
            TwilioInterconnectError::ErrorCode62010 => r#"## Error - 62010

### No authentication was provided.

An attempt was made to access a restricted resource without proper
authorization. Provide authentication details and try again."#,
            TwilioInterconnectError::ErrorCode62002 => r#"## Error - 62002

### Exchange not found

The exchange SID in the request could not be found."#,
            TwilioInterconnectError::ErrorCode62004 => r#"## Error - 62004

### Direct connect not found

The direct connect SID in the request could not be found.

"#,
            TwilioInterconnectError::ErrorCode62026 => r#"## Error - 62026

#### Possible Causes 
* Missing MPLS carrier SID

#### Possible Solutions
*  Please add a valid MPLS carrier SID."#,
            TwilioInterconnectError::ErrorCode62023 => r#"## Error - 62023

### Missing exchange

The request did not specify an exchange SID. "#,
            TwilioInterconnectError::ErrorCode62018 => r#"## Error - 62018

### Invalid IP route

The request provided an invalid IP route. The IP route must be a valid formed CIDR address that does not exist within the RFC 1918 address space."#,
            TwilioInterconnectError::ErrorCode62000 => r#"## Error - 62000

### Failed to write to the database.

An internal error occurred, and the service was unable to write data relevant
to your request."#,
            TwilioInterconnectError::ErrorCode62013 => r#"## Error - 62013

### Unable to identify account owner of connection

The connection SID in the request is orphaned, and the owner cannot be
identified."#,
            TwilioInterconnectError::ErrorCode62035 => r#"## Error - 62035

#### Possible Causes 
*  No bandwidth was specified in the request

#### Possible Solutions
* The request did not specify bandwidth. Please include an approved bandwidth option--those are 10.0Mb, 100.0mb, and 1000.0Mb"#,
            TwilioInterconnectError::ErrorCode62022 => r#"## Error - 62022

### Missing account SID

The request did not specify an account SID."#,
            TwilioInterconnectError::ErrorCode62016 => r#"## Error - 62016

### Connection not active

The connection SID in the request is not currently active. To transition a connection from active to inactive, the connection must be in an active state."#,
            TwilioInterconnectError::ErrorCode62052 => r#"#### Possible Causes 
*  This account is not a subaccount of the Interconnect Connection owner's account.
 
 
#### Possible Solutions
*  Please verify your Connection SID. Visit [Interconnect Connections](https://www.twilio.com/console/sip-trunking/interconnect/connections).
 
"#,
            TwilioInterconnectError::ErrorCode62011 => r#"## Error - 62011

### Unauthorized

The access credentials provided are not authorized to access this resource."#,
            TwilioInterconnectError::ErrorCode32302 => r#"#### Possible Causes 
*  The Interconnect Connection (TNX) SID was not found.


#### Possible Solutions
*  Please verify your Interconnect Connection SID, visit [Interconnect Connections](https://www.twilio.com/console/sip-trunking/interconnect/connections).
"#,
            TwilioInterconnectError::ErrorCode62220 => r#"## ERROR - 62220

### Provisioning failure - Requested bandwidth not available on the network device.

Network device does not have the capacity available to perform the provisioning task Bandwidth is not available on the network device due to lack of capacity

#### Possible Causes
Network device is out of capacity

#### Possible Solutions
Upgrade the capacity on the network device."#
        }
    }
}
