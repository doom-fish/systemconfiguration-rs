# systemconfiguration-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 498
VERIFIED: 203
GAPS: 259
EXEMPT: 36
COVERAGE_PCT: 43.94%

This audit parsed the public `SystemConfiguration.framework` headers in `MacOSX26.2.sdk`, filtered out macOS-unavailable declarations, treated macOS-deprecated declarations as EXEMPT, and counted symbols as covered when they are reachable through the safe Swift bridge or the optional `raw-ffi` feature.

The gap count is dominated by long-tail schema constants: `Schema::catalog` currently covers 77/251 non-deprecated `SCSchemaDefinitions.h` constants, leaving 174 schema constants uncovered. Largest remaining uncovered families: SCSchemaDefinitions constants: 174; SCNetworkConnection family: 19; SCBond interface family: 18; SCVLAN interface family: 11; SCDynamicStore advanced helpers: 10.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| CNCopySupportedInterfaces | function | CaptiveNetwork.h | CaptiveNetwork; raw_ffi::CNCopySupportedInterfaces (feature="raw-ffi") |
| CNMarkPortalOffline | function | CaptiveNetwork.h | CaptiveNetwork; raw_ffi::CNMarkPortalOffline (feature="raw-ffi") |
| CNMarkPortalOnline | function | CaptiveNetwork.h | CaptiveNetwork; raw_ffi::CNMarkPortalOnline (feature="raw-ffi") |
| CNSetSupportedSSIDs | function | CaptiveNetwork.h | CaptiveNetwork; raw_ffi::CNSetSupportedSSIDs (feature="raw-ffi") |
| SCDynamicStoreAddTemporaryValue | function | SCDynamicStore.h | DynamicStore; raw_ffi::SCDynamicStoreAddTemporaryValue (feature="raw-ffi") |
| SCDynamicStoreAddValue | function | SCDynamicStore.h | DynamicStore; raw_ffi::SCDynamicStoreAddValue (feature="raw-ffi") |
| SCDynamicStoreCopyKeyList | function | SCDynamicStore.h | DynamicStore; raw_ffi::SCDynamicStoreCopyKeyList (feature="raw-ffi") |
| SCDynamicStoreCopyMultiple | function | SCDynamicStore.h | DynamicStore; raw_ffi::SCDynamicStoreCopyMultiple (feature="raw-ffi") |
| SCDynamicStoreCopyNotifiedKeys | function | SCDynamicStore.h | DynamicStore; raw_ffi::SCDynamicStoreCopyNotifiedKeys (feature="raw-ffi") |
| SCDynamicStoreCopyValue | function | SCDynamicStore.h | DynamicStore; raw_ffi::SCDynamicStoreCopyValue (feature="raw-ffi") |
| SCDynamicStoreCreate | function | SCDynamicStore.h | DynamicStore; raw_ffi::SCDynamicStoreCreate (feature="raw-ffi") |
| SCDynamicStoreCreateWithOptions | function | SCDynamicStore.h | DynamicStore |
| SCDynamicStoreNotifyValue | function | SCDynamicStore.h | DynamicStore; raw_ffi::SCDynamicStoreNotifyValue (feature="raw-ffi") |
| SCDynamicStoreRef | typedef | SCDynamicStore.h | raw_ffi::SCDynamicStoreRef (feature="raw-ffi") |
| SCDynamicStoreRemoveValue | function | SCDynamicStore.h | DynamicStore; raw_ffi::SCDynamicStoreRemoveValue (feature="raw-ffi") |
| SCDynamicStoreSetNotificationKeys | function | SCDynamicStore.h | DynamicStore; raw_ffi::SCDynamicStoreSetNotificationKeys (feature="raw-ffi") |
| SCDynamicStoreSetValue | function | SCDynamicStore.h | DynamicStore; raw_ffi::SCDynamicStoreSetValue (feature="raw-ffi") |
| kSCDynamicStoreUseSessionKeys | const | SCDynamicStore.h | DynamicStore |
| SCDynamicStoreCopyDHCPInfo | function | SCDynamicStoreCopyDHCPInfo.h | DynamicStore; raw_ffi::SCDynamicStoreCopyDHCPInfo (feature="raw-ffi") |
| SCDynamicStoreCopyComputerName | function | SCDynamicStoreCopySpecific.h | DynamicStore; raw_ffi::SCDynamicStoreCopyComputerName (feature="raw-ffi") |
| SCDynamicStoreCopyConsoleUser | function | SCDynamicStoreCopySpecific.h | ConsoleUser; raw_ffi::SCDynamicStoreCopyConsoleUser (feature="raw-ffi") |
| SCDynamicStoreCopyLocalHostName | function | SCDynamicStoreCopySpecific.h | DynamicStore; raw_ffi::SCDynamicStoreCopyLocalHostName (feature="raw-ffi") |
| SCDynamicStoreCopyLocation | function | SCDynamicStoreCopySpecific.h | DynamicStore; raw_ffi::SCDynamicStoreCopyLocation (feature="raw-ffi") |
| SCDynamicStoreCopyProxies | function | SCDynamicStoreCopySpecific.h | DynamicStore; raw_ffi::SCDynamicStoreCopyProxies (feature="raw-ffi") |
| SCDynamicStoreKeyCreateComputerName | function | SCDynamicStoreKey.h | DynamicStore; raw_ffi::SCDynamicStoreKeyCreateComputerName (feature="raw-ffi") |
| SCDynamicStoreKeyCreateConsoleUser | function | SCDynamicStoreKey.h | DynamicStore; raw_ffi::SCDynamicStoreKeyCreateConsoleUser (feature="raw-ffi") |
| SCDynamicStoreKeyCreateHostNames | function | SCDynamicStoreKey.h | DynamicStore; raw_ffi::SCDynamicStoreKeyCreateHostNames (feature="raw-ffi") |
| SCDynamicStoreKeyCreateLocation | function | SCDynamicStoreKey.h | DynamicStore; raw_ffi::SCDynamicStoreKeyCreateLocation (feature="raw-ffi") |
| SCDynamicStoreKeyCreateNetworkGlobalEntity | function | SCDynamicStoreKey.h | DynamicStore; raw_ffi::SCDynamicStoreKeyCreateNetworkGlobalEntity (feature="raw-ffi") |
| SCDynamicStoreKeyCreateNetworkInterface | function | SCDynamicStoreKey.h | DynamicStore; raw_ffi::SCDynamicStoreKeyCreateNetworkInterface (feature="raw-ffi") |
| SCDynamicStoreKeyCreateNetworkInterfaceEntity | function | SCDynamicStoreKey.h | DynamicStore; raw_ffi::SCDynamicStoreKeyCreateNetworkInterfaceEntity (feature="raw-ffi") |
| SCDynamicStoreKeyCreateNetworkServiceEntity | function | SCDynamicStoreKey.h | DynamicStore; raw_ffi::SCDynamicStoreKeyCreateNetworkServiceEntity (feature="raw-ffi") |
| SCDynamicStoreKeyCreateProxies | function | SCDynamicStoreKey.h | DynamicStore; raw_ffi::SCDynamicStoreKeyCreateProxies (feature="raw-ffi") |
| SCNetworkInterfaceCopyAll | function | SCNetworkConfiguration.h | NetworkConfiguration::overview; NetworkInterface; raw_ffi::SCNetworkInterfaceCopyAll (feature="raw-ffi") |
| SCNetworkInterfaceCopyMTU | function | SCNetworkConfiguration.h | NetworkInterface; raw_ffi::SCNetworkInterfaceCopyMTU (feature="raw-ffi") |
| SCNetworkInterfaceCreateWithInterface | function | SCNetworkConfiguration.h | NetworkInterface; raw_ffi::SCNetworkInterfaceCreateWithInterface (feature="raw-ffi") |
| SCNetworkInterfaceForceConfigurationRefresh | function | SCNetworkConfiguration.h | NetworkInterface; raw_ffi::SCNetworkInterfaceForceConfigurationRefresh (feature="raw-ffi") |
| SCNetworkInterfaceGetBSDName | function | SCNetworkConfiguration.h | NetworkInterface; raw_ffi::SCNetworkInterfaceGetBSDName (feature="raw-ffi") |
| SCNetworkInterfaceGetConfiguration | function | SCNetworkConfiguration.h | NetworkInterface; raw_ffi::SCNetworkInterfaceGetConfiguration (feature="raw-ffi") |
| SCNetworkInterfaceGetExtendedConfiguration | function | SCNetworkConfiguration.h | NetworkInterface; raw_ffi::SCNetworkInterfaceGetExtendedConfiguration (feature="raw-ffi") |
| SCNetworkInterfaceGetHardwareAddressString | function | SCNetworkConfiguration.h | NetworkInterface; raw_ffi::SCNetworkInterfaceGetHardwareAddressString (feature="raw-ffi") |
| SCNetworkInterfaceGetInterface | function | SCNetworkConfiguration.h | NetworkInterface; raw_ffi::SCNetworkInterfaceGetInterface (feature="raw-ffi") |
| SCNetworkInterfaceGetInterfaceType | function | SCNetworkConfiguration.h | NetworkInterface; raw_ffi::SCNetworkInterfaceGetInterfaceType (feature="raw-ffi") |
| SCNetworkInterfaceGetLocalizedDisplayName | function | SCNetworkConfiguration.h | NetworkInterface; raw_ffi::SCNetworkInterfaceGetLocalizedDisplayName (feature="raw-ffi") |
| SCNetworkInterfaceGetSupportedInterfaceTypes | function | SCNetworkConfiguration.h | NetworkInterface; raw_ffi::SCNetworkInterfaceGetSupportedInterfaceTypes (feature="raw-ffi") |
| SCNetworkInterfaceGetSupportedProtocolTypes | function | SCNetworkConfiguration.h | NetworkInterface; raw_ffi::SCNetworkInterfaceGetSupportedProtocolTypes (feature="raw-ffi") |
| SCNetworkInterfaceRef | typedef | SCNetworkConfiguration.h | raw_ffi::SCNetworkInterfaceRef (feature="raw-ffi") |
| SCNetworkProtocolGetConfiguration | function | SCNetworkConfiguration.h | NetworkProtocol; raw_ffi::SCNetworkProtocolGetConfiguration (feature="raw-ffi") |
| SCNetworkProtocolGetEnabled | function | SCNetworkConfiguration.h | NetworkProtocol; raw_ffi::SCNetworkProtocolGetEnabled (feature="raw-ffi") |
| SCNetworkProtocolGetProtocolType | function | SCNetworkConfiguration.h | NetworkProtocol; raw_ffi::SCNetworkProtocolGetProtocolType (feature="raw-ffi") |
| SCNetworkProtocolRef | typedef | SCNetworkConfiguration.h | raw_ffi::SCNetworkProtocolRef (feature="raw-ffi") |
| SCNetworkProtocolSetConfiguration | function | SCNetworkConfiguration.h | NetworkProtocol; raw_ffi::SCNetworkProtocolSetConfiguration (feature="raw-ffi") |
| SCNetworkProtocolSetEnabled | function | SCNetworkConfiguration.h | NetworkProtocol; raw_ffi::SCNetworkProtocolSetEnabled (feature="raw-ffi") |
| SCNetworkServiceAddProtocolType | function | SCNetworkConfiguration.h | NetworkService; raw_ffi::SCNetworkServiceAddProtocolType (feature="raw-ffi") |
| SCNetworkServiceCopyAll | function | SCNetworkConfiguration.h | NetworkConfiguration::overview; NetworkService; raw_ffi::SCNetworkServiceCopyAll (feature="raw-ffi") |
| SCNetworkServiceCopyProtocol | function | SCNetworkConfiguration.h | NetworkService; raw_ffi::SCNetworkServiceCopyProtocol (feature="raw-ffi") |
| SCNetworkServiceCopyProtocols | function | SCNetworkConfiguration.h | NetworkService; raw_ffi::SCNetworkServiceCopyProtocols (feature="raw-ffi") |
| SCNetworkServiceEstablishDefaultConfiguration | function | SCNetworkConfiguration.h | NetworkService; raw_ffi::SCNetworkServiceEstablishDefaultConfiguration (feature="raw-ffi") |
| SCNetworkServiceGetEnabled | function | SCNetworkConfiguration.h | NetworkService; raw_ffi::SCNetworkServiceGetEnabled (feature="raw-ffi") |
| SCNetworkServiceGetInterface | function | SCNetworkConfiguration.h | NetworkService; raw_ffi::SCNetworkServiceGetInterface (feature="raw-ffi") |
| SCNetworkServiceGetName | function | SCNetworkConfiguration.h | NetworkService; raw_ffi::SCNetworkServiceGetName (feature="raw-ffi") |
| SCNetworkServiceGetServiceID | function | SCNetworkConfiguration.h | NetworkService; raw_ffi::SCNetworkServiceGetServiceID (feature="raw-ffi") |
| SCNetworkServiceRef | typedef | SCNetworkConfiguration.h | raw_ffi::SCNetworkServiceRef (feature="raw-ffi") |
| SCNetworkServiceRemove | function | SCNetworkConfiguration.h | NetworkService; raw_ffi::SCNetworkServiceRemove (feature="raw-ffi") |
| SCNetworkServiceRemoveProtocolType | function | SCNetworkConfiguration.h | NetworkService; raw_ffi::SCNetworkServiceRemoveProtocolType (feature="raw-ffi") |
| SCNetworkServiceSetEnabled | function | SCNetworkConfiguration.h | NetworkService; raw_ffi::SCNetworkServiceSetEnabled (feature="raw-ffi") |
| SCNetworkServiceSetName | function | SCNetworkConfiguration.h | NetworkService; raw_ffi::SCNetworkServiceSetName (feature="raw-ffi") |
| SCNetworkSetAddService | function | SCNetworkConfiguration.h | NetworkSet; raw_ffi::SCNetworkSetAddService (feature="raw-ffi") |
| SCNetworkSetContainsInterface | function | SCNetworkConfiguration.h | NetworkSet; raw_ffi::SCNetworkSetContainsInterface (feature="raw-ffi") |
| SCNetworkSetCopyAll | function | SCNetworkConfiguration.h | NetworkConfiguration::overview; NetworkSet; raw_ffi::SCNetworkSetCopyAll (feature="raw-ffi") |
| SCNetworkSetCopyCurrent | function | SCNetworkConfiguration.h | NetworkConfiguration::overview; NetworkSet; raw_ffi::SCNetworkSetCopyCurrent (feature="raw-ffi") |
| SCNetworkSetCopyServices | function | SCNetworkConfiguration.h | NetworkSet; raw_ffi::SCNetworkSetCopyServices (feature="raw-ffi") |
| SCNetworkSetGetName | function | SCNetworkConfiguration.h | NetworkConfiguration::overview; NetworkSet; raw_ffi::SCNetworkSetGetName (feature="raw-ffi") |
| SCNetworkSetGetServiceOrder | function | SCNetworkConfiguration.h | NetworkSet; raw_ffi::SCNetworkSetGetServiceOrder (feature="raw-ffi") |
| SCNetworkSetGetSetID | function | SCNetworkConfiguration.h | NetworkConfiguration::overview; NetworkSet; raw_ffi::SCNetworkSetGetSetID (feature="raw-ffi") |
| SCNetworkSetRef | typedef | SCNetworkConfiguration.h | raw_ffi::SCNetworkSetRef (feature="raw-ffi") |
| SCNetworkSetRemove | function | SCNetworkConfiguration.h | NetworkSet; raw_ffi::SCNetworkSetRemove (feature="raw-ffi") |
| SCNetworkSetRemoveService | function | SCNetworkConfiguration.h | NetworkSet; raw_ffi::SCNetworkSetRemoveService (feature="raw-ffi") |
| SCNetworkSetSetCurrent | function | SCNetworkConfiguration.h | NetworkSet; raw_ffi::SCNetworkSetSetCurrent (feature="raw-ffi") |
| SCNetworkSetSetName | function | SCNetworkConfiguration.h | NetworkSet; raw_ffi::SCNetworkSetSetName (feature="raw-ffi") |
| SCNetworkSetSetServiceOrder | function | SCNetworkConfiguration.h | NetworkSet; raw_ffi::SCNetworkSetSetServiceOrder (feature="raw-ffi") |
| kSCNetworkInterfaceType6to4 | const | SCNetworkConfiguration.h | NetworkConfiguration::overview |
| kSCNetworkInterfaceTypeBluetooth | const | SCNetworkConfiguration.h | NetworkConfiguration::overview |
| kSCNetworkInterfaceTypeBond | const | SCNetworkConfiguration.h | NetworkConfiguration::overview |
| kSCNetworkInterfaceTypeEthernet | const | SCNetworkConfiguration.h | NetworkConfiguration::overview |
| kSCNetworkInterfaceTypeFireWire | const | SCNetworkConfiguration.h | NetworkConfiguration::overview |
| kSCNetworkInterfaceTypeIEEE80211 | const | SCNetworkConfiguration.h | NetworkConfiguration::overview |
| kSCNetworkInterfaceTypeIPSec | const | SCNetworkConfiguration.h | NetworkConfiguration::overview |
| kSCNetworkInterfaceTypeIPv4 | const | SCNetworkConfiguration.h | NetworkConfiguration::overview |
| kSCNetworkInterfaceTypeL2TP | const | SCNetworkConfiguration.h | NetworkConfiguration::overview |
| kSCNetworkInterfaceTypeModem | const | SCNetworkConfiguration.h | NetworkConfiguration::overview |
| kSCNetworkInterfaceTypePPP | const | SCNetworkConfiguration.h | NetworkConfiguration::overview |
| kSCNetworkInterfaceTypeSerial | const | SCNetworkConfiguration.h | NetworkConfiguration::overview |
| kSCNetworkInterfaceTypeVLAN | const | SCNetworkConfiguration.h | NetworkConfiguration::overview |
| kSCNetworkInterfaceTypeWWAN | const | SCNetworkConfiguration.h | NetworkConfiguration::overview |
| kSCNetworkProtocolTypeDNS | const | SCNetworkConfiguration.h | NetworkConfiguration::overview |
| kSCNetworkProtocolTypeIPv4 | const | SCNetworkConfiguration.h | NetworkConfiguration::overview |
| kSCNetworkProtocolTypeIPv6 | const | SCNetworkConfiguration.h | NetworkConfiguration::overview |
| kSCNetworkProtocolTypeProxies | const | SCNetworkConfiguration.h | NetworkConfiguration::overview |
| kSCNetworkProtocolTypeSMB | const | SCNetworkConfiguration.h | NetworkConfiguration::overview |
| SCNetworkReachabilityCallBack | typedef | SCNetworkReachability.h | raw_ffi::SCNetworkReachabilityCallBack (feature="raw-ffi") |
| SCNetworkReachabilityContext | typedef | SCNetworkReachability.h | Reachability / NetworkReachability; raw_ffi::SCNetworkReachabilityContext (feature="raw-ffi") |
| SCNetworkReachabilityRef | typedef | SCNetworkReachability.h | raw_ffi::SCNetworkReachabilityRef (feature="raw-ffi") |
| SCPreferencesAddValue | function | SCPreferences.h | Preferences; raw_ffi::SCPreferencesAddValue (feature="raw-ffi") |
| SCPreferencesApplyChanges | function | SCPreferences.h | Preferences; raw_ffi::SCPreferencesApplyChanges (feature="raw-ffi") |
| SCPreferencesCommitChanges | function | SCPreferences.h | Preferences; raw_ffi::SCPreferencesCommitChanges (feature="raw-ffi") |
| SCPreferencesCopyKeyList | function | SCPreferences.h | Preferences; raw_ffi::SCPreferencesCopyKeyList (feature="raw-ffi") |
| SCPreferencesCreate | function | SCPreferences.h | NetworkConfiguration::overview; Preferences; raw_ffi::SCPreferencesCreate (feature="raw-ffi") |
| SCPreferencesGetSignature | function | SCPreferences.h | Preferences; raw_ffi::SCPreferencesGetSignature (feature="raw-ffi") |
| SCPreferencesGetValue | function | SCPreferences.h | Preferences; raw_ffi::SCPreferencesGetValue (feature="raw-ffi") |
| SCPreferencesLock | function | SCPreferences.h | Preferences; raw_ffi::SCPreferencesLock (feature="raw-ffi") |
| SCPreferencesRef | typedef | SCPreferences.h | raw_ffi::SCPreferencesRef (feature="raw-ffi") |
| SCPreferencesRemoveValue | function | SCPreferences.h | Preferences; raw_ffi::SCPreferencesRemoveValue (feature="raw-ffi") |
| SCPreferencesSetValue | function | SCPreferences.h | Preferences; raw_ffi::SCPreferencesSetValue (feature="raw-ffi") |
| SCPreferencesSynchronize | function | SCPreferences.h | Preferences; raw_ffi::SCPreferencesSynchronize (feature="raw-ffi") |
| SCPreferencesUnlock | function | SCPreferences.h | Preferences; raw_ffi::SCPreferencesUnlock (feature="raw-ffi") |
| SCPreferencesPathCreateUniqueChild | function | SCPreferencesPath.h | Preferences; raw_ffi::SCPreferencesPathCreateUniqueChild (feature="raw-ffi") |
| SCPreferencesPathGetLink | function | SCPreferencesPath.h | Preferences; raw_ffi::SCPreferencesPathGetLink (feature="raw-ffi") |
| SCPreferencesPathGetValue | function | SCPreferencesPath.h | Preferences; raw_ffi::SCPreferencesPathGetValue (feature="raw-ffi") |
| SCPreferencesPathRemoveValue | function | SCPreferencesPath.h | Preferences; raw_ffi::SCPreferencesPathRemoveValue (feature="raw-ffi") |
| SCPreferencesPathSetLink | function | SCPreferencesPath.h | Preferences; raw_ffi::SCPreferencesPathSetLink (feature="raw-ffi") |
| SCPreferencesPathSetValue | function | SCPreferencesPath.h | Preferences; raw_ffi::SCPreferencesPathSetValue (feature="raw-ffi") |
| SCPreferencesSetComputerName | function | SCPreferencesSetSpecific.h | Preferences; raw_ffi::SCPreferencesSetComputerName (feature="raw-ffi") |
| SCPreferencesSetLocalHostName | function | SCPreferencesSetSpecific.h | Preferences; raw_ffi::SCPreferencesSetLocalHostName (feature="raw-ffi") |
| kSCCompAnyRegex | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCCompGlobal | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCCompHostNames | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCCompInterface | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCCompNetwork | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCCompService | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCCompSystem | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCCompUsers | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCEntNet6to4 | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCEntNetAirPort | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCEntNetDHCP | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCEntNetDNS | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCEntNetEthernet | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCEntNetIPSec | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCEntNetIPv4 | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCEntNetIPv6 | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCEntNetInterface | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCEntNetL2TP | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCEntNetLink | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCEntNetModem | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCEntNetPPP | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCEntNetPPPoE | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCEntNetProxies | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCEntNetSMB | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPrefCurrentSet | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPrefNetworkServices | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPrefSets | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPrefSystem | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPropInterfaceName | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPropMACAddress | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPropNetDNSDomainName | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPropNetDNSOptions | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPropNetDNSSearchDomains | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPropNetDNSServerAddresses | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPropNetDNSServerPort | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPropNetIPv4Addresses | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPropNetIPv4ConfigMethod | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPropNetIPv4DHCPClientID | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPropNetIPv4Router | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPropNetIPv4SubnetMasks | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPropNetIPv6Addresses | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPropNetIPv6ConfigMethod | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPropNetIPv6Router | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPropNetProxiesExceptionsList | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPropNetProxiesHTTPEnable | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPropNetProxiesHTTPPort | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPropNetProxiesHTTPProxy | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPropNetProxiesHTTPSEnable | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPropNetProxiesHTTPSPort | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPropNetProxiesHTTPSProxy | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPropNetProxiesSOCKSEnable | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPropNetProxiesSOCKSPort | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPropNetProxiesSOCKSProxy | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPropUserDefinedName | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCPropVersion | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCResvInactive | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCResvLink | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCValNetIPv4ConfigMethodAutomatic | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCValNetIPv4ConfigMethodBOOTP | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCValNetIPv4ConfigMethodDHCP | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCValNetIPv4ConfigMethodINFORM | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCValNetIPv4ConfigMethodLinkLocal | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCValNetIPv4ConfigMethodManual | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCValNetIPv4ConfigMethodPPP | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCValNetIPv6ConfigMethod6to4 | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCValNetIPv6ConfigMethodAutomatic | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCValNetIPv6ConfigMethodLinkLocal | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCValNetIPv6ConfigMethodManual | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCValNetIPv6ConfigMethodRouterAdvertisement | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCValNetInterfaceSubTypeL2TP | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCValNetInterfaceSubTypePPPSerial | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCValNetInterfaceSubTypePPPoE | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCValNetInterfaceType6to4 | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCValNetInterfaceTypeEthernet | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCValNetInterfaceTypeFireWire | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCValNetInterfaceTypeIPSec | const | SCSchemaDefinitions.h | Schema::catalog |
| kSCValNetInterfaceTypePPP | const | SCSchemaDefinitions.h | Schema::catalog |
| SCError | function | SystemConfiguration.h | SystemConfigurationError; raw_ffi::SCError (feature="raw-ffi") |
| SCErrorString | function | SystemConfiguration.h | SystemConfigurationError; raw_ffi::SCErrorString (feature="raw-ffi") |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| SCDynamicStoreCallBack | typedef | SCDynamicStore.h | Callback/run-loop/TypeID helpers are not wrapped. |
| SCDynamicStoreContext | typedef | SCDynamicStore.h | Callback/run-loop/TypeID helpers are not wrapped. |
| SCDynamicStoreCreateRunLoopSource | function | SCDynamicStore.h | Callback/run-loop/TypeID helpers are not wrapped. |
| SCDynamicStoreGetTypeID | function | SCDynamicStore.h | Callback/run-loop/TypeID helpers are not wrapped. |
| SCDynamicStoreSetDispatchQueue | function | SCDynamicStore.h | Callback/run-loop/TypeID helpers are not wrapped. |
| SCDynamicStoreSetMultiple | function | SCDynamicStore.h | Callback/run-loop/TypeID helpers are not wrapped. |
| DHCPInfoGetLeaseExpirationTime | function | SCDynamicStoreCopyDHCPInfo.h | Only SCDynamicStoreCopyDHCPInfo is wrapped; the DHCPInfo accessor helpers are not. |
| DHCPInfoGetLeaseStartTime | function | SCDynamicStoreCopyDHCPInfo.h | Only SCDynamicStoreCopyDHCPInfo is wrapped; the DHCPInfo accessor helpers are not. |
| DHCPInfoGetOptionData | function | SCDynamicStoreCopyDHCPInfo.h | Only SCDynamicStoreCopyDHCPInfo is wrapped; the DHCPInfo accessor helpers are not. |
| SCDynamicStoreKeyCreate | function | SCDynamicStoreKey.h | Callback/run-loop/TypeID helpers are not wrapped. |
| SCNetworkConnectionFlags | typedef | SCNetwork.h | The SCNetworkConnection session/control family is entirely absent. |
| SCBondInterfaceCopyAll | function | SCNetworkConfiguration.h | The bond-interface family is not wrapped. |
| SCBondInterfaceCopyAvailableMemberInterfaces | function | SCNetworkConfiguration.h | The bond-interface family is not wrapped. |
| SCBondInterfaceCopyStatus | function | SCNetworkConfiguration.h | The bond-interface family is not wrapped. |
| SCBondInterfaceCreate | function | SCNetworkConfiguration.h | The bond-interface family is not wrapped. |
| SCBondInterfaceGetMemberInterfaces | function | SCNetworkConfiguration.h | The bond-interface family is not wrapped. |
| SCBondInterfaceGetOptions | function | SCNetworkConfiguration.h | The bond-interface family is not wrapped. |
| SCBondInterfaceRef | typedef | SCNetworkConfiguration.h | The bond-interface family is not wrapped. |
| SCBondInterfaceRemove | function | SCNetworkConfiguration.h | The bond-interface family is not wrapped. |
| SCBondInterfaceSetLocalizedDisplayName | function | SCNetworkConfiguration.h | The bond-interface family is not wrapped. |
| SCBondInterfaceSetMemberInterfaces | function | SCNetworkConfiguration.h | The bond-interface family is not wrapped. |
| SCBondInterfaceSetOptions | function | SCNetworkConfiguration.h | The bond-interface family is not wrapped. |
| SCBondStatusGetInterfaceStatus | function | SCNetworkConfiguration.h | The bond-interface family is not wrapped. |
| SCBondStatusGetMemberInterfaces | function | SCNetworkConfiguration.h | The bond-interface family is not wrapped. |
| SCBondStatusGetTypeID | function | SCNetworkConfiguration.h | The bond-interface family is not wrapped. |
| SCBondStatusRef | typedef | SCNetworkConfiguration.h | The bond-interface family is not wrapped. |
| SCNetworkInterfaceCopyMediaOptions | function | SCNetworkConfiguration.h | Advanced interface configuration helpers are not wrapped. |
| SCNetworkInterfaceCopyMediaSubTypeOptions | function | SCNetworkConfiguration.h | Advanced interface configuration helpers are not wrapped. |
| SCNetworkInterfaceCopyMediaSubTypes | function | SCNetworkConfiguration.h | Advanced interface configuration helpers are not wrapped. |
| SCNetworkInterfaceGetTypeID | function | SCNetworkConfiguration.h | Advanced interface configuration helpers are not wrapped. |
| SCNetworkInterfaceSetConfiguration | function | SCNetworkConfiguration.h | Advanced interface configuration helpers are not wrapped. |
| SCNetworkInterfaceSetExtendedConfiguration | function | SCNetworkConfiguration.h | Advanced interface configuration helpers are not wrapped. |
| SCNetworkInterfaceSetMTU | function | SCNetworkConfiguration.h | Advanced interface configuration helpers are not wrapped. |
| SCNetworkInterfaceSetMediaOptions | function | SCNetworkConfiguration.h | Advanced interface configuration helpers are not wrapped. |
| SCNetworkProtocolGetTypeID | function | SCNetworkConfiguration.h | The protocol type-ID helper is not wrapped. |
| SCNetworkServiceCopy | function | SCNetworkConfiguration.h | Service creation/copy/type-ID helpers are not wrapped. |
| SCNetworkServiceCreate | function | SCNetworkConfiguration.h | Service creation/copy/type-ID helpers are not wrapped. |
| SCNetworkServiceGetTypeID | function | SCNetworkConfiguration.h | Service creation/copy/type-ID helpers are not wrapped. |
| SCNetworkSetCopy | function | SCNetworkConfiguration.h | Set creation/copy/type-ID helpers are not wrapped. |
| SCNetworkSetCreate | function | SCNetworkConfiguration.h | Set creation/copy/type-ID helpers are not wrapped. |
| SCNetworkSetGetTypeID | function | SCNetworkConfiguration.h | Set creation/copy/type-ID helpers are not wrapped. |
| SCVLANInterfaceCopyAll | function | SCNetworkConfiguration.h | The VLAN-interface family is not wrapped. |
| SCVLANInterfaceCopyAvailablePhysicalInterfaces | function | SCNetworkConfiguration.h | The VLAN-interface family is not wrapped. |
| SCVLANInterfaceCreate | function | SCNetworkConfiguration.h | The VLAN-interface family is not wrapped. |
| SCVLANInterfaceGetOptions | function | SCNetworkConfiguration.h | The VLAN-interface family is not wrapped. |
| SCVLANInterfaceGetPhysicalInterface | function | SCNetworkConfiguration.h | The VLAN-interface family is not wrapped. |
| SCVLANInterfaceGetTag | function | SCNetworkConfiguration.h | The VLAN-interface family is not wrapped. |
| SCVLANInterfaceRef | typedef | SCNetworkConfiguration.h | The VLAN-interface family is not wrapped. |
| SCVLANInterfaceRemove | function | SCNetworkConfiguration.h | The VLAN-interface family is not wrapped. |
| SCVLANInterfaceSetLocalizedDisplayName | function | SCNetworkConfiguration.h | The VLAN-interface family is not wrapped. |
| SCVLANInterfaceSetOptions | function | SCNetworkConfiguration.h | The VLAN-interface family is not wrapped. |
| SCVLANInterfaceSetPhysicalInterfaceAndTag | function | SCNetworkConfiguration.h | The VLAN-interface family is not wrapped. |
| kSCBondStatusDeviceAggregationStatus | const | SCNetworkConfiguration.h | The bond-interface family is not wrapped. |
| kSCBondStatusDeviceCollecting | const | SCNetworkConfiguration.h | The bond-interface family is not wrapped. |
| kSCBondStatusDeviceDistributing | const | SCNetworkConfiguration.h | The bond-interface family is not wrapped. |
| kSCNetworkInterfaceIPv4 | const | SCNetworkConfiguration.h | This network-interface constant is not surfaced. |
| SCNetworkConnectionCallBack | typedef | SCNetworkConnection.h | The SCNetworkConnection session/control family is entirely absent. |
| SCNetworkConnectionContext | typedef | SCNetworkConnection.h | The SCNetworkConnection session/control family is entirely absent. |
| SCNetworkConnectionCopyExtendedStatus | function | SCNetworkConnection.h | The SCNetworkConnection session/control family is entirely absent. |
| SCNetworkConnectionCopyServiceID | function | SCNetworkConnection.h | The SCNetworkConnection session/control family is entirely absent. |
| SCNetworkConnectionCopyStatistics | function | SCNetworkConnection.h | The SCNetworkConnection session/control family is entirely absent. |
| SCNetworkConnectionCopyUserOptions | function | SCNetworkConnection.h | The SCNetworkConnection session/control family is entirely absent. |
| SCNetworkConnectionCopyUserPreferences | function | SCNetworkConnection.h | The SCNetworkConnection session/control family is entirely absent. |
| SCNetworkConnectionCreateWithServiceID | function | SCNetworkConnection.h | The SCNetworkConnection session/control family is entirely absent. |
| SCNetworkConnectionGetStatus | function | SCNetworkConnection.h | The SCNetworkConnection session/control family is entirely absent. |
| SCNetworkConnectionGetTypeID | function | SCNetworkConnection.h | The SCNetworkConnection session/control family is entirely absent. |
| SCNetworkConnectionPPPStatus | typedef | SCNetworkConnection.h | The SCNetworkConnection session/control family is entirely absent. |
| SCNetworkConnectionRef | typedef | SCNetworkConnection.h | The SCNetworkConnection session/control family is entirely absent. |
| SCNetworkConnectionScheduleWithRunLoop | function | SCNetworkConnection.h | The SCNetworkConnection session/control family is entirely absent. |
| SCNetworkConnectionSetDispatchQueue | function | SCNetworkConnection.h | The SCNetworkConnection session/control family is entirely absent. |
| SCNetworkConnectionStart | function | SCNetworkConnection.h | The SCNetworkConnection session/control family is entirely absent. |
| SCNetworkConnectionStatus | typedef | SCNetworkConnection.h | The SCNetworkConnection session/control family is entirely absent. |
| SCNetworkConnectionStop | function | SCNetworkConnection.h | The SCNetworkConnection session/control family is entirely absent. |
| SCNetworkConnectionUnscheduleFromRunLoop | function | SCNetworkConnection.h | The SCNetworkConnection session/control family is entirely absent. |
| SCPreferencesCallBack | typedef | SCPreferences.h | Authorization/callback/dispatch helpers are not wrapped. |
| SCPreferencesContext | typedef | SCPreferences.h | Authorization/callback/dispatch helpers are not wrapped. |
| SCPreferencesCreateWithAuthorization | function | SCPreferences.h | Authorization/callback/dispatch helpers are not wrapped. |
| SCPreferencesGetTypeID | function | SCPreferences.h | Authorization/callback/dispatch helpers are not wrapped. |
| SCPreferencesNotification | typedef | SCPreferences.h | Authorization/callback/dispatch helpers are not wrapped. |
| SCPreferencesScheduleWithRunLoop | function | SCPreferences.h | Authorization/callback/dispatch helpers are not wrapped. |
| SCPreferencesSetCallback | function | SCPreferences.h | Authorization/callback/dispatch helpers are not wrapped. |
| SCPreferencesSetDispatchQueue | function | SCPreferences.h | Authorization/callback/dispatch helpers are not wrapped. |
| SCPreferencesUnscheduleFromRunLoop | function | SCPreferences.h | Authorization/callback/dispatch helpers are not wrapped. |
| kSCDynamicStoreDomainFile | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCDynamicStoreDomainPlugin | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCDynamicStoreDomainPrefs | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCDynamicStoreDomainSetup | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCDynamicStoreDomainState | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCDynamicStorePropNetInterfaces | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCDynamicStorePropNetPrimaryInterface | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCDynamicStorePropNetPrimaryService | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCDynamicStorePropNetServiceIDs | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCDynamicStorePropSetupCurrentSet | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCDynamicStorePropSetupLastUpdated | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCEntNetFireWire | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCEntNetPPPSerial | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCEntUsersConsoleUser | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNet6to4Relay | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetDNSSearchOrder | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetDNSServerTimeout | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetDNSSortList | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetDNSSupplementalMatchDomains | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetDNSSupplementalMatchOrders | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetEthernetMTU | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetEthernetMediaOptions | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetEthernetMediaSubType | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetIPSecAuthenticationMethod | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetIPSecConnectTime | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetIPSecLocalCertificate | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetIPSecLocalIdentifier | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetIPSecLocalIdentifierType | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetIPSecRemoteAddress | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetIPSecSharedSecret | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetIPSecSharedSecretEncryption | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetIPSecStatus | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetIPSecXAuthEnabled | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetIPSecXAuthName | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetIPSecXAuthPassword | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetIPSecXAuthPasswordEncryption | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetIPv4BroadcastAddresses | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetIPv4DestAddresses | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetIPv6DestAddresses | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetIPv6Flags | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetIPv6PrefixLength | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetInterfaceDeviceName | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetInterfaceHardware | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetInterfaceSubType | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetInterfaceType | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetInterfaces | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetL2TPIPSecSharedSecret | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetL2TPIPSecSharedSecretEncryption | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetL2TPTransport | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetLinkActive | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetLinkDetaching | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetLocalHostName | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetModemAccessPointName | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetModemConnectSpeed | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetModemConnectionPersonality | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetModemConnectionScript | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetModemDataCompression | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetModemDeviceContextID | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetModemDeviceModel | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetModemDeviceVendor | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetModemDialMode | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetModemErrorCorrection | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetModemHoldCallWaitingAudibleAlert | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetModemHoldDisconnectOnAnswer | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetModemHoldEnabled | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetModemHoldReminder | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetModemHoldReminderTime | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetModemNote | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetModemPulseDial | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetModemSpeaker | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetModemSpeed | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetOverridePrimary | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPACSPEnabled | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPAuthName | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPAuthPassword | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPAuthPasswordEncryption | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPAuthPrompt | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPAuthProtocol | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPCCPEnabled | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPCCPMPPE128Enabled | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPCCPMPPE40Enabled | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPCommAlternateRemoteAddress | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPCommConnectDelay | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPCommDisplayTerminalWindow | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPCommRedialCount | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPCommRedialEnabled | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPCommRedialInterval | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPCommRemoteAddress | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPCommTerminalScript | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPCommUseTerminalScript | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPConnectTime | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPDeviceLastCause | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPDialOnDemand | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPDisconnectOnFastUserSwitch | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPDisconnectOnIdle | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPDisconnectOnIdleTimer | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPDisconnectOnLogout | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPDisconnectOnSleep | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPDisconnectTime | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPIPCPCompressionVJ | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPIPCPUsePeerDNS | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPIdleReminder | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPIdleReminderTimer | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPLCPCompressionACField | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPLCPCompressionPField | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPLCPEchoEnabled | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPLCPEchoFailure | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPLCPEchoInterval | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPLCPMRU | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPLCPMTU | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPLCPReceiveACCM | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPLCPTransmitACCM | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPLastCause | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPLogfile | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPOverridePrimary | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPRetryConnectTime | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPSessionTimer | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPStatus | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPUseSessionTimer | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetPPPVerboseLogging | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetProxiesExcludeSimpleHostnames | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetProxiesFTPEnable | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetProxiesFTPPassive | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetProxiesFTPPort | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetProxiesFTPProxy | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetProxiesFTPUser | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetProxiesGopherEnable | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetProxiesGopherPort | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetProxiesGopherProxy | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetProxiesGopherUser | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetProxiesHTTPSUser | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetProxiesHTTPUser | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetProxiesProxyAutoConfigEnable | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetProxiesProxyAutoConfigJavaScript | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetProxiesProxyAutoConfigURLString | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetProxiesProxyAutoDiscoveryEnable | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetProxiesRTSPEnable | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetProxiesRTSPPort | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetProxiesRTSPProxy | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetProxiesRTSPUser | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetProxiesSOCKSUser | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetSMBNetBIOSName | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetSMBNetBIOSNodeType | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetSMBWINSAddresses | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetSMBWorkgroup | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropNetServiceOrder | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropSystemComputerName | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCPropSystemComputerNameEncoding | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCValNetIPSecAuthenticationMethodCertificate | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCValNetIPSecAuthenticationMethodHybrid | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCValNetIPSecAuthenticationMethodSharedSecret | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCValNetIPSecLocalIdentifierTypeKeyID | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCValNetIPSecSharedSecretEncryptionKeychain | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCValNetIPSecXAuthPasswordEncryptionKeychain | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCValNetIPSecXAuthPasswordEncryptionPrompt | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCValNetL2TPIPSecSharedSecretEncryptionKeychain | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCValNetL2TPTransportIP | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCValNetL2TPTransportIPSec | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCValNetModemDialModeIgnoreDialTone | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCValNetModemDialModeManual | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCValNetModemDialModeWaitForDialTone | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCValNetPPPAuthPasswordEncryptionKeychain | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCValNetPPPAuthPasswordEncryptionToken | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCValNetPPPAuthPromptAfter | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCValNetPPPAuthPromptBefore | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCValNetPPPAuthProtocolCHAP | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCValNetPPPAuthProtocolEAP | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCValNetPPPAuthProtocolMSCHAP1 | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCValNetPPPAuthProtocolMSCHAP2 | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCValNetPPPAuthProtocolPAP | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCValNetSMBNetBIOSNodeTypeBroadcast | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCValNetSMBNetBIOSNodeTypeHybrid | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCValNetSMBNetBIOSNodeTypeMixed | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| kSCValNetSMBNetBIOSNodeTypePeer | const | SCSchemaDefinitions.h | Schema::catalog exposes a curated subset; this constant is not surfaced. |
| SCCopyLastError | function | SystemConfiguration.h | CFError-domain helpers are not exposed by the crate. |
| kCFErrorDomainSystemConfiguration | const | SystemConfiguration.h | CFError-domain helpers are not exposed by the crate. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| SCNetworkCheckReachabilityByAddress | function | SCNetwork.h | Excluded from coverage because the SDK marks it deprecated on macOS. | API_DEPRECATED("No longer supported", macos(10.1,10.6)) API_UNAVAILABLE(ios, tvos, watchos); |
| SCNetworkCheckReachabilityByName | function | SCNetwork.h | Excluded from coverage because the SDK marks it deprecated on macOS. | API_DEPRECATED("No longer supported", macos(10.1,10.6)) API_UNAVAILABLE(ios, tvos, watchos); |
| SCNetworkInterfaceRefreshConfiguration | function | SCNetwork.h | Excluded from coverage because the SDK marks it deprecated on macOS. | API_DEPRECATED("No longer supported", macos(10.1,10.5)) API_UNAVAILABLE(ios, tvos, watchos); |
| kSCNetworkInterfaceTypeIrDA | const | SCNetworkConfiguration.h | Wrapped but excluded from coverage because the SDK marks it deprecated on macOS. (NetworkConfiguration::overview) | API_DEPRECATED("No longer supported", macos(10.4,12.0)) API_UNAVAILABLE(ios, tvos, watchos); |
| kSCNetworkInterfaceTypePPTP | const | SCNetworkConfiguration.h | Wrapped but excluded from coverage because the SDK marks it deprecated on macOS. (NetworkConfiguration::overview) | API_DEPRECATED("No longer supported", macos(10.4,10.12)) API_UNAVAILABLE(ios, tvos, watchos); |
| SCNetworkReachabilityCreateWithAddress | function | SCNetworkReachability.h | Wrapped but excluded from coverage because the SDK marks it deprecated on macOS. (Reachability / NetworkReachability; raw_ffi::SCNetworkReachabilityCreateWithAddress (feature="raw-ffi")) | API_DEPRECATED_WITH_REPLACEMENT("Use URLSession or NWConnection to create connections that dynamically handle changing networks. Use NWPathMonitor to enumerate available network interfaces.", macos(10.3, 14.4), ios(2.0, 17.4)); |
| SCNetworkReachabilityCreateWithAddressPair | function | SCNetworkReachability.h | Wrapped but excluded from coverage because the SDK marks it deprecated on macOS. (Reachability / NetworkReachability; raw_ffi::SCNetworkReachabilityCreateWithAddressPair (feature="raw-ffi")) | API_DEPRECATED_WITH_REPLACEMENT("Use URLSession or NWConnection to create connections that dynamically handle changing networks. Use NWPathMonitor to enumerate available network interfaces.", macos(10.3, 14.4), ios(2.0, 17.4)); |
| SCNetworkReachabilityCreateWithName | function | SCNetworkReachability.h | Wrapped but excluded from coverage because the SDK marks it deprecated on macOS. (Reachability / NetworkReachability; raw_ffi::SCNetworkReachabilityCreateWithName (feature="raw-ffi")) | API_DEPRECATED_WITH_REPLACEMENT("Use URLSession or NWConnection to create connections that dynamically handle changing networks. Use NWPathMonitor to enumerate available network interfaces.", macos(10.3, 14.4), ios(2.0, 17.4)); |
| SCNetworkReachabilityGetFlags | function | SCNetworkReachability.h | Wrapped but excluded from coverage because the SDK marks it deprecated on macOS. (Reachability / NetworkReachability; raw_ffi::SCNetworkReachabilityGetFlags (feature="raw-ffi")) | API_DEPRECATED_WITH_REPLACEMENT("Use URLSession or NWConnection to create connections that dynamically handle changing networks. Use NWPathMonitor to enumerate available network interfaces.", macos(10.3, 14.4), ios(2.0, 17.4)); |
| SCNetworkReachabilityGetTypeID | function | SCNetworkReachability.h | Excluded from coverage because the SDK marks it deprecated on macOS. | API_DEPRECATED_WITH_REPLACEMENT("Use URLSession or NWConnection to create connections that dynamically handle changing networks. Use NWPathMonitor to enumerate available network interfaces.", macos(10.3, 14.4), ios(2.0, 17.4)); |
| SCNetworkReachabilityScheduleWithRunLoop | function | SCNetworkReachability.h | Wrapped but excluded from coverage because the SDK marks it deprecated on macOS. (Reachability / NetworkReachability; raw_ffi::SCNetworkReachabilityScheduleWithRunLoop (feature="raw-ffi")) | API_DEPRECATED_WITH_REPLACEMENT("Use URLSession or NWConnection to create connections that dynamically handle changing networks. Use NWPathMonitor to enumerate available network interfaces.", macos(10.3, 14.4), ios(2.0, 17.4)); |
| SCNetworkReachabilitySetCallback | function | SCNetworkReachability.h | Wrapped but excluded from coverage because the SDK marks it deprecated on macOS. (Reachability / NetworkReachability; raw_ffi::SCNetworkReachabilitySetCallback (feature="raw-ffi")) | API_DEPRECATED_WITH_REPLACEMENT("Use URLSession or NWConnection to create connections that dynamically handle changing networks. Use NWPathMonitor to enumerate available network interfaces.", macos(10.3, 14.4), ios(2.0, 17.4)); |
| SCNetworkReachabilitySetDispatchQueue | function | SCNetworkReachability.h | Excluded from coverage because the SDK marks it deprecated on macOS. | API_DEPRECATED_WITH_REPLACEMENT("Use URLSession or NWConnection to create connections that dynamically handle changing networks. Use NWPathMonitor to enumerate available network interfaces.", macos(10.6, 14.4), ios(4.0, 17.4)); |
| SCNetworkReachabilityUnscheduleFromRunLoop | function | SCNetworkReachability.h | Wrapped but excluded from coverage because the SDK marks it deprecated on macOS. (Reachability / NetworkReachability; raw_ffi::SCNetworkReachabilityUnscheduleFromRunLoop (feature="raw-ffi")) | API_DEPRECATED_WITH_REPLACEMENT("Use URLSession or NWConnection to create connections that dynamically handle changing networks. Use NWPathMonitor to enumerate available network interfaces.", macos(10.3, 14.4), ios(2.0, 17.4)); |
| kSCEntNetPPTP | const | SCSchemaDefinitions.h | Wrapped but excluded from coverage because the SDK marks it deprecated on macOS. (Schema::catalog) | API_DEPRECATED("No longer supported", macos(10.3,10.12)) API_UNAVAILABLE(ios); |
| kSCPropNetAirPortAllowNetCreation | const | SCSchemaDefinitions.h | Excluded from coverage because the SDK marks it deprecated on macOS. | API_DEPRECATED("No longer supported", macos(10.2,10.9)) API_UNAVAILABLE(ios, tvos, watchos); |
| kSCPropNetAirPortAuthPassword | const | SCSchemaDefinitions.h | Excluded from coverage because the SDK marks it deprecated on macOS. | API_DEPRECATED("No longer supported", macos(10.1,10.9)) API_UNAVAILABLE(ios, tvos, watchos); |
| kSCPropNetAirPortAuthPasswordEncryption | const | SCSchemaDefinitions.h | Excluded from coverage because the SDK marks it deprecated on macOS. | API_DEPRECATED("No longer supported", macos(10.1,10.9)) API_UNAVAILABLE(ios, tvos, watchos); |
| kSCPropNetAirPortJoinMode | const | SCSchemaDefinitions.h | Excluded from coverage because the SDK marks it deprecated on macOS. | API_DEPRECATED("No longer supported", macos(10.2,10.9)) API_UNAVAILABLE(ios, tvos, watchos); |
| kSCPropNetAirPortPowerEnabled | const | SCSchemaDefinitions.h | Excluded from coverage because the SDK marks it deprecated on macOS. | API_DEPRECATED("No longer supported", macos(10.1,10.9)) API_UNAVAILABLE(ios, tvos, watchos); |
| kSCPropNetAirPortPreferredNetwork | const | SCSchemaDefinitions.h | Excluded from coverage because the SDK marks it deprecated on macOS. | API_DEPRECATED("No longer supported", macos(10.1,10.9)) API_UNAVAILABLE(ios, tvos, watchos); |
| kSCPropNetAirPortSavePasswords | const | SCSchemaDefinitions.h | Excluded from coverage because the SDK marks it deprecated on macOS. | API_DEPRECATED("No longer supported", macos(10.2,10.9)) API_UNAVAILABLE(ios, tvos, watchos); |
| kSCPropNetInterfaceSupportsModemOnHold | const | SCSchemaDefinitions.h | Excluded from coverage because the SDK marks it deprecated on macOS. | API_DEPRECATED("No longer supported", macos(10.2,10.13)) API_UNAVAILABLE(ios, tvos, watchos); |
| kSCPropNetPPPAuthEAPPlugins | const | SCSchemaDefinitions.h | Excluded from coverage because the SDK marks it deprecated on macOS. | API_DEPRECATED("No longer supported", macos(10.3,10.15)) API_UNAVAILABLE(ios, tvos, watchos); |
| kSCPropNetPPPPlugins | const | SCSchemaDefinitions.h | Excluded from coverage because the SDK marks it deprecated on macOS. | API_DEPRECATED("No longer supported", macos(10.2,10.15)) API_UNAVAILABLE(ios, tvos, watchos); |
| kSCPropNetSMBNetBIOSScope | const | SCSchemaDefinitions.h | Excluded from coverage because the SDK marks it deprecated on macOS. | API_DEPRECATED("No longer supported", macos(10.5,10.7)) API_UNAVAILABLE(ios, tvos, watchos); |
| kSCPropUsersConsoleUserGID | const | SCSchemaDefinitions.h | Excluded from coverage because the SDK marks it deprecated on macOS. | API_DEPRECATED("No longer supported", macos(10.1,10.4)) API_UNAVAILABLE(ios, tvos, watchos); |
| kSCPropUsersConsoleUserName | const | SCSchemaDefinitions.h | Excluded from coverage because the SDK marks it deprecated on macOS. | API_DEPRECATED("No longer supported", macos(10.1,10.4)) API_UNAVAILABLE(ios, tvos, watchos); |
| kSCPropUsersConsoleUserUID | const | SCSchemaDefinitions.h | Excluded from coverage because the SDK marks it deprecated on macOS. | API_DEPRECATED("No longer supported", macos(10.1,10.4)) API_UNAVAILABLE(ios, tvos, watchos); |
| kSCValNetAirPortAuthPasswordEncryptionKeychain | const | SCSchemaDefinitions.h | Excluded from coverage because the SDK marks it deprecated on macOS. | API_DEPRECATED("No longer supported", macos(10.3,10.9)) API_UNAVAILABLE(ios, tvos, watchos); |
| kSCValNetAirPortJoinModeAutomatic | const | SCSchemaDefinitions.h | Excluded from coverage because the SDK marks it deprecated on macOS. | API_DEPRECATED("No longer supported", macos(10.3,10.9)) API_UNAVAILABLE(ios, tvos, watchos); |
| kSCValNetAirPortJoinModePreferred | const | SCSchemaDefinitions.h | Excluded from coverage because the SDK marks it deprecated on macOS. | API_DEPRECATED("No longer supported", macos(10.2,10.9)) API_UNAVAILABLE(ios, tvos, watchos); |
| kSCValNetAirPortJoinModeRanked | const | SCSchemaDefinitions.h | Excluded from coverage because the SDK marks it deprecated on macOS. | API_DEPRECATED("No longer supported", macos(10.4,10.9)) API_UNAVAILABLE(ios, tvos, watchos); |
| kSCValNetAirPortJoinModeRecent | const | SCSchemaDefinitions.h | Excluded from coverage because the SDK marks it deprecated on macOS. | API_DEPRECATED("No longer supported", macos(10.2,10.9)) API_UNAVAILABLE(ios, tvos, watchos); |
| kSCValNetAirPortJoinModeStrongest | const | SCSchemaDefinitions.h | Excluded from coverage because the SDK marks it deprecated on macOS. | API_DEPRECATED("No longer supported", macos(10.2,10.9)) API_UNAVAILABLE(ios, tvos, watchos); |
| kSCValNetInterfaceSubTypePPTP | const | SCSchemaDefinitions.h | Wrapped but excluded from coverage because the SDK marks it deprecated on macOS. (Schema::catalog) | API_DEPRECATED("No longer supported", macos(10.2,10.12)) API_UNAVAILABLE(ios); |
