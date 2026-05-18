// Reserved Keys
/// Wraps `kSCResvLink`.
pub const SC_RESV_LINK: &str = "__LINK__";
/// Wraps `kSCResvInactive`.
pub const SC_RESV_INACTIVE: &str = "__INACTIVE__";

// Generic Keys
/// Wraps `kSCPropInterfaceName`.
pub const SC_PROP_INTERFACE_NAME: &str = "InterfaceName";
/// Wraps `kSCPropMACAddress`.
pub const SC_PROP_MAC_ADDRESS: &str = "MACAddress";
/// Wraps `kSCPropUserDefinedName`.
pub const SC_PROP_USER_DEFINED_NAME: &str = "UserDefinedName";
/// Wraps `kSCPropVersion`.
pub const SC_PROP_VERSION: &str = "Version";

// Preference Keys
/// Wraps `kSCPrefCurrentSet`.
pub const SC_PREF_CURRENT_SET: &str = "CurrentSet";
/// Wraps `kSCPrefNetworkServices`.
pub const SC_PREF_NETWORK_SERVICES: &str = "NetworkServices";
/// Wraps `kSCPrefSets`.
pub const SC_PREF_SETS: &str = "Sets";
/// Wraps `kSCPrefSystem`.
pub const SC_PREF_SYSTEM: &str = "System";

// Component Keys
/// Wraps `kSCCompNetwork`.
pub const SC_COMP_NETWORK: &str = "Network";
/// Wraps `kSCCompService`.
pub const SC_COMP_SERVICE: &str = "Service";
/// Wraps `kSCCompGlobal`.
pub const SC_COMP_GLOBAL: &str = "Global";
/// Wraps `kSCCompHostNames`.
pub const SC_COMP_HOST_NAMES: &str = "HostNames";
/// Wraps `kSCCompInterface`.
pub const SC_COMP_INTERFACE: &str = "Interface";
/// Wraps `kSCCompSystem`.
pub const SC_COMP_SYSTEM: &str = "System";
/// Wraps `kSCCompUsers`.
pub const SC_COMP_USERS: &str = "Users";
/// Wraps `kSCCompAnyRegex`.
pub const SC_COMP_ANY_REGEX: &str = "[^/]+";
/// Wraps `kSCCompNetworkHostNames`.
pub const SC_COMP_NETWORK_HOST_NAMES: &str = "HostNames";

// Network Entity Keys
/// Wraps `kSCEntNetAirPort`.
pub const SC_ENT_NET_AIR_PORT: &str = "AirPort";
/// Wraps `kSCEntNetDHCP`.
pub const SC_ENT_NET_DHCP: &str = "DHCP";
/// Wraps `kSCEntNetDNS`.
pub const SC_ENT_NET_DNS: &str = "DNS";
/// Wraps `kSCEntNetEthernet`.
pub const SC_ENT_NET_ETHERNET: &str = "Ethernet";
/// Wraps `kSCEntNetFireWire`.
pub const SC_ENT_NET_FIRE_WIRE: &str = "FireWire";
/// Wraps `kSCEntNetInterface`.
pub const SC_ENT_NET_INTERFACE: &str = "Interface";
/// Wraps `kSCEntNetIPSec`.
pub const SC_ENT_NET_IP_SEC: &str = "IPSec";
/// Wraps `kSCEntNetIPv4`.
pub const SC_ENT_NET_IPV4: &str = "IPv4";
/// Wraps `kSCEntNetIPv6`.
pub const SC_ENT_NET_IPV6: &str = "IPv6";
/// Wraps `kSCEntNetL2TP`.
pub const SC_ENT_NET_L2TP: &str = "L2TP";
/// Wraps `kSCEntNetLink`.
pub const SC_ENT_NET_LINK: &str = "Link";
/// Wraps `kSCEntNetModem`.
pub const SC_ENT_NET_MODEM: &str = "Modem";
/// Wraps `kSCEntNetPPP`.
pub const SC_ENT_NET_PPP: &str = "PPP";
/// Wraps `kSCEntNetPPPoE`.
pub const SC_ENT_NET_PPPOE: &str = "PPPoE";
/// Wraps `kSCEntNetPPPSerial`.
pub const SC_ENT_NET_PPP_SERIAL: &str = "PPPSerial";
/// Wraps `kSCEntNetPPTP`.
pub const SC_ENT_NET_PPTP: &str = "PPTP";
/// Wraps `kSCEntNetProxies`.
pub const SC_ENT_NET_PROXIES: &str = "Proxies";
/// Wraps `kSCEntNetSMB`.
pub const SC_ENT_NET_SMB: &str = "SMB";
/// Wraps `kSCEntNet6to4`.
pub const SC_ENT_NET_6TO4: &str = "6to4";
/// Wraps `kSCEntUsersConsoleUser`.
pub const SC_ENT_USERS_CONSOLE_USER: &str = "ConsoleUser";

// kSCCompNetwork Properties
/// Wraps `kSCPropNetOverridePrimary`.
pub const SC_PROP_NET_OVERRIDE_PRIMARY: &str = "OverridePrimary";
/// Wraps `kSCPropNetServiceOrder`.
pub const SC_PROP_NET_SERVICE_ORDER: &str = "ServiceOrder";
/// Wraps `kSCPropNetPPPOverridePrimary`.
pub const SC_PROP_NET_PPP_OVERRIDE_PRIMARY: &str = "PPPOverridePrimary";
/// Wraps `kSCPropNetServiceSubType`.
pub const SC_PROP_NET_SERVICE_SUB_TYPE: &str = "ServiceSubType";

// kSCCompNetworkInterface Properties
/// Wraps `kSCPropNetInterfaces`.
pub const SC_PROP_NET_INTERFACES: &str = "Interfaces";

// kSCCompNetworkHostNames Properties
/// Wraps `kSCPropNetLocalHostName`.
pub const SC_PROP_NET_LOCAL_HOST_NAME: &str = "LocalHostName";

// DNS
/// Wraps `kSCPropNetDNSDomainName`.
pub const SC_PROP_NET_DNS_DOMAIN_NAME: &str = "DomainName";
/// Wraps `kSCPropNetDNSOptions`.
pub const SC_PROP_NET_DNS_OPTIONS: &str = "Options";
/// Wraps `kSCPropNetDNSSearchDomains`.
pub const SC_PROP_NET_DNS_SEARCH_DOMAINS: &str = "SearchDomains";
/// Wraps `kSCPropNetDNSSearchOrder`.
pub const SC_PROP_NET_DNS_SEARCH_ORDER: &str = "SearchOrder";
/// Wraps `kSCPropNetDNSServerAddresses`.
pub const SC_PROP_NET_DNS_SERVER_ADDRESSES: &str = "ServerAddresses";
/// Wraps `kSCPropNetDNSServerPort`.
pub const SC_PROP_NET_DNS_SERVER_PORT: &str = "ServerPort";
/// Wraps `kSCPropNetDNSServerTimeout`.
pub const SC_PROP_NET_DNS_SERVER_TIMEOUT: &str = "ServerTimeout";
/// Wraps `kSCPropNetDNSSortList`.
pub const SC_PROP_NET_DNS_SORT_LIST: &str = "SortList";
/// Wraps `kSCPropNetDNSSupplementalMatchDomains`.
pub const SC_PROP_NET_DNS_SUPPLEMENTAL_MATCH_DOMAINS: &str = "SupplementalMatchDomains";
/// Wraps `kSCPropNetDNSSupplementalMatchOrders`.
pub const SC_PROP_NET_DNS_SUPPLEMENTAL_MATCH_ORDERS: &str = "SupplementalMatchOrders";

// Ethernet
/// Wraps `kSCPropNetEthernetMediaSubType`.
pub const SC_PROP_NET_ETHERNET_MEDIA_SUB_TYPE: &str = "MediaSubType";
/// Wraps `kSCPropNetEthernetMediaOptions`.
pub const SC_PROP_NET_ETHERNET_MEDIA_OPTIONS: &str = "MediaOptions";
/// Wraps `kSCPropNetEthernetMTU`.
pub const SC_PROP_NET_ETHERNET_MTU: &str = "MTU";

// Interface entity
/// Wraps `kSCPropNetInterfaceDeviceName`.
pub const SC_PROP_NET_INTERFACE_DEVICE_NAME: &str = "DeviceName";
/// Wraps `kSCPropNetInterfaceHardware`.
pub const SC_PROP_NET_INTERFACE_HARDWARE: &str = "Hardware";
/// Wraps `kSCPropNetInterfaceType`.
pub const SC_PROP_NET_INTERFACE_TYPE: &str = "Type";
/// Wraps `kSCPropNetInterfaceSubType`.
pub const SC_PROP_NET_INTERFACE_SUB_TYPE: &str = "SubType";
/// Wraps `kSCPropNetInterfaceSupportsModemOnHold`.
pub const SC_PROP_NET_INTERFACE_SUPPORTS_MODEM_ON_HOLD: &str = "SupportsModemOnHold";

// Interface type values
/// Wraps `kSCValNetInterfaceTypeEthernet`.
pub const SC_VAL_NET_INTERFACE_TYPE_ETHERNET: &str = "Ethernet";
/// Wraps `kSCValNetInterfaceTypeFireWire`.
pub const SC_VAL_NET_INTERFACE_TYPE_FIRE_WIRE: &str = "FireWire";
/// Wraps `kSCValNetInterfaceTypePPP`.
pub const SC_VAL_NET_INTERFACE_TYPE_PPP: &str = "PPP";
/// Wraps `kSCValNetInterfaceType6to4`.
pub const SC_VAL_NET_INTERFACE_TYPE_6TO4: &str = "6to4";
/// Wraps `kSCValNetInterfaceTypeIPSec`.
pub const SC_VAL_NET_INTERFACE_TYPE_IP_SEC: &str = "IPSec";

// Interface subtype values
/// Wraps `kSCValNetInterfaceSubTypePPPoE`.
pub const SC_VAL_NET_INTERFACE_SUB_TYPE_PPPOE: &str = "PPPoE";
/// Wraps `kSCValNetInterfaceSubTypePPPSerial`.
pub const SC_VAL_NET_INTERFACE_SUB_TYPE_PPP_SERIAL: &str = "PPPSerial";
/// Wraps `kSCValNetInterfaceSubTypePPTP`.
pub const SC_VAL_NET_INTERFACE_SUB_TYPE_PPTP: &str = "PPTP";
/// Wraps `kSCValNetInterfaceSubTypeL2TP`.
pub const SC_VAL_NET_INTERFACE_SUB_TYPE_L2TP: &str = "L2TP";

// IPSec
/// Wraps `kSCPropNetIPSecAuthenticationMethod`.
pub const SC_PROP_NET_IP_SEC_AUTHENTICATION_METHOD: &str = "AuthenticationMethod";
/// Wraps `kSCPropNetIPSecLocalCertificate`.
pub const SC_PROP_NET_IP_SEC_LOCAL_CERTIFICATE: &str = "LocalCertificate";
/// Wraps `kSCPropNetIPSecLocalIdentifier`.
pub const SC_PROP_NET_IP_SEC_LOCAL_IDENTIFIER: &str = "LocalIdentifier";
/// Wraps `kSCPropNetIPSecLocalIdentifierType`.
pub const SC_PROP_NET_IP_SEC_LOCAL_IDENTIFIER_TYPE: &str = "LocalIdentifierType";
/// Wraps `kSCPropNetIPSecSharedSecret`.
pub const SC_PROP_NET_IP_SEC_SHARED_SECRET: &str = "SharedSecret";
/// Wraps `kSCPropNetIPSecSharedSecretEncryption`.
pub const SC_PROP_NET_IP_SEC_SHARED_SECRET_ENCRYPTION: &str = "SharedSecretEncryption";
/// Wraps `kSCPropNetIPSecConnectTime`.
pub const SC_PROP_NET_IP_SEC_CONNECT_TIME: &str = "ConnectTime";
/// Wraps `kSCPropNetIPSecRemoteAddress`.
pub const SC_PROP_NET_IP_SEC_REMOTE_ADDRESS: &str = "RemoteAddress";
/// Wraps `kSCPropNetIPSecStatus`.
pub const SC_PROP_NET_IP_SEC_STATUS: &str = "Status";
/// Wraps `kSCPropNetIPSecXAuthEnabled`.
pub const SC_PROP_NET_IP_SEC_XAUTH_ENABLED: &str = "XAuthEnabled";
/// Wraps `kSCPropNetIPSecXAuthName`.
pub const SC_PROP_NET_IP_SEC_XAUTH_NAME: &str = "XAuthName";
/// Wraps `kSCPropNetIPSecXAuthPassword`.
pub const SC_PROP_NET_IP_SEC_XAUTH_PASSWORD: &str = "XAuthPassword";
/// Wraps `kSCPropNetIPSecXAuthPasswordEncryption`.
pub const SC_PROP_NET_IP_SEC_XAUTH_PASSWORD_ENCRYPTION: &str = "XAuthPasswordEncryption";

/// Wraps `kSCValNetIPSecAuthenticationMethodSharedSecret`.
pub const SC_VAL_NET_IP_SEC_AUTHENTICATION_METHOD_SHARED_SECRET: &str = "SharedSecret";
/// Wraps `kSCValNetIPSecAuthenticationMethodCertificate`.
pub const SC_VAL_NET_IP_SEC_AUTHENTICATION_METHOD_CERTIFICATE: &str = "Certificate";
/// Wraps `kSCValNetIPSecAuthenticationMethodHybrid`.
pub const SC_VAL_NET_IP_SEC_AUTHENTICATION_METHOD_HYBRID: &str = "Hybrid";
/// Wraps `kSCValNetIPSecLocalIdentifierTypeKeyID`.
pub const SC_VAL_NET_IP_SEC_LOCAL_IDENTIFIER_TYPE_KEY_ID: &str = "KeyID";
/// Wraps `kSCValNetIPSecSharedSecretEncryptionKeychain`.
pub const SC_VAL_NET_IP_SEC_SHARED_SECRET_ENCRYPTION_KEYCHAIN: &str = "Keychain";
/// Wraps `kSCValNetIPSecXAuthPasswordEncryptionKeychain`.
pub const SC_VAL_NET_IP_SEC_XAUTH_PASSWORD_ENCRYPTION_KEYCHAIN: &str = "Keychain";
/// Wraps `kSCValNetIPSecXAuthPasswordEncryptionPrompt`.
pub const SC_VAL_NET_IP_SEC_XAUTH_PASSWORD_ENCRYPTION_PROMPT: &str = "Prompt";

// IPv4
/// Wraps `kSCPropNetIPv4Addresses`.
pub const SC_PROP_NET_IPV4_ADDRESSES: &str = "Addresses";
/// Wraps `kSCPropNetIPv4ConfigMethod`.
pub const SC_PROP_NET_IPV4_CONFIG_METHOD: &str = "ConfigMethod";
/// Wraps `kSCPropNetIPv4DHCPClientID`.
pub const SC_PROP_NET_IPV4_DHCP_CLIENT_ID: &str = "DHCPClientID";
/// Wraps `kSCPropNetIPv4Router`.
pub const SC_PROP_NET_IPV4_ROUTER: &str = "Router";
/// Wraps `kSCPropNetIPv4SubnetMasks`.
pub const SC_PROP_NET_IPV4_SUBNET_MASKS: &str = "SubnetMasks";
/// Wraps `kSCPropNetIPv4DestAddresses`.
pub const SC_PROP_NET_IPV4_DEST_ADDRESSES: &str = "DestAddresses";
/// Wraps `kSCPropNetIPv4BroadcastAddresses`.
pub const SC_PROP_NET_IPV4_BROADCAST_ADDRESSES: &str = "BroadcastAddresses";

/// Wraps `kSCValNetIPv4ConfigMethodAutomatic`.
pub const SC_VAL_NET_IPV4_CONFIG_METHOD_AUTOMATIC: &str = "Automatic";
/// Wraps `kSCValNetIPv4ConfigMethodBootp`.
pub const SC_VAL_NET_IPV4_CONFIG_METHOD_BOOTP: &str = "BOOTP";
/// Wraps `kSCValNetIPv4ConfigMethodDHCP`.
pub const SC_VAL_NET_IPV4_CONFIG_METHOD_DHCP: &str = "DHCP";
/// Wraps `kSCValNetIPv4ConfigMethodInform`.
pub const SC_VAL_NET_IPV4_CONFIG_METHOD_INFORM: &str = "INFORM";
/// Wraps `kSCValNetIPv4ConfigMethodLinkLocal`.
pub const SC_VAL_NET_IPV4_CONFIG_METHOD_LINK_LOCAL: &str = "LinkLocal";
/// Wraps `kSCValNetIPv4ConfigMethodManual`.
pub const SC_VAL_NET_IPV4_CONFIG_METHOD_MANUAL: &str = "Manual";
/// Wraps `kSCValNetIPv4ConfigMethodPPP`.
pub const SC_VAL_NET_IPV4_CONFIG_METHOD_PPP: &str = "PPP";

// IPv6
/// Wraps `kSCPropNetIPv6Addresses`.
pub const SC_PROP_NET_IPV6_ADDRESSES: &str = "Addresses";
/// Wraps `kSCPropNetIPv6ConfigMethod`.
pub const SC_PROP_NET_IPV6_CONFIG_METHOD: &str = "ConfigMethod";
/// Wraps `kSCPropNetIPv6DestAddresses`.
pub const SC_PROP_NET_IPV6_DEST_ADDRESSES: &str = "DestAddresses";
/// Wraps `kSCPropNetIPv6Flags`.
pub const SC_PROP_NET_IPV6_FLAGS: &str = "Flags";
/// Wraps `kSCPropNetIPv6PrefixLength`.
pub const SC_PROP_NET_IPV6_PREFIX_LENGTH: &str = "PrefixLength";
/// Wraps `kSCPropNetIPv6Router`.
pub const SC_PROP_NET_IPV6_ROUTER: &str = "Router";

/// Wraps `kSCValNetIPv6ConfigMethodAutomatic`.
pub const SC_VAL_NET_IPV6_CONFIG_METHOD_AUTOMATIC: &str = "Automatic";
/// Wraps `kSCValNetIPv6ConfigMethodLinkLocal`.
pub const SC_VAL_NET_IPV6_CONFIG_METHOD_LINK_LOCAL: &str = "LinkLocal";
/// Wraps `kSCValNetIPv6ConfigMethodManual`.
pub const SC_VAL_NET_IPV6_CONFIG_METHOD_MANUAL: &str = "Manual";
/// Wraps `kSCValNetIPv6ConfigMethodRouterAdvertisement`.
pub const SC_VAL_NET_IPV6_CONFIG_METHOD_ROUTER_ADVERTISEMENT: &str = "RouterAdvertisement";
/// Wraps `kSCValNetIPv6ConfigMethod6to4`.
pub const SC_VAL_NET_IPV6_CONFIG_METHOD_6TO4: &str = "6to4";

// 6to4
/// Wraps `kSCPropNet6to4Relay`.
pub const SC_PROP_NET_6TO4_RELAY: &str = "Relay";

// Link
/// Wraps `kSCPropNetLinkActive`.
pub const SC_PROP_NET_LINK_ACTIVE: &str = "Active";
/// Wraps `kSCPropNetLinkDetaching`.
pub const SC_PROP_NET_LINK_DETACHING: &str = "Detaching";

// Modem
/// Wraps `kSCPropNetModemAccessPointName`.
pub const SC_PROP_NET_MODEM_ACCESS_POINT_NAME: &str = "AccessPointName";
/// Wraps `kSCPropNetModemConnectionPersonality`.
pub const SC_PROP_NET_MODEM_CONNECTION_PERSONALITY: &str = "ConnectionPersonality";
/// Wraps `kSCPropNetModemConnectionScript`.
pub const SC_PROP_NET_MODEM_CONNECTION_SCRIPT: &str = "ConnectionScript";
/// Wraps `kSCPropNetModemConnectSpeed`.
pub const SC_PROP_NET_MODEM_CONNECT_SPEED: &str = "ConnectSpeed";
/// Wraps `kSCPropNetModemDataCompression`.
pub const SC_PROP_NET_MODEM_DATA_COMPRESSION: &str = "DataCompression";
/// Wraps `kSCPropNetModemDeviceContextID`.
pub const SC_PROP_NET_MODEM_DEVICE_CONTEXT_ID: &str = "DeviceContextID";
/// Wraps `kSCPropNetModemDeviceModel`.
pub const SC_PROP_NET_MODEM_DEVICE_MODEL: &str = "DeviceModel";
/// Wraps `kSCPropNetModemDeviceVendor`.
pub const SC_PROP_NET_MODEM_DEVICE_VENDOR: &str = "DeviceVendor";
/// Wraps `kSCPropNetModemDialMode`.
pub const SC_PROP_NET_MODEM_DIAL_MODE: &str = "DialMode";
/// Wraps `kSCPropNetModemErrorCorrection`.
pub const SC_PROP_NET_MODEM_ERROR_CORRECTION: &str = "ErrorCorrection";
/// Wraps `kSCPropNetModemHoldCallWaitingAudibleAlert`.
pub const SC_PROP_NET_MODEM_HOLD_CALL_WAITING_AUDIBLE_ALERT: &str = "HoldCallWaitingAudibleAlert";
/// Wraps `kSCPropNetModemHoldDisconnectOnAnswer`.
pub const SC_PROP_NET_MODEM_HOLD_DISCONNECT_ON_ANSWER: &str = "HoldDisconnectOnAnswer";
/// Wraps `kSCPropNetModemHoldEnabled`.
pub const SC_PROP_NET_MODEM_HOLD_ENABLED: &str = "HoldEnabled";
/// Wraps `kSCPropNetModemHoldReminder`.
pub const SC_PROP_NET_MODEM_HOLD_REMINDER: &str = "HoldReminder";
/// Wraps `kSCPropNetModemHoldReminderTime`.
pub const SC_PROP_NET_MODEM_HOLD_REMINDER_TIME: &str = "HoldReminderTime";
/// Wraps `kSCPropNetModemNote`.
pub const SC_PROP_NET_MODEM_NOTE: &str = "Note";
/// Wraps `kSCPropNetModemPulseDial`.
pub const SC_PROP_NET_MODEM_PULSE_DIAL: &str = "PulseDial";
/// Wraps `kSCPropNetModemSpeaker`.
pub const SC_PROP_NET_MODEM_SPEAKER: &str = "Speaker";
/// Wraps `kSCPropNetModemSpeed`.
pub const SC_PROP_NET_MODEM_SPEED: &str = "Speed";

/// Wraps `kSCValNetModemDialModeIgnoreDialTone`.
pub const SC_VAL_NET_MODEM_DIAL_MODE_IGNORE_DIAL_TONE: &str = "IgnoreDialTone";
/// Wraps `kSCValNetModemDialModeManual`.
pub const SC_VAL_NET_MODEM_DIAL_MODE_MANUAL: &str = "Manual";
/// Wraps `kSCValNetModemDialModeWaitForDialTone`.
pub const SC_VAL_NET_MODEM_DIAL_MODE_WAIT_FOR_DIAL_TONE: &str = "WaitForDialTone";

// PPP
/// Wraps `kSCPropNetPPPACSPEnabled`.
pub const SC_PROP_NET_PPP_ACSP_ENABLED: &str = "ACSPEnabled";
/// Wraps `kSCPropNetPPPConnectTime`.
pub const SC_PROP_NET_PPP_CONNECT_TIME: &str = "ConnectTime";
/// Wraps `kSCPropNetPPPDeviceLastCause`.
pub const SC_PROP_NET_PPP_DEVICE_LAST_CAUSE: &str = "DeviceLastCause";
/// Wraps `kSCPropNetPPPDialOnDemand`.
pub const SC_PROP_NET_PPP_DIAL_ON_DEMAND: &str = "DialOnDemand";
/// Wraps `kSCPropNetPPPDisconnectOnFastUserSwitch`.
pub const SC_PROP_NET_PPP_DISCONNECT_ON_FAST_USER_SWITCH: &str = "DisconnectOnFastUserSwitch";
/// Wraps `kSCPropNetPPPDisconnectOnIdle`.
pub const SC_PROP_NET_PPP_DISCONNECT_ON_IDLE: &str = "DisconnectOnIdle";
/// Wraps `kSCPropNetPPPDisconnectOnIdleTimer`.
pub const SC_PROP_NET_PPP_DISCONNECT_ON_IDLE_TIMER: &str = "DisconnectOnIdleTimer";
/// Wraps `kSCPropNetPPPDisconnectOnLogout`.
pub const SC_PROP_NET_PPP_DISCONNECT_ON_LOGOUT: &str = "DisconnectOnLogout";
/// Wraps `kSCPropNetPPPDisconnectOnSleep`.
pub const SC_PROP_NET_PPP_DISCONNECT_ON_SLEEP: &str = "DisconnectOnSleep";
/// Wraps `kSCPropNetPPPDisconnectTime`.
pub const SC_PROP_NET_PPP_DISCONNECT_TIME: &str = "DisconnectTime";
/// Wraps `kSCPropNetPPPIdleReminder`.
pub const SC_PROP_NET_PPP_IDLE_REMINDER: &str = "IdleReminder";
/// Wraps `kSCPropNetPPPIdleReminderTimer`.
pub const SC_PROP_NET_PPP_IDLE_REMINDER_TIMER: &str = "IdleReminderTimer";
/// Wraps `kSCPropNetPPPLastCause`.
pub const SC_PROP_NET_PPP_LAST_CAUSE: &str = "LastCause";
/// Wraps `kSCPropNetPPPLogfile`.
pub const SC_PROP_NET_PPP_LOGFILE: &str = "Logfile";
/// Wraps `kSCPropNetPPPRetryConnectTime`.
pub const SC_PROP_NET_PPP_RETRY_CONNECT_TIME: &str = "RetryConnectTime";
/// Wraps `kSCPropNetPPPSessionTimer`.
pub const SC_PROP_NET_PPP_SESSION_TIMER: &str = "SessionTimer";
/// Wraps `kSCPropNetPPPStatus`.
pub const SC_PROP_NET_PPP_STATUS: &str = "Status";
/// Wraps `kSCPropNetPPPUseSessionTimer`.
pub const SC_PROP_NET_PPP_USE_SESSION_TIMER: &str = "UseSessionTimer";
/// Wraps `kSCPropNetPPPVerboseLogging`.
pub const SC_PROP_NET_PPP_VERBOSE_LOGGING: &str = "VerboseLogging";

// PPP Auth
/// Wraps `kSCPropNetPPPAuthEAPPlugins`.
pub const SC_PROP_NET_PPP_AUTH_EAP_PLUGINS: &str = "AuthEAPPlugins";
/// Wraps `kSCPropNetPPPAuthName`.
pub const SC_PROP_NET_PPP_AUTH_NAME: &str = "AuthName";
/// Wraps `kSCPropNetPPPAuthPassword`.
pub const SC_PROP_NET_PPP_AUTH_PASSWORD: &str = "AuthPassword";
/// Wraps `kSCPropNetPPPAuthPasswordEncryption`.
pub const SC_PROP_NET_PPP_AUTH_PASSWORD_ENCRYPTION: &str = "AuthPasswordEncryption";
/// Wraps `kSCPropNetPPPAuthPrompt`.
pub const SC_PROP_NET_PPP_AUTH_PROMPT: &str = "AuthPrompt";
/// Wraps `kSCPropNetPPPAuthProtocol`.
pub const SC_PROP_NET_PPP_AUTH_PROTOCOL: &str = "AuthProtocol";

/// Wraps `kSCValNetPPPAuthPasswordEncryptionKeychain`.
pub const SC_VAL_NET_PPP_AUTH_PASSWORD_ENCRYPTION_KEYCHAIN: &str = "Keychain";
/// Wraps `kSCValNetPPPAuthPasswordEncryptionToken`.
pub const SC_VAL_NET_PPP_AUTH_PASSWORD_ENCRYPTION_TOKEN: &str = "Token";
/// Wraps `kSCValNetPPPAuthPromptBefore`.
pub const SC_VAL_NET_PPP_AUTH_PROMPT_BEFORE: &str = "Before";
/// Wraps `kSCValNetPPPAuthPromptAfter`.
pub const SC_VAL_NET_PPP_AUTH_PROMPT_AFTER: &str = "After";
/// Wraps `kSCValNetPPPAuthProtocolChap`.
pub const SC_VAL_NET_PPP_AUTH_PROTOCOL_CHAP: &str = "CHAP";
/// Wraps `kSCValNetPPPAuthProtocolEAP`.
pub const SC_VAL_NET_PPP_AUTH_PROTOCOL_EAP: &str = "EAP";
/// Wraps `kSCValNetPPPAuthProtocolMschap1`.
pub const SC_VAL_NET_PPP_AUTH_PROTOCOL_MSCHAP1: &str = "MSCHAP1";
/// Wraps `kSCValNetPPPAuthProtocolMschap2`.
pub const SC_VAL_NET_PPP_AUTH_PROTOCOL_MSCHAP2: &str = "MSCHAP2";
/// Wraps `kSCValNetPPPAuthProtocolPap`.
pub const SC_VAL_NET_PPP_AUTH_PROTOCOL_PAP: &str = "PAP";

// PPP Comm
/// Wraps `kSCPropNetPPPCommAlternateRemoteAddress`.
pub const SC_PROP_NET_PPP_COMM_ALTERNATE_REMOTE_ADDRESS: &str = "CommAlternateRemoteAddress";
/// Wraps `kSCPropNetPPPCommConnectDelay`.
pub const SC_PROP_NET_PPP_COMM_CONNECT_DELAY: &str = "CommConnectDelay";
/// Wraps `kSCPropNetPPPCommDisplayTerminalWindow`.
pub const SC_PROP_NET_PPP_COMM_DISPLAY_TERMINAL_WINDOW: &str = "CommDisplayTerminalWindow";
/// Wraps `kSCPropNetPPPCommRedialCount`.
pub const SC_PROP_NET_PPP_COMM_REDIAL_COUNT: &str = "CommRedialCount";
/// Wraps `kSCPropNetPPPCommRedialEnabled`.
pub const SC_PROP_NET_PPP_COMM_REDIAL_ENABLED: &str = "CommRedialEnabled";
/// Wraps `kSCPropNetPPPCommRedialInterval`.
pub const SC_PROP_NET_PPP_COMM_REDIAL_INTERVAL: &str = "CommRedialInterval";
/// Wraps `kSCPropNetPPPCommRemoteAddress`.
pub const SC_PROP_NET_PPP_COMM_REMOTE_ADDRESS: &str = "CommRemoteAddress";
/// Wraps `kSCPropNetPPPCommTerminalScript`.
pub const SC_PROP_NET_PPP_COMM_TERMINAL_SCRIPT: &str = "CommTerminalScript";
/// Wraps `kSCPropNetPPPCommUseTerminalScript`.
pub const SC_PROP_NET_PPP_COMM_USE_TERMINAL_SCRIPT: &str = "CommUseTerminalScript";

// PPP CCP
/// Wraps `kSCPropNetPPPCcpEnabled`.
pub const SC_PROP_NET_PPP_CCP_ENABLED: &str = "CCPEnabled";
/// Wraps `kSCPropNetPPPCcpMppe40Enabled`.
pub const SC_PROP_NET_PPP_CCP_MPPE40_ENABLED: &str = "CCPMPPE40Enabled";
/// Wraps `kSCPropNetPPPCcpMppe128Enabled`.
pub const SC_PROP_NET_PPP_CCP_MPPE128_ENABLED: &str = "CCPMPPE128Enabled";

// PPP IPCP
/// Wraps `kSCPropNetPPPIpcpCompressionVj`.
pub const SC_PROP_NET_PPP_IPCP_COMPRESSION_VJ: &str = "IPCPCompressionVJ";
/// Wraps `kSCPropNetPPPIpcpUsePeerDNS`.
pub const SC_PROP_NET_PPP_IPCP_USE_PEER_DNS: &str = "IPCPUsePeerDNS";

// PPP LCP
/// Wraps `kSCPropNetPPPLcpCompressionAcField`.
pub const SC_PROP_NET_PPP_LCP_COMPRESSION_AC_FIELD: &str = "LCPCompressionACField";
/// Wraps `kSCPropNetPPPLcpCompressionPField`.
pub const SC_PROP_NET_PPP_LCP_COMPRESSION_P_FIELD: &str = "LCPCompressionPField";
/// Wraps `kSCPropNetPPPLcpEchoEnabled`.
pub const SC_PROP_NET_PPP_LCP_ECHO_ENABLED: &str = "LCPEchoEnabled";
/// Wraps `kSCPropNetPPPLcpEchoFailure`.
pub const SC_PROP_NET_PPP_LCP_ECHO_FAILURE: &str = "LCPEchoFailure";
/// Wraps `kSCPropNetPPPLcpEchoInterval`.
pub const SC_PROP_NET_PPP_LCP_ECHO_INTERVAL: &str = "LCPEchoInterval";
/// Wraps `kSCPropNetPPPLcpMru`.
pub const SC_PROP_NET_PPP_LCP_MRU: &str = "LCPMRU";
/// Wraps `kSCPropNetPPPLcpMTU`.
pub const SC_PROP_NET_PPP_LCP_MTU: &str = "LCPMTU";
/// Wraps `kSCPropNetPPPLcpReceiveAccm`.
pub const SC_PROP_NET_PPP_LCP_RECEIVE_ACCM: &str = "LCPReceiveACCM";
/// Wraps `kSCPropNetPPPLcpTransmitAccm`.
pub const SC_PROP_NET_PPP_LCP_TRANSMIT_ACCM: &str = "LCPTransmitACCM";

// PPP plugins
/// Wraps `kSCPropNetPPPPlugins`.
pub const SC_PROP_NET_PPP_PLUGINS: &str = "Plugins";

// Proxies
/// Wraps `kSCPropNetProxiesExceptionList`.
pub const SC_PROP_NET_PROXIES_EXCEPTION_LIST: &str = "ExceptionsList";
/// Wraps `kSCPropNetProxiesExcludeSimpleHostnames`.
pub const SC_PROP_NET_PROXIES_EXCLUDE_SIMPLE_HOSTNAMES: &str = "ExcludeSimpleHostnames";
/// Wraps `kSCPropNetProxiesFtpEnable`.
pub const SC_PROP_NET_PROXIES_FTP_ENABLE: &str = "FTPEnable";
/// Wraps `kSCPropNetProxiesFtpPassive`.
pub const SC_PROP_NET_PROXIES_FTP_PASSIVE: &str = "FTPPassive";
/// Wraps `kSCPropNetProxiesFtpPort`.
pub const SC_PROP_NET_PROXIES_FTP_PORT: &str = "FTPPort";
/// Wraps `kSCPropNetProxiesFtpProxy`.
pub const SC_PROP_NET_PROXIES_FTP_PROXY: &str = "FTPProxy";
/// Wraps `kSCPropNetProxiesFtpUser`.
pub const SC_PROP_NET_PROXIES_FTP_USER: &str = "FTPUser";
/// Wraps `kSCPropNetProxiesGopherEnable`.
pub const SC_PROP_NET_PROXIES_GOPHER_ENABLE: &str = "GopherEnable";
/// Wraps `kSCPropNetProxiesGopherPort`.
pub const SC_PROP_NET_PROXIES_GOPHER_PORT: &str = "GopherPort";
/// Wraps `kSCPropNetProxiesGopherProxy`.
pub const SC_PROP_NET_PROXIES_GOPHER_PROXY: &str = "GopherProxy";
/// Wraps `kSCPropNetProxiesGopherUser`.
pub const SC_PROP_NET_PROXIES_GOPHER_USER: &str = "GopherUser";
/// Wraps `kSCPropNetProxiesHttpEnable`.
pub const SC_PROP_NET_PROXIES_HTTP_ENABLE: &str = "HTTPEnable";
/// Wraps `kSCPropNetProxiesHttpPort`.
pub const SC_PROP_NET_PROXIES_HTTP_PORT: &str = "HTTPPort";
/// Wraps `kSCPropNetProxiesHttpProxy`.
pub const SC_PROP_NET_PROXIES_HTTP_PROXY: &str = "HTTPProxy";
/// Wraps `kSCPropNetProxiesHttpsEnable`.
pub const SC_PROP_NET_PROXIES_HTTPS_ENABLE: &str = "HTTPSEnable";
/// Wraps `kSCPropNetProxiesHttpsPort`.
pub const SC_PROP_NET_PROXIES_HTTPS_PORT: &str = "HTTPSPort";
/// Wraps `kSCPropNetProxiesHttpsProxy`.
pub const SC_PROP_NET_PROXIES_HTTPS_PROXY: &str = "HTTPSProxy";
/// Wraps `kSCPropNetProxiesHttpsUser`.
pub const SC_PROP_NET_PROXIES_HTTPS_USER: &str = "HTTPSUser";
/// Wraps `kSCPropNetProxiesHttpUser`.
pub const SC_PROP_NET_PROXIES_HTTP_USER: &str = "HTTPUser";
/// Wraps `kSCPropNetProxiesProxyAutoConfigEnable`.
pub const SC_PROP_NET_PROXIES_PROXY_AUTO_CONFIG_ENABLE: &str = "ProxyAutoConfigEnable";
/// Wraps `kSCPropNetProxiesProxyAutoConfigJavascript`.
pub const SC_PROP_NET_PROXIES_PROXY_AUTO_CONFIG_JAVASCRIPT: &str = "ProxyAutoConfigJavaScript";
/// Wraps `kSCPropNetProxiesProxyAutoConfigUrlString`.
pub const SC_PROP_NET_PROXIES_PROXY_AUTO_CONFIG_URL_STRING: &str = "ProxyAutoConfigURLString";
/// Wraps `kSCPropNetProxiesProxyAutoDiscoveryEnable`.
pub const SC_PROP_NET_PROXIES_PROXY_AUTO_DISCOVERY_ENABLE: &str = "ProxyAutoDiscoveryEnable";
/// Wraps `kSCPropNetProxiesRtspEnable`.
pub const SC_PROP_NET_PROXIES_RTSP_ENABLE: &str = "RTSPEnable";
/// Wraps `kSCPropNetProxiesRtspPort`.
pub const SC_PROP_NET_PROXIES_RTSP_PORT: &str = "RTSPPort";
/// Wraps `kSCPropNetProxiesRtspProxy`.
pub const SC_PROP_NET_PROXIES_RTSP_PROXY: &str = "RTSPProxy";
/// Wraps `kSCPropNetProxiesRtspUser`.
pub const SC_PROP_NET_PROXIES_RTSP_USER: &str = "RTSPUser";
/// Wraps `kSCPropNetProxiesSocksEnable`.
pub const SC_PROP_NET_PROXIES_SOCKS_ENABLE: &str = "SOCKSEnable";
/// Wraps `kSCPropNetProxiesSocksPort`.
pub const SC_PROP_NET_PROXIES_SOCKS_PORT: &str = "SOCKSPort";
/// Wraps `kSCPropNetProxiesSocksProxy`.
pub const SC_PROP_NET_PROXIES_SOCKS_PROXY: &str = "SOCKSProxy";
/// Wraps `kSCPropNetProxiesSocksUser`.
pub const SC_PROP_NET_PROXIES_SOCKS_USER: &str = "SOCKSUser";

// SMB
/// Wraps `kSCPropNetSMBNetBiosName`.
pub const SC_PROP_NET_SMB_NET_BIOS_NAME: &str = "NetBIOSName";
/// Wraps `kSCPropNetSMBNetBiosNodeType`.
pub const SC_PROP_NET_SMB_NET_BIOS_NODE_TYPE: &str = "NetBIOSNodeType";
/// Wraps `kSCPropNetSMBNetBiosScope`.
pub const SC_PROP_NET_SMB_NET_BIOS_SCOPE: &str = "NetBIOSScope";
/// Wraps `kSCPropNetSMBWinsAddresses`.
pub const SC_PROP_NET_SMB_WINS_ADDRESSES: &str = "WINSAddresses";
/// Wraps `kSCPropNetSMBWorkgroup`.
pub const SC_PROP_NET_SMB_WORKGROUP: &str = "Workgroup";

/// Wraps `kSCValNetSMBNetBiosNodeTypeBroadcast`.
pub const SC_VAL_NET_SMB_NET_BIOS_NODE_TYPE_BROADCAST: &str = "Broadcast";
/// Wraps `kSCValNetSMBNetBiosNodeTypePeer`.
pub const SC_VAL_NET_SMB_NET_BIOS_NODE_TYPE_PEER: &str = "Peer";
/// Wraps `kSCValNetSMBNetBiosNodeTypeMixed`.
pub const SC_VAL_NET_SMB_NET_BIOS_NODE_TYPE_MIXED: &str = "Mixed";
/// Wraps `kSCValNetSMBNetBiosNodeTypeHybrid`.
pub const SC_VAL_NET_SMB_NET_BIOS_NODE_TYPE_HYBRID: &str = "Hybrid";

// L2TP
/// Wraps `kSCPropNetL2TPIPSecSharedSecret`.
pub const SC_PROP_NET_L2TP_IP_SEC_SHARED_SECRET: &str = "IPSecSharedSecret";
/// Wraps `kSCPropNetL2TPIPSecSharedSecretEncryption`.
pub const SC_PROP_NET_L2TP_IP_SEC_SHARED_SECRET_ENCRYPTION: &str = "IPSecSharedSecretEncryption";
/// Wraps `kSCPropNetL2TPTransport`.
pub const SC_PROP_NET_L2TP_TRANSPORT: &str = "Transport";

/// Wraps `kSCValNetL2TPIPSecSharedSecretEncryptionKeychain`.
pub const SC_VAL_NET_L2TP_IP_SEC_SHARED_SECRET_ENCRYPTION_KEYCHAIN: &str = "Keychain";
/// Wraps `kSCValNetL2TPTransportIP`.
pub const SC_VAL_NET_L2TP_TRANSPORT_IP: &str = "IP";
/// Wraps `kSCValNetL2TPTransportIPSec`.
pub const SC_VAL_NET_L2TP_TRANSPORT_IP_SEC: &str = "IPSec";

// AirPort
/// Wraps `kSCPropNetAirPortAllowNetCreation`.
pub const SC_PROP_NET_AIR_PORT_ALLOW_NET_CREATION: &str = "AllowNetCreation";
/// Wraps `kSCPropNetAirPortAuthPassword`.
pub const SC_PROP_NET_AIR_PORT_AUTH_PASSWORD: &str = "AuthPassword";
/// Wraps `kSCPropNetAirPortAuthPasswordEncryption`.
pub const SC_PROP_NET_AIR_PORT_AUTH_PASSWORD_ENCRYPTION: &str = "AuthPasswordEncryption";
/// Wraps `kSCPropNetAirPortJoinMode`.
pub const SC_PROP_NET_AIR_PORT_JOIN_MODE: &str = "JoinMode";
/// Wraps `kSCPropNetAirPortPowerEnabled`.
pub const SC_PROP_NET_AIR_PORT_POWER_ENABLED: &str = "PowerEnabled";
/// Wraps `kSCPropNetAirPortPreferredNetwork`.
pub const SC_PROP_NET_AIR_PORT_PREFERRED_NETWORK: &str = "PreferredNetwork";
/// Wraps `kSCPropNetAirPortSavePasswords`.
pub const SC_PROP_NET_AIR_PORT_SAVE_PASSWORDS: &str = "SavePasswords";

// System
/// Wraps `kSCPropSystemComputerName`.
pub const SC_PROP_SYSTEM_COMPUTER_NAME: &str = "ComputerName";
/// Wraps `kSCPropSystemComputerNameEncoding`.
pub const SC_PROP_SYSTEM_COMPUTER_NAME_ENCODING: &str = "ComputerNameEncoding";

// Users/ConsoleUser
/// Wraps `kSCPropUsersConsoleUserName`.
pub const SC_PROP_USERS_CONSOLE_USER_NAME: &str = "Name";
/// Wraps `kSCPropUsersConsoleUserUID`.
pub const SC_PROP_USERS_CONSOLE_USER_UID: &str = "UID";
/// Wraps `kSCPropUsersConsoleUserGID`.
pub const SC_PROP_USERS_CONSOLE_USER_GID: &str = "GID";

// Dynamic Store
/// Wraps `kSCDynamicStoreDomainFile`.
pub const SC_DYNAMIC_STORE_DOMAIN_FILE: &str = "File";
/// Wraps `kSCDynamicStoreDomainPlugin`.
pub const SC_DYNAMIC_STORE_DOMAIN_PLUGIN: &str = "Plugin";
/// Wraps `kSCDynamicStoreDomainPrefs`.
pub const SC_DYNAMIC_STORE_DOMAIN_PREFS: &str = "Prefs";
/// Wraps `kSCDynamicStoreDomainSetup`.
pub const SC_DYNAMIC_STORE_DOMAIN_SETUP: &str = "Setup";
/// Wraps `kSCDynamicStoreDomainState`.
pub const SC_DYNAMIC_STORE_DOMAIN_STATE: &str = "State";
/// Wraps `kSCDynamicStorePropNetInterfaces`.
pub const SC_DYNAMIC_STORE_PROP_NET_INTERFACES: &str = "Interfaces";
/// Wraps `kSCDynamicStorePropNetPrimaryInterface`.
pub const SC_DYNAMIC_STORE_PROP_NET_PRIMARY_INTERFACE: &str = "PrimaryInterface";
/// Wraps `kSCDynamicStorePropNetPrimaryService`.
pub const SC_DYNAMIC_STORE_PROP_NET_PRIMARY_SERVICE: &str = "PrimaryService";
/// Wraps `kSCDynamicStorePropNetServiceIds`.
pub const SC_DYNAMIC_STORE_PROP_NET_SERVICE_IDS: &str = "ServiceIDs";
/// Wraps `kSCDynamicStorePropSetupCurrentSet`.
pub const SC_DYNAMIC_STORE_PROP_SETUP_CURRENT_SET: &str = "CurrentSet";
/// Wraps `kSCDynamicStorePropSetupLastUpdated`.
pub const SC_DYNAMIC_STORE_PROP_SETUP_LAST_UPDATED: &str = "LastUpdated";
/// Wraps `kSCDynamicStoreUseSessionKeys`.
pub const SC_DYNAMIC_STORE_USE_SESSION_KEYS: &str = "UseSessionKeys";

// Network Interface Type constants (SCNetworkConfiguration.h)
/// Wraps `kSCNetworkInterfaceType6to4`.
pub const SC_NETWORK_INTERFACE_TYPE_6TO4: &str = "6to4";
/// Wraps `kSCNetworkInterfaceTypeBluetooth`.
pub const SC_NETWORK_INTERFACE_TYPE_BLUETOOTH: &str = "Bluetooth";
/// Wraps `kSCNetworkInterfaceTypeBond`.
pub const SC_NETWORK_INTERFACE_TYPE_BOND: &str = "Bond";
/// Wraps `kSCNetworkInterfaceTypeEthernet`.
pub const SC_NETWORK_INTERFACE_TYPE_ETHERNET: &str = "Ethernet";
/// Wraps `kSCNetworkInterfaceTypeFireWire`.
pub const SC_NETWORK_INTERFACE_TYPE_FIRE_WIRE: &str = "FireWire";
/// Wraps `kSCNetworkInterfaceTypeIeee80211`.
pub const SC_NETWORK_INTERFACE_TYPE_IEEE80211: &str = "IEEE80211";
/// Wraps `kSCNetworkInterfaceTypeIPSec`.
pub const SC_NETWORK_INTERFACE_TYPE_IP_SEC: &str = "IPSec";
/// Wraps `kSCNetworkInterfaceTypeIPv4`.
pub const SC_NETWORK_INTERFACE_TYPE_IPV4: &str = "IPv4";
/// Wraps `kSCNetworkInterfaceTypeL2TP`.
pub const SC_NETWORK_INTERFACE_TYPE_L2TP: &str = "L2TP";
/// Wraps `kSCNetworkInterfaceTypeModem`.
pub const SC_NETWORK_INTERFACE_TYPE_MODEM: &str = "Modem";
/// Wraps `kSCNetworkInterfaceTypePPP`.
pub const SC_NETWORK_INTERFACE_TYPE_PPP: &str = "PPP";
/// Wraps `kSCNetworkInterfaceTypeSerial`.
pub const SC_NETWORK_INTERFACE_TYPE_SERIAL: &str = "Serial";
/// Wraps `kSCNetworkInterfaceTypeVLAN`.
pub const SC_NETWORK_INTERFACE_TYPE_VLAN: &str = "VLAN";
/// Wraps `kSCNetworkInterfaceTypeWWAN`.
pub const SC_NETWORK_INTERFACE_TYPE_WWAN: &str = "WWAN";
/// Wraps `kSCNetworkInterfaceTypeXxx`.
pub const SC_NETWORK_INTERFACE_TYPE_XXX: &str = "XXX";
// The loopback-like IPv4 virtual interface
/// Wraps `kSCNetworkInterfaceIPv4`.
pub const SC_NETWORK_INTERFACE_IPV4: &str = "lo0";

// Network Protocol Type constants
/// Wraps `kSCNetworkProtocolTypeDNS`.
pub const SC_NETWORK_PROTOCOL_TYPE_DNS: &str = "DNS";
/// Wraps `kSCNetworkProtocolTypeIPv4`.
pub const SC_NETWORK_PROTOCOL_TYPE_IPV4: &str = "IPv4";
/// Wraps `kSCNetworkProtocolTypeIPv6`.
pub const SC_NETWORK_PROTOCOL_TYPE_IPV6: &str = "IPv6";
/// Wraps `kSCNetworkProtocolTypeProxies`.
pub const SC_NETWORK_PROTOCOL_TYPE_PROXIES: &str = "Proxies";
/// Wraps `kSCNetworkProtocolTypeSMB`.
pub const SC_NETWORK_PROTOCOL_TYPE_SMB: &str = "SMB";
/// Wraps `kSCNetworkProtocolTypeXxx`.
pub const SC_NETWORK_PROTOCOL_TYPE_XXX: &str = "XXX";

// Legacy SCNetworkFlags (deprecated since 10.6 — wrap for completeness)
/// Wraps `kSCNetworkFlagsTransientConnection`.
pub const SC_NETWORK_FLAGS_TRANSIENT_CONNECTION: u32 = 1 << 0;
/// Wraps `kSCNetworkFlagsReachable`.
pub const SC_NETWORK_FLAGS_REACHABLE: u32 = 1 << 1;
/// Wraps `kSCNetworkFlagsConnectionRequired`.
pub const SC_NETWORK_FLAGS_CONNECTION_REQUIRED: u32 = 1 << 2;
/// Wraps `kSCNetworkFlagsConnectionAutomatic`.
pub const SC_NETWORK_FLAGS_CONNECTION_AUTOMATIC: u32 = 1 << 3;
/// Wraps `kSCNetworkFlagsInterventionRequired`.
pub const SC_NETWORK_FLAGS_INTERVENTION_REQUIRED: u32 = 1 << 4;
/// Wraps `kSCNetworkFlagsIsLocalAddress`.
pub const SC_NETWORK_FLAGS_IS_LOCAL_ADDRESS: u32 = 1 << 16;
/// Wraps `kSCNetworkFlagsIsDirect`.
pub const SC_NETWORK_FLAGS_IS_DIRECT: u32 = 1 << 17;

/// Wraps `kSCNetworkReachabilityFlagsConnectionAutomatic`.
pub const SC_NETWORK_REACHABILITY_FLAGS_CONNECTION_AUTOMATIC: u32 = 1 << 3;

/// Wraps `kSCPreferencesNotificationCommit`.
pub const SC_PREFERENCES_NOTIFICATION_COMMIT: u32 = 1 << 0;
/// Wraps `kSCPreferencesNotificationApply`.
pub const SC_PREFERENCES_NOTIFICATION_APPLY: u32 = 1 << 1;

/// Wraps `kSCBondStatusOk`.
pub const SC_BOND_STATUS_OK: i32 = 0;
/// Wraps `kSCBondStatusLinkInvalid`.
pub const SC_BOND_STATUS_LINK_INVALID: i32 = 1;
/// Wraps `kSCBondStatusNoPartner`.
pub const SC_BOND_STATUS_NO_PARTNER: i32 = 2;
/// Wraps `kSCBondStatusNotInActiveGroup`.
pub const SC_BOND_STATUS_NOT_IN_ACTIVE_GROUP: i32 = 3;
/// Wraps `kSCBondStatusUnknown`.
pub const SC_BOND_STATUS_UNKNOWN: i32 = 999;

/// Wraps `kSCStatusOk`.
pub const SC_STATUS_OK: i32 = 0;
/// Wraps `kSCStatusFailed`.
pub const SC_STATUS_FAILED: i32 = 1001;
/// Wraps `kSCStatusInvalidArgument`.
pub const SC_STATUS_INVALID_ARGUMENT: i32 = 1002;
/// Wraps `kSCStatusAccessError`.
pub const SC_STATUS_ACCESS_ERROR: i32 = 1003;
/// Wraps `kSCStatusNoKey`.
pub const SC_STATUS_NO_KEY: i32 = 1004;
/// Wraps `kSCStatusKeyExists`.
pub const SC_STATUS_KEY_EXISTS: i32 = 1005;
/// Wraps `kSCStatusLocked`.
pub const SC_STATUS_LOCKED: i32 = 1006;
/// Wraps `kSCStatusNeedLock`.
pub const SC_STATUS_NEED_LOCK: i32 = 1007;
/// Wraps `kSCStatusNoStoreSession`.
pub const SC_STATUS_NO_STORE_SESSION: i32 = 2001;
/// Wraps `kSCStatusNoStoreServer`.
pub const SC_STATUS_NO_STORE_SERVER: i32 = 2002;
/// Wraps `kSCStatusNotifierActive`.
pub const SC_STATUS_NOTIFIER_ACTIVE: i32 = 2003;
/// Wraps `kSCStatusNoPrefsSession`.
pub const SC_STATUS_NO_PREFS_SESSION: i32 = 3001;
/// Wraps `kSCStatusPrefsBusy`.
pub const SC_STATUS_PREFS_BUSY: i32 = 3002;
/// Wraps `kSCStatusNoConfigFile`.
pub const SC_STATUS_NO_CONFIG_FILE: i32 = 3003;
/// Wraps `kSCStatusNoLink`.
pub const SC_STATUS_NO_LINK: i32 = 3004;
/// Wraps `kSCStatusStale`.
pub const SC_STATUS_STALE: i32 = 3005;
/// Wraps `kSCStatusMaxLink`.
pub const SC_STATUS_MAX_LINK: i32 = 3006;
/// Wraps `kSCStatusReachabilityUnknown`.
pub const SC_STATUS_REACHABILITY_UNKNOWN: i32 = 4001;
/// Wraps `kSCStatusConnectionNoService`.
pub const SC_STATUS_CONNECTION_NO_SERVICE: i32 = 5001;
/// Wraps `kSCStatusConnectionIgnore`.
pub const SC_STATUS_CONNECTION_IGNORE: i32 = 5002;

/// Wraps `CN_NETWORK_INFO_KEY_BSSID`.
pub const CN_NETWORK_INFO_KEY_BSSID: &str = "BSSID";
/// Wraps `CN_NETWORK_INFO_KEY_SSID`.
pub const CN_NETWORK_INFO_KEY_SSID: &str = "SSID";
/// Wraps `CN_NETWORK_INFO_KEY_SSID_DATA`.
pub const CN_NETWORK_INFO_KEY_SSID_DATA: &str = "SSIDDATA";

/// Wraps `kSCNetworkConnectionBytesIn`.
pub const SC_NETWORK_CONNECTION_BYTES_IN: &str = "BytesIn";
/// Wraps `kSCNetworkConnectionBytesOut`.
pub const SC_NETWORK_CONNECTION_BYTES_OUT: &str = "BytesOut";
/// Wraps `kSCNetworkConnectionPacketsIn`.
pub const SC_NETWORK_CONNECTION_PACKETS_IN: &str = "PacketsIn";
/// Wraps `kSCNetworkConnectionPacketsOut`.
pub const SC_NETWORK_CONNECTION_PACKETS_OUT: &str = "PacketsOut";
/// Wraps `kSCNetworkConnectionErrorsIn`.
pub const SC_NETWORK_CONNECTION_ERRORS_IN: &str = "ErrorsIn";
/// Wraps `kSCNetworkConnectionErrorsOut`.
pub const SC_NETWORK_CONNECTION_ERRORS_OUT: &str = "ErrorsOut";
/// Wraps `kSCNetworkConnectionSelectionOptionOnDemandHostName`.
pub const SC_NETWORK_CONNECTION_SELECTION_OPTION_ON_DEMAND_HOST_NAME: &str = "OnDemandHostName";
/// Wraps `kSCNetworkConnectionSelectionOptionOnDemandRetry`.
pub const SC_NETWORK_CONNECTION_SELECTION_OPTION_ON_DEMAND_RETRY: &str = "OnDemandRetry";
/// Wraps `kSCNetworkConnectionDisconnected`.
pub const SC_NETWORK_CONNECTION_DISCONNECTED: &str = "Disconnected";
/// Wraps `kSCNetworkConnectionConnecting`.
pub const SC_NETWORK_CONNECTION_CONNECTING: &str = "Connecting";
/// Wraps `kSCNetworkConnectionConnected`.
pub const SC_NETWORK_CONNECTION_CONNECTED: &str = "Connected";
/// Wraps `kSCNetworkConnectionDisconnecting`.
pub const SC_NETWORK_CONNECTION_DISCONNECTING: &str = "Disconnecting";
/// Wraps `kSCNetworkConnectionInvalid`.
pub const SC_NETWORK_CONNECTION_INVALID: &str = "Invalid";
/// Wraps `kSCNetworkConnectionPPPAuthenticating`.
pub const SC_NETWORK_CONNECTION_PPP_AUTHENTICATING: &str = "Authenticating";
/// Wraps `kSCNetworkConnectionPPPConnected`.
pub const SC_NETWORK_CONNECTION_PPP_CONNECTED: &str = "Connected";
/// Wraps `kSCNetworkConnectionPPPConnectingLink`.
pub const SC_NETWORK_CONNECTION_PPP_CONNECTING_LINK: &str = "ConnectingLink";
/// Wraps `kSCNetworkConnectionPPPDialOnTraffic`.
pub const SC_NETWORK_CONNECTION_PPP_DIAL_ON_TRAFFIC: &str = "DialOnTraffic";
/// Wraps `kSCNetworkConnectionPPPDisconnected`.
pub const SC_NETWORK_CONNECTION_PPP_DISCONNECTED: &str = "Disconnected";
/// Wraps `kSCNetworkConnectionPPPDisconnectingLink`.
pub const SC_NETWORK_CONNECTION_PPP_DISCONNECTING_LINK: &str = "DisconnectingLink";
/// Wraps `kSCNetworkConnectionPPPHoldLinkOff`.
pub const SC_NETWORK_CONNECTION_PPP_HOLD_LINK_OFF: &str = "HoldingLinkOff";
/// Wraps `kSCNetworkConnectionPPPInitializing`.
pub const SC_NETWORK_CONNECTION_PPP_INITIALIZING: &str = "Initializing";
/// Wraps `kSCNetworkConnectionPPPNegotiatingLink`.
pub const SC_NETWORK_CONNECTION_PPP_NEGOTIATING_LINK: &str = "NegotiatingLink";
/// Wraps `kSCNetworkConnectionPPPNegotiatingNetwork`.
pub const SC_NETWORK_CONNECTION_PPP_NEGOTIATING_NETWORK: &str = "NegotiatingNetwork";
/// Wraps `kSCNetworkConnectionPPPSuspended`.
pub const SC_NETWORK_CONNECTION_PPP_SUSPENDED: &str = "Suspended";
/// Wraps `kSCNetworkConnectionPPPTerminating`.
pub const SC_NETWORK_CONNECTION_PPP_TERMINATING: &str = "Terminating";
/// Wraps `kSCNetworkConnectionPPPWaitingForCallBack`.
pub const SC_NETWORK_CONNECTION_PPP_WAITING_FOR_CALL_BACK: &str = "WaitingForCallBack";
/// Wraps `kSCNetworkConnectionPPPWaitingForRedial`.
pub const SC_NETWORK_CONNECTION_PPP_WAITING_FOR_REDIAL: &str = "WaitingForRedial";
