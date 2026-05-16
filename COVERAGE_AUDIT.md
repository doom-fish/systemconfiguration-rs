# systemconfiguration-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 498
VERIFIED: 462
GAPS: 0
EXEMPT: 36
COVERAGE_PCT: 100.00%

This audit parsed the public `SystemConfiguration.framework` headers in `MacOSX26.2.sdk`, filtered out macOS-unavailable declarations, treated macOS-deprecated declarations as EXEMPT, and counted symbols as covered when they are reachable through the safe Swift bridge or the optional `raw-ffi` feature.

All non-exempt public macOS symbols in the audited headers are now surfaced. The final 259 gaps were closed by expanding `Schema::catalog()` to the full `SCSchemaDefinitions.h` catalog and adding the missing `DynamicStore`, `Preferences`, `NetworkConnection`, `BondInterface`, `VlanInterface`, and `SystemConfiguration` helpers.

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
| --- | --- | --- | Swift bridge safe wrapper |
| SCDynamicStoreCallBack | typedef | SCDynamicStore.h | DynamicStore closure callback |
| SCDynamicStoreContext | typedef | SCDynamicStore.h | DynamicStore closure callback |
| SCDynamicStoreCreateRunLoopSource | function | SCDynamicStore.h | DynamicStore::create_run_loop_source / DynamicStoreRunLoopSource |
| SCDynamicStoreGetTypeID | function | SCDynamicStore.h | DynamicStore::type_id |
| SCDynamicStoreSetDispatchQueue | function | SCDynamicStore.h | DynamicStore::set_dispatch_queue_global / clear_dispatch_queue |
| SCDynamicStoreSetMultiple | function | SCDynamicStore.h | DynamicStore::set_multiple |
| DHCPInfoGetLeaseExpirationTime | function | SCDynamicStoreCopyDHCPInfo.h | DynamicStore::dhcp_lease_expiration_time |
| DHCPInfoGetLeaseStartTime | function | SCDynamicStoreCopyDHCPInfo.h | DynamicStore::dhcp_lease_start_time |
| DHCPInfoGetOptionData | function | SCDynamicStoreCopyDHCPInfo.h | DynamicStore::dhcp_option_data |
| SCDynamicStoreKeyCreate | function | SCDynamicStoreKey.h | DynamicStore::key_create |
| SCNetworkConnectionFlags | typedef | SCNetwork.h | NetworkConnectionFlags (= ReachabilityFlags) |
| SCBondInterfaceCopyAll | function | SCNetworkConfiguration.h | BondInterface::copy_all |
| SCBondInterfaceCopyAvailableMemberInterfaces | function | SCNetworkConfiguration.h | BondInterface::copy_available_member_interfaces |
| SCBondInterfaceCopyStatus | function | SCNetworkConfiguration.h | BondInterface::status |
| SCBondInterfaceCreate | function | SCNetworkConfiguration.h | BondInterface::create |
| SCBondInterfaceGetMemberInterfaces | function | SCNetworkConfiguration.h | BondInterface::member_interfaces |
| SCBondInterfaceGetOptions | function | SCNetworkConfiguration.h | BondInterface::options |
| SCBondInterfaceRef | typedef | SCNetworkConfiguration.h | BondInterface |
| SCBondInterfaceRemove | function | SCNetworkConfiguration.h | BondInterface::remove |
| SCBondInterfaceSetLocalizedDisplayName | function | SCNetworkConfiguration.h | BondInterface::set_localized_display_name |
| SCBondInterfaceSetMemberInterfaces | function | SCNetworkConfiguration.h | BondInterface::set_member_interfaces |
| SCBondInterfaceSetOptions | function | SCNetworkConfiguration.h | BondInterface::set_options |
| SCBondStatusGetInterfaceStatus | function | SCNetworkConfiguration.h | BondStatus::interface_status / bond_status |
| SCBondStatusGetMemberInterfaces | function | SCNetworkConfiguration.h | BondStatus::member_interfaces |
| SCBondStatusGetTypeID | function | SCNetworkConfiguration.h | BondStatus::type_id |
| SCBondStatusRef | typedef | SCNetworkConfiguration.h | BondStatus |
| SCNetworkInterfaceCopyMediaOptions | function | SCNetworkConfiguration.h | NetworkInterface::media_options |
| SCNetworkInterfaceCopyMediaSubTypeOptions | function | SCNetworkConfiguration.h | NetworkInterface::media_subtype_options |
| SCNetworkInterfaceCopyMediaSubTypes | function | SCNetworkConfiguration.h | NetworkInterface::media_subtypes |
| SCNetworkInterfaceGetTypeID | function | SCNetworkConfiguration.h | NetworkInterface::type_id |
| SCNetworkInterfaceSetConfiguration | function | SCNetworkConfiguration.h | NetworkInterface::set_configuration |
| SCNetworkInterfaceSetExtendedConfiguration | function | SCNetworkConfiguration.h | NetworkInterface::set_extended_configuration |
| SCNetworkInterfaceSetMTU | function | SCNetworkConfiguration.h | NetworkInterface::set_mtu |
| SCNetworkInterfaceSetMediaOptions | function | SCNetworkConfiguration.h | NetworkInterface::set_media_options |
| SCNetworkProtocolGetTypeID | function | SCNetworkConfiguration.h | NetworkProtocol::type_id |
| SCNetworkServiceCopy | function | SCNetworkConfiguration.h | NetworkService::copy |
| SCNetworkServiceCreate | function | SCNetworkConfiguration.h | NetworkService::create |
| SCNetworkServiceGetTypeID | function | SCNetworkConfiguration.h | NetworkService::type_id |
| SCNetworkSetCopy | function | SCNetworkConfiguration.h | NetworkSet::copy |
| SCNetworkSetCreate | function | SCNetworkConfiguration.h | NetworkSet::create |
| SCNetworkSetGetTypeID | function | SCNetworkConfiguration.h | NetworkSet::type_id |
| SCVLANInterfaceCopyAll | function | SCNetworkConfiguration.h | VlanInterface::copy_all |
| SCVLANInterfaceCopyAvailablePhysicalInterfaces | function | SCNetworkConfiguration.h | VlanInterface::copy_available_physical_interfaces |
| SCVLANInterfaceCreate | function | SCNetworkConfiguration.h | VlanInterface::create |
| SCVLANInterfaceGetOptions | function | SCNetworkConfiguration.h | VlanInterface::options |
| SCVLANInterfaceGetPhysicalInterface | function | SCNetworkConfiguration.h | VlanInterface::physical_interface |
| SCVLANInterfaceGetTag | function | SCNetworkConfiguration.h | VlanInterface::tag |
| SCVLANInterfaceRef | typedef | SCNetworkConfiguration.h | VlanInterface |
| SCVLANInterfaceRemove | function | SCNetworkConfiguration.h | VlanInterface::remove |
| SCVLANInterfaceSetLocalizedDisplayName | function | SCNetworkConfiguration.h | VlanInterface::set_localized_display_name |
| SCVLANInterfaceSetOptions | function | SCNetworkConfiguration.h | VlanInterface::set_options |
| SCVLANInterfaceSetPhysicalInterfaceAndTag | function | SCNetworkConfiguration.h | VlanInterface::set_physical_interface_and_tag |
| kSCBondStatusDeviceAggregationStatus | const | SCNetworkConfiguration.h | BondStatus::device_aggregation_status_key |
| kSCBondStatusDeviceCollecting | const | SCNetworkConfiguration.h | BondStatus::device_collecting_key |
| kSCBondStatusDeviceDistributing | const | SCNetworkConfiguration.h | BondStatus::device_distributing_key |
| kSCNetworkInterfaceIPv4 | const | SCNetworkConfiguration.h | NetworkInterface::ipv4 |
| SCNetworkConnectionCallBack | typedef | SCNetworkConnection.h | NetworkConnection callback closure |
| SCNetworkConnectionContext | typedef | SCNetworkConnection.h | NetworkConnection callback closure |
| SCNetworkConnectionCopyExtendedStatus | function | SCNetworkConnection.h | NetworkConnection::extended_status |
| SCNetworkConnectionCopyServiceID | function | SCNetworkConnection.h | NetworkConnection::service_id |
| SCNetworkConnectionCopyStatistics | function | SCNetworkConnection.h | NetworkConnection::statistics |
| SCNetworkConnectionCopyUserOptions | function | SCNetworkConnection.h | NetworkConnection::user_options |
| SCNetworkConnectionCopyUserPreferences | function | SCNetworkConnection.h | NetworkConnection::copy_user_preferences |
| SCNetworkConnectionCreateWithServiceID | function | SCNetworkConnection.h | NetworkConnection::with_service_id |
| SCNetworkConnectionGetStatus | function | SCNetworkConnection.h | NetworkConnection::status / NetworkConnectionStatus |
| SCNetworkConnectionGetTypeID | function | SCNetworkConnection.h | NetworkConnection::type_id |
| SCNetworkConnectionPPPStatus | typedef | SCNetworkConnection.h | NetworkConnectionPppStatus |
| SCNetworkConnectionRef | typedef | SCNetworkConnection.h | NetworkConnection |
| SCNetworkConnectionScheduleWithRunLoop | function | SCNetworkConnection.h | NetworkConnection::schedule_with_run_loop_current |
| SCNetworkConnectionSetDispatchQueue | function | SCNetworkConnection.h | NetworkConnection::set_dispatch_queue_global / clear_dispatch_queue |
| SCNetworkConnectionStart | function | SCNetworkConnection.h | NetworkConnection::start |
| SCNetworkConnectionStatus | typedef | SCNetworkConnection.h | NetworkConnectionStatus |
| SCNetworkConnectionStop | function | SCNetworkConnection.h | NetworkConnection::stop |
| SCNetworkConnectionUnscheduleFromRunLoop | function | SCNetworkConnection.h | NetworkConnection::unschedule_from_run_loop_current |
| SCPreferencesCallBack | typedef | SCPreferences.h | Preferences callback closure |
| SCPreferencesContext | typedef | SCPreferences.h | Preferences callback closure |
| SCPreferencesCreateWithAuthorization | function | SCPreferences.h | Preferences::new_with_authorization / new_with_authorization_raw |
| SCPreferencesGetTypeID | function | SCPreferences.h | Preferences::type_id |
| SCPreferencesNotification | typedef | SCPreferences.h | PreferencesNotification |
| SCPreferencesScheduleWithRunLoop | function | SCPreferences.h | Preferences::schedule_with_run_loop_current |
| SCPreferencesSetCallback | function | SCPreferences.h | Preferences::set_callback / clear_callback |
| SCPreferencesSetDispatchQueue | function | SCPreferences.h | Preferences::set_dispatch_queue_global / clear_dispatch_queue |
| SCPreferencesUnscheduleFromRunLoop | function | SCPreferences.h | Preferences::unschedule_from_run_loop_current |
| kSCDynamicStoreDomainFile | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCDynamicStoreDomainPlugin | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCDynamicStoreDomainPrefs | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCDynamicStoreDomainSetup | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCDynamicStoreDomainState | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCDynamicStorePropNetInterfaces | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCDynamicStorePropNetPrimaryInterface | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCDynamicStorePropNetPrimaryService | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCDynamicStorePropNetServiceIDs | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCDynamicStorePropSetupCurrentSet | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCDynamicStorePropSetupLastUpdated | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCEntNetFireWire | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCEntNetPPPSerial | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCEntUsersConsoleUser | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNet6to4Relay | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetDNSSearchOrder | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetDNSServerTimeout | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetDNSSortList | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetDNSSupplementalMatchDomains | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetDNSSupplementalMatchOrders | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetEthernetMTU | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetEthernetMediaOptions | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetEthernetMediaSubType | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetIPSecAuthenticationMethod | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetIPSecConnectTime | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetIPSecLocalCertificate | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetIPSecLocalIdentifier | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetIPSecLocalIdentifierType | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetIPSecRemoteAddress | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetIPSecSharedSecret | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetIPSecSharedSecretEncryption | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetIPSecStatus | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetIPSecXAuthEnabled | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetIPSecXAuthName | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetIPSecXAuthPassword | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetIPSecXAuthPasswordEncryption | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetIPv4BroadcastAddresses | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetIPv4DestAddresses | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetIPv6DestAddresses | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetIPv6Flags | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetIPv6PrefixLength | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetInterfaceDeviceName | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetInterfaceHardware | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetInterfaceSubType | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetInterfaceType | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetInterfaces | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetL2TPIPSecSharedSecret | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetL2TPIPSecSharedSecretEncryption | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetL2TPTransport | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetLinkActive | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetLinkDetaching | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetLocalHostName | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetModemAccessPointName | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetModemConnectSpeed | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetModemConnectionPersonality | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetModemConnectionScript | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetModemDataCompression | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetModemDeviceContextID | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetModemDeviceModel | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetModemDeviceVendor | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetModemDialMode | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetModemErrorCorrection | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetModemHoldCallWaitingAudibleAlert | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetModemHoldDisconnectOnAnswer | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetModemHoldEnabled | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetModemHoldReminder | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetModemHoldReminderTime | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetModemNote | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetModemPulseDial | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetModemSpeaker | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetModemSpeed | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetOverridePrimary | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPACSPEnabled | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPAuthName | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPAuthPassword | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPAuthPasswordEncryption | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPAuthPrompt | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPAuthProtocol | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPCCPEnabled | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPCCPMPPE128Enabled | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPCCPMPPE40Enabled | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPCommAlternateRemoteAddress | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPCommConnectDelay | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPCommDisplayTerminalWindow | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPCommRedialCount | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPCommRedialEnabled | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPCommRedialInterval | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPCommRemoteAddress | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPCommTerminalScript | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPCommUseTerminalScript | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPConnectTime | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPDeviceLastCause | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPDialOnDemand | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPDisconnectOnFastUserSwitch | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPDisconnectOnIdle | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPDisconnectOnIdleTimer | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPDisconnectOnLogout | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPDisconnectOnSleep | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPDisconnectTime | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPIPCPCompressionVJ | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPIPCPUsePeerDNS | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPIdleReminder | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPIdleReminderTimer | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPLCPCompressionACField | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPLCPCompressionPField | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPLCPEchoEnabled | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPLCPEchoFailure | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPLCPEchoInterval | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPLCPMRU | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPLCPMTU | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPLCPReceiveACCM | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPLCPTransmitACCM | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPLastCause | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPLogfile | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPOverridePrimary | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPRetryConnectTime | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPSessionTimer | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPStatus | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPUseSessionTimer | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetPPPVerboseLogging | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetProxiesExcludeSimpleHostnames | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetProxiesFTPEnable | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetProxiesFTPPassive | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetProxiesFTPPort | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetProxiesFTPProxy | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetProxiesFTPUser | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetProxiesGopherEnable | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetProxiesGopherPort | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetProxiesGopherProxy | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetProxiesGopherUser | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetProxiesHTTPSUser | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetProxiesHTTPUser | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetProxiesProxyAutoConfigEnable | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetProxiesProxyAutoConfigJavaScript | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetProxiesProxyAutoConfigURLString | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetProxiesProxyAutoDiscoveryEnable | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetProxiesRTSPEnable | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetProxiesRTSPPort | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetProxiesRTSPProxy | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetProxiesRTSPUser | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetProxiesSOCKSUser | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetSMBNetBIOSName | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetSMBNetBIOSNodeType | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetSMBWINSAddresses | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetSMBWorkgroup | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropNetServiceOrder | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropSystemComputerName | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCPropSystemComputerNameEncoding | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCValNetIPSecAuthenticationMethodCertificate | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCValNetIPSecAuthenticationMethodHybrid | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCValNetIPSecAuthenticationMethodSharedSecret | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCValNetIPSecLocalIdentifierTypeKeyID | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCValNetIPSecSharedSecretEncryptionKeychain | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCValNetIPSecXAuthPasswordEncryptionKeychain | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCValNetIPSecXAuthPasswordEncryptionPrompt | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCValNetL2TPIPSecSharedSecretEncryptionKeychain | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCValNetL2TPTransportIP | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCValNetL2TPTransportIPSec | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCValNetModemDialModeIgnoreDialTone | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCValNetModemDialModeManual | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCValNetModemDialModeWaitForDialTone | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCValNetPPPAuthPasswordEncryptionKeychain | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCValNetPPPAuthPasswordEncryptionToken | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCValNetPPPAuthPromptAfter | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCValNetPPPAuthPromptBefore | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCValNetPPPAuthProtocolCHAP | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCValNetPPPAuthProtocolEAP | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCValNetPPPAuthProtocolMSCHAP1 | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCValNetPPPAuthProtocolMSCHAP2 | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCValNetPPPAuthProtocolPAP | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCValNetSMBNetBIOSNodeTypeBroadcast | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCValNetSMBNetBIOSNodeTypeHybrid | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCValNetSMBNetBIOSNodeTypeMixed | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| kSCValNetSMBNetBIOSNodeTypePeer | const | SCSchemaDefinitions.h | Schema::catalog().all / SchemaCatalog::get |
| SCCopyLastError | function | SystemConfiguration.h | SystemConfiguration::copy_last_error |
| kCFErrorDomainSystemConfiguration | const | SystemConfiguration.h | SystemConfiguration::error_domain |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| _None_ | - | - | All non-exempt public macOS symbols in the audited headers are now surfaced via the safe bridge and/or the optional `raw-ffi` layer. |

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
