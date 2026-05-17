# SystemConfiguration.framework coverage audit

Audit basis:

- `SCDynamicStore.h`
- `SCDynamicStoreCopySpecific.h`
- `SCDynamicStoreKey.h`
- `SCDynamicStoreCopyDHCPInfo.h`
- `SCNetworkConfiguration.h`
- `SCNetworkReachability.h`
- `SCPreferences.h`
- `SCPreferencesPath.h`
- `SCPreferencesSetSpecific.h`
- `SCSchemaDefinitions.h`
- `CaptiveNetwork.h`

Legend:

- ✅ implemented — shipped as a safe Swift-bridge wrapper in `0.2.2`
- ⏭️ skipped — intentionally omitted because the API is unavailable on modern
  macOS or otherwise unsuitable for this crate

Current safe-wrapper status: 100% of the non-skipped APIs in the audited headers
are exposed.

## DynamicStore

### `SCDynamicStore.h`

- ✅ `SCDynamicStoreCreate`, `SCDynamicStoreCopyKeyList`,
  `SCDynamicStoreCopyValue`, `SCDynamicStoreCopyMultiple`,
  `SCDynamicStoreAddValue`, `SCDynamicStoreAddTemporaryValue`,
  `SCDynamicStoreSetValue`, `SCDynamicStoreRemoveValue`,
  `SCDynamicStoreNotifyValue`, `SCDynamicStoreSetNotificationKeys`,
  `SCDynamicStoreCopyNotifiedKeys`, `SCDynamicStoreCreateRunLoopSource`,
  `SCDynamicStoreGetTypeID`, `SCDynamicStoreSetDispatchQueue`, and
  `SCDynamicStoreSetMultiple` are wrapped by `DynamicStore` /
  `DynamicStoreRunLoopSource`.
- ✅ `SCDynamicStoreCreateWithOptions` is wrapped by
  `DynamicStore::new_with_options` and
  `DynamicStore::new_with_options_and_callback`.

### `SCDynamicStoreCopySpecific.h`

- ✅ `SCDynamicStoreCopyComputerName`, `SCDynamicStoreCopyLocalHostName`,
  `SCDynamicStoreCopyLocation`, and `SCDynamicStoreCopyProxies` are wrapped by
  `DynamicStore`.
- ✅ `SCDynamicStoreCopyConsoleUser` is wrapped by `ConsoleUser::current()`.

### `SCDynamicStoreKey.h`

- ✅ `SCDynamicStoreKeyCreate`, `SCDynamicStoreKeyCreateComputerName`,
  `SCDynamicStoreKeyCreateConsoleUser`, `SCDynamicStoreKeyCreateHostNames`,
  `SCDynamicStoreKeyCreateLocation`, `SCDynamicStoreKeyCreateNetworkGlobalEntity`,
  `SCDynamicStoreKeyCreateNetworkInterface`,
  `SCDynamicStoreKeyCreateNetworkInterfaceEntity`,
  `SCDynamicStoreKeyCreateNetworkServiceEntity`, and
  `SCDynamicStoreKeyCreateProxies` are wrapped by `DynamicStore` key helpers.

### `SCDynamicStoreCopyDHCPInfo.h`

- ✅ `SCDynamicStoreCopyDHCPInfo`, `DHCPInfoGetOptionData`,
  `DHCPInfoGetLeaseStartTime`, and `DHCPInfoGetLeaseExpirationTime` are
  wrapped by `DynamicStore` DHCP helpers.

## Preferences

### `SCPreferences.h`

- ✅ `SCPreferencesCreate`, `SCPreferencesCreateWithAuthorization`,
  `SCPreferencesGetTypeID`, `SCPreferencesLock`,
  `SCPreferencesCommitChanges`, `SCPreferencesApplyChanges`,
  `SCPreferencesUnlock`, `SCPreferencesGetSignature`,
  `SCPreferencesCopyKeyList`, `SCPreferencesGetValue`,
  `SCPreferencesAddValue`, `SCPreferencesSetValue`,
  `SCPreferencesRemoveValue`, `SCPreferencesSetCallback`,
  `SCPreferencesScheduleWithRunLoop`, `SCPreferencesUnscheduleFromRunLoop`,
  `SCPreferencesSetDispatchQueue`, and `SCPreferencesSynchronize` are wrapped by
  `Preferences` / `PreferencesNotification`.

### `SCPreferencesPath.h`

- ✅ `SCPreferencesPathCreateUniqueChild`, `SCPreferencesPathGetValue`,
  `SCPreferencesPathGetLink`, `SCPreferencesPathSetValue`,
  `SCPreferencesPathSetLink`, and `SCPreferencesPathRemoveValue` are wrapped by
  `Preferences` path helpers.

### `SCPreferencesSetSpecific.h`

- ✅ `SCPreferencesSetComputerName` and `SCPreferencesSetLocalHostName` are
  wrapped by `Preferences`.

## Reachability

### `SCNetworkReachability.h`

- ✅ `SCNetworkReachabilityCreateWithAddress`,
  `SCNetworkReachabilityCreateWithAddressPair`,
  `SCNetworkReachabilityCreateWithName`, `SCNetworkReachabilityGetFlags`,
  `SCNetworkReachabilityGetTypeID`, `SCNetworkReachabilitySetCallback`,
  `SCNetworkReachabilityScheduleWithRunLoop`,
  `SCNetworkReachabilitySetDispatchQueue`, and
  `SCNetworkReachabilityUnscheduleFromRunLoop` are wrapped by `Reachability`.

## Network configuration families

`NetworkConfiguration::overview()` provides a high-level snapshot of interfaces,
services, and sets, while the modules below expose the directly wrapped object
families.

### `SCNetworkConfiguration.h` — `SCNetworkInterface*`

- ✅ `SCNetworkInterfaceCopyAll`, `SCNetworkInterfaceCopyMTU`,
  `SCNetworkInterfaceCreateWithInterface`,
  `SCNetworkInterfaceForceConfigurationRefresh`,
  `SCNetworkInterfaceGetBSDName`, `SCNetworkInterfaceGetConfiguration`,
  `SCNetworkInterfaceGetExtendedConfiguration`,
  `SCNetworkInterfaceGetHardwareAddressString`,
  `SCNetworkInterfaceGetInterface`, `SCNetworkInterfaceGetInterfaceType`,
  `SCNetworkInterfaceGetLocalizedDisplayName`,
  `SCNetworkInterfaceGetSupportedInterfaceTypes`,
  `SCNetworkInterfaceGetSupportedProtocolTypes`,
  `SCNetworkInterfaceGetTypeID`, `SCNetworkInterfaceCopyMediaOptions`,
  `SCNetworkInterfaceCopyMediaSubTypeOptions`,
  `SCNetworkInterfaceCopyMediaSubTypes`, `SCNetworkInterfaceSetConfiguration`,
  `SCNetworkInterfaceSetExtendedConfiguration`, `SCNetworkInterfaceSetMTU`, and
  `SCNetworkInterfaceSetMediaOptions` are wrapped by `NetworkInterface`.

### `SCNetworkConfiguration.h` — `SCNetworkProtocol*`

- ✅ `SCNetworkProtocolGetConfiguration`, `SCNetworkProtocolGetEnabled`,
  `SCNetworkProtocolGetProtocolType`, `SCNetworkProtocolGetTypeID`,
  `SCNetworkProtocolSetConfiguration`, and `SCNetworkProtocolSetEnabled` are
  wrapped by `NetworkProtocol`.

### `SCNetworkConfiguration.h` — `SCNetworkService*`

- ✅ `SCNetworkServiceCopyAll`, `SCNetworkServiceCopy`,
  `SCNetworkServiceCopyProtocols`, `SCNetworkServiceCopyProtocol`,
  `SCNetworkServiceAddProtocolType`, `SCNetworkServiceCreate`,
  `SCNetworkServiceEstablishDefaultConfiguration`,
  `SCNetworkServiceGetEnabled`, `SCNetworkServiceGetInterface`,
  `SCNetworkServiceGetName`, `SCNetworkServiceGetServiceID`,
  `SCNetworkServiceGetTypeID`, `SCNetworkServiceRemove`,
  `SCNetworkServiceRemoveProtocolType`, `SCNetworkServiceSetEnabled`, and
  `SCNetworkServiceSetName` are wrapped by `NetworkService`.

### `SCNetworkConfiguration.h` — `SCNetworkSet*`

- ✅ `SCNetworkSetCopyAll`, `SCNetworkSetCopyCurrent`, `SCNetworkSetCopy`,
  `SCNetworkSetCopyServices`, `SCNetworkSetCreate`, `SCNetworkSetGetName`,
  `SCNetworkSetGetServiceOrder`, `SCNetworkSetGetSetID`,
  `SCNetworkSetGetTypeID`, `SCNetworkSetContainsInterface`,
  `SCNetworkSetAddService`, `SCNetworkSetRemove`,
  `SCNetworkSetRemoveService`, `SCNetworkSetSetCurrent`,
  `SCNetworkSetSetName`, and `SCNetworkSetSetServiceOrder` are wrapped by
  `NetworkSet`.

### `SCNetworkConfiguration.h` — `SCBondInterface*` / `SCBondStatus*`

- ✅ `SCBondInterfaceCopyAll`, `SCBondInterfaceCopyAvailableMemberInterfaces`,
  `SCBondInterfaceCopyStatus`, `SCBondInterfaceCreate`,
  `SCBondInterfaceGetMemberInterfaces`, `SCBondInterfaceGetOptions`,
  `SCBondInterfaceRemove`, `SCBondInterfaceSetLocalizedDisplayName`,
  `SCBondInterfaceSetMemberInterfaces`, `SCBondInterfaceSetOptions`,
  `SCBondStatusGetInterfaceStatus`, `SCBondStatusGetMemberInterfaces`,
  `SCBondStatusGetTypeID`, and the `kSCBondStatus*` member-status keys are
  wrapped by `BondInterface` / `BondStatus`.

### `SCNetworkConfiguration.h` — `SCVLANInterface*`

- ✅ `SCVLANInterfaceCopyAll`,
  `SCVLANInterfaceCopyAvailablePhysicalInterfaces`, `SCVLANInterfaceCreate`,
  `SCVLANInterfaceGetOptions`, `SCVLANInterfaceGetPhysicalInterface`,
  `SCVLANInterfaceGetTag`, `SCVLANInterfaceRemove`,
  `SCVLANInterfaceSetLocalizedDisplayName`, `SCVLANInterfaceSetOptions`, and
  `SCVLANInterfaceSetPhysicalInterfaceAndTag` are wrapped by `VlanInterface`.

## Network connections

### `SCNetworkConnection.h`

- ✅ `SCNetworkConnectionCreateWithServiceID`,
  `SCNetworkConnectionCopyUserPreferences`, `SCNetworkConnectionCopyServiceID`,
  `SCNetworkConnectionCopyExtendedStatus`,
  `SCNetworkConnectionCopyStatistics`, `SCNetworkConnectionCopyUserOptions`,
  `SCNetworkConnectionGetStatus`, `SCNetworkConnectionGetTypeID`,
  `SCNetworkConnectionScheduleWithRunLoop`,
  `SCNetworkConnectionUnscheduleFromRunLoop`,
  `SCNetworkConnectionSetDispatchQueue`, `SCNetworkConnectionStart`, and
  `SCNetworkConnectionStop` are wrapped by `NetworkConnection` plus the
  `NetworkConnectionStatus`, `NetworkConnectionPppStatus`, and
  `NetworkConnectionFlags` Rust types.

## Schema

### `SCSchemaDefinitions.h`

- ✅ `Schema::catalog()` now exposes the full `SCSchemaDefinitions.h` constant
  set through `SchemaCatalog::all` / `SchemaCatalog::get`, while preserving the
  original curated convenience groups for reserved keys, preferences,
  components, entities, generic properties, IPv4, IPv6, DNS, proxies, and
  interface types.

## SystemConfiguration error helpers

### `SystemConfiguration.h`

- ✅ `SCCopyLastError` and `kCFErrorDomainSystemConfiguration` are wrapped by
  `SystemConfiguration::copy_last_error()` and `SystemConfiguration::error_domain()`.

## Console user

- ✅ `ConsoleUser::current()` provides a dedicated safe wrapper over
  `SCDynamicStoreCopyConsoleUser`.

## Captive network

### `CaptiveNetwork.h`

- ✅ `CNSetSupportedSSIDs`, `CNMarkPortalOnline`, `CNMarkPortalOffline`, and
  `CNCopySupportedInterfaces` are wrapped by `CaptiveNetwork`.
- ⏭️ `CNCopyCurrentNetworkInfo` is intentionally skipped because it is
  unavailable on modern macOS and typically requires entitlements even where it
  still exists.
