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

- ✅ implemented — shipped as a safe Swift-bridge wrapper in `0.2.0`
- 🟡 partial — some access exists, but the safe API is incomplete or the symbol
  is only represented indirectly
- ⏭️ skipped — intentionally omitted because the API is unavailable on modern
  macOS or otherwise unsuitable for this crate

## DynamicStore

### `SCDynamicStore.h`

- ✅ `SCDynamicStoreCreate`, `SCDynamicStoreCopyKeyList`,
  `SCDynamicStoreCopyValue`, `SCDynamicStoreCopyMultiple`,
  `SCDynamicStoreAddValue`, `SCDynamicStoreAddTemporaryValue`,
  `SCDynamicStoreSetValue`, `SCDynamicStoreRemoveValue`,
  `SCDynamicStoreNotifyValue`, `SCDynamicStoreSetNotificationKeys`, and
  `SCDynamicStoreCopyNotifiedKeys` are wrapped by `DynamicStore`.
- 🟡 `SCDynamicStoreCreateWithOptions` is only represented by the crate's
  session-key creation mode; arbitrary options dictionaries are not yet exposed.
- 🟡 `SCDynamicStoreCreateRunLoopSource` is not surfaced as a safe Rust type.
- 🟡 `SCDynamicStoreGetTypeID` is not surfaced.
- 🟡 `SCDynamicStoreSetDispatchQueue` is not wrapped.
- 🟡 `SCDynamicStoreSetMultiple` is not wrapped.

### `SCDynamicStoreCopySpecific.h`

- ✅ `SCDynamicStoreCopyComputerName`, `SCDynamicStoreCopyLocalHostName`,
  `SCDynamicStoreCopyLocation`, and `SCDynamicStoreCopyProxies` are wrapped by
  `DynamicStore`.
- ✅ `SCDynamicStoreCopyConsoleUser` is wrapped by `ConsoleUser::current()`.

### `SCDynamicStoreKey.h`

- ✅ `SCDynamicStoreKeyCreateComputerName`,
  `SCDynamicStoreKeyCreateConsoleUser`, `SCDynamicStoreKeyCreateHostNames`,
  `SCDynamicStoreKeyCreateLocation`, `SCDynamicStoreKeyCreateNetworkGlobalEntity`,
  `SCDynamicStoreKeyCreateNetworkInterface`,
  `SCDynamicStoreKeyCreateNetworkInterfaceEntity`,
  `SCDynamicStoreKeyCreateNetworkServiceEntity`, and
  `SCDynamicStoreKeyCreateProxies` are wrapped by `DynamicStore` key helpers.
- 🟡 `SCDynamicStoreKeyCreate` is not wrapped directly.

### `SCDynamicStoreCopyDHCPInfo.h`

- ✅ `SCDynamicStoreCopyDHCPInfo` is wrapped by `DynamicStore::dhcp_info()`.

## Preferences

### `SCPreferences.h`

- ✅ `SCPreferencesCreate`, `SCPreferencesLock`,
  `SCPreferencesCommitChanges`, `SCPreferencesApplyChanges`,
  `SCPreferencesUnlock`, `SCPreferencesGetSignature`,
  `SCPreferencesCopyKeyList`, `SCPreferencesGetValue`,
  `SCPreferencesAddValue`, `SCPreferencesSetValue`,
  `SCPreferencesRemoveValue`, and `SCPreferencesSynchronize` are wrapped by
  `Preferences`.
- 🟡 `SCPreferencesCreateWithAuthorization` is not wrapped.
- 🟡 `SCPreferencesGetTypeID` is not surfaced.
- 🟡 `SCPreferencesSetCallback`, `SCPreferencesScheduleWithRunLoop`,
  `SCPreferencesUnscheduleFromRunLoop`, and `SCPreferencesSetDispatchQueue` are
  not wrapped.

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
  `SCNetworkReachabilitySetCallback`,
  `SCNetworkReachabilityScheduleWithRunLoop`, and
  `SCNetworkReachabilityUnscheduleFromRunLoop` are wrapped by `Reachability`.
- 🟡 `SCNetworkReachabilityGetTypeID` is not surfaced.
- 🟡 `SCNetworkReachabilitySetDispatchQueue` is not wrapped.

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
  `SCNetworkInterfaceGetSupportedInterfaceTypes`, and
  `SCNetworkInterfaceGetSupportedProtocolTypes` are wrapped by
  `NetworkInterface`.
- 🟡 `SCNetworkInterfaceGetTypeID` is not surfaced.
- 🟡 `SCNetworkInterfaceCopyMediaOptions`,
  `SCNetworkInterfaceCopyMediaSubTypeOptions`,
  `SCNetworkInterfaceCopyMediaSubTypes`, `SCNetworkInterfaceSetConfiguration`,
  `SCNetworkInterfaceSetExtendedConfiguration`, `SCNetworkInterfaceSetMTU`, and
  `SCNetworkInterfaceSetMediaOptions` are not wrapped.

### `SCNetworkConfiguration.h` — `SCNetworkProtocol*`

- ✅ `SCNetworkProtocolGetConfiguration`, `SCNetworkProtocolGetEnabled`,
  `SCNetworkProtocolGetProtocolType`, `SCNetworkProtocolSetConfiguration`, and
  `SCNetworkProtocolSetEnabled` are wrapped by `NetworkProtocol`.
- 🟡 `SCNetworkProtocolGetTypeID` is not surfaced.

### `SCNetworkConfiguration.h` — `SCNetworkService*`

- ✅ `SCNetworkServiceCopyAll`, `SCNetworkServiceCopyProtocols`,
  `SCNetworkServiceCopyProtocol`, `SCNetworkServiceAddProtocolType`,
  `SCNetworkServiceEstablishDefaultConfiguration`,
  `SCNetworkServiceGetEnabled`, `SCNetworkServiceGetInterface`,
  `SCNetworkServiceGetName`, `SCNetworkServiceGetServiceID`,
  `SCNetworkServiceRemove`, `SCNetworkServiceRemoveProtocolType`,
  `SCNetworkServiceSetEnabled`, and `SCNetworkServiceSetName` are wrapped by
  `NetworkService`.
- 🟡 `SCNetworkServiceCopy`, `SCNetworkServiceCreate`, and
  `SCNetworkServiceGetTypeID` are not wrapped.

### `SCNetworkConfiguration.h` — `SCNetworkSet*`

- ✅ `SCNetworkSetCopyAll`, `SCNetworkSetCopyCurrent`,
  `SCNetworkSetCopyServices`, `SCNetworkSetGetName`,
  `SCNetworkSetGetServiceOrder`, `SCNetworkSetGetSetID`,
  `SCNetworkSetContainsInterface`, `SCNetworkSetAddService`,
  `SCNetworkSetRemove`, `SCNetworkSetRemoveService`,
  `SCNetworkSetSetCurrent`, `SCNetworkSetSetName`, and
  `SCNetworkSetSetServiceOrder` are wrapped by `NetworkSet`.
- 🟡 `SCNetworkSetCopy`, `SCNetworkSetCreate`, and `SCNetworkSetGetTypeID` are
  not wrapped.

### `SCNetworkConfiguration.h` — `SCBondInterface*` / `SCBondStatus*`

- 🟡 `SCBondInterfaceCopyAll`, `SCBondInterfaceCopyAvailableMemberInterfaces`,
  `SCBondInterfaceCopyStatus`, `SCBondInterfaceCreate`,
  `SCBondInterfaceGetMemberInterfaces`, `SCBondInterfaceGetOptions`,
  `SCBondInterfaceRemove`, `SCBondInterfaceSetLocalizedDisplayName`,
  `SCBondInterfaceSetMemberInterfaces`, `SCBondInterfaceSetOptions`,
  `SCBondStatusGetInterfaceStatus`, `SCBondStatusGetMemberInterfaces`, and
  `SCBondStatusGetTypeID` are not wrapped yet.

### `SCNetworkConfiguration.h` — `SCVLANInterface*`

- 🟡 `SCVLANInterfaceCopyAll`,
  `SCVLANInterfaceCopyAvailablePhysicalInterfaces`, `SCVLANInterfaceCreate`,
  `SCVLANInterfaceGetOptions`, `SCVLANInterfaceGetPhysicalInterface`,
  `SCVLANInterfaceGetTag`, `SCVLANInterfaceRemove`,
  `SCVLANInterfaceSetLocalizedDisplayName`, `SCVLANInterfaceSetOptions`, and
  `SCVLANInterfaceSetPhysicalInterfaceAndTag` are not wrapped yet.

## Schema

### `SCSchemaDefinitions.h`

- ✅ `Schema::catalog()` currently mirrors the highest-value constant groups used
  by the crate: reserved keys, preference keys, component names, entity names,
  generic properties, IPv4, IPv6, DNS, proxies, and interface types.
- 🟡 The header contains many more `kSC*` constants than the current curated
  catalog; the crate does not yet mirror every individual constant.

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
