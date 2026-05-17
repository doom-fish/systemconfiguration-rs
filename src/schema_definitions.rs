// Reserved Keys
pub const SC_RESV_LINK: &str = "__LINK__";
pub const SC_RESV_INACTIVE: &str = "__INACTIVE__";

// Generic Keys
pub const SC_PROP_INTERFACE_NAME: &str = "InterfaceName";
pub const SC_PROP_MAC_ADDRESS: &str = "MACAddress";
pub const SC_PROP_USER_DEFINED_NAME: &str = "UserDefinedName";
pub const SC_PROP_VERSION: &str = "Version";

// Preference Keys
pub const SC_PREF_CURRENT_SET: &str = "CurrentSet";
pub const SC_PREF_NETWORK_SERVICES: &str = "NetworkServices";
pub const SC_PREF_SETS: &str = "Sets";
pub const SC_PREF_SYSTEM: &str = "System";

// Component Keys
pub const SC_COMP_NETWORK: &str = "Network";
pub const SC_COMP_SERVICE: &str = "Service";
pub const SC_COMP_GLOBAL: &str = "Global";
pub const SC_COMP_HOST_NAMES: &str = "HostNames";
pub const SC_COMP_INTERFACE: &str = "Interface";
pub const SC_COMP_SYSTEM: &str = "System";
pub const SC_COMP_USERS: &str = "Users";
pub const SC_COMP_ANY_REGEX: &str = "[^/]+";
pub const SC_COMP_NETWORK_HOST_NAMES: &str = "HostNames";

// Network Entity Keys
pub const SC_ENT_NET_AIR_PORT: &str = "AirPort";
pub const SC_ENT_NET_DHCP: &str = "DHCP";
pub const SC_ENT_NET_DNS: &str = "DNS";
pub const SC_ENT_NET_ETHERNET: &str = "Ethernet";
pub const SC_ENT_NET_FIRE_WIRE: &str = "FireWire";
pub const SC_ENT_NET_INTERFACE: &str = "Interface";
pub const SC_ENT_NET_IP_SEC: &str = "IPSec";
pub const SC_ENT_NET_IPV4: &str = "IPv4";
pub const SC_ENT_NET_IPV6: &str = "IPv6";
pub const SC_ENT_NET_L2TP: &str = "L2TP";
pub const SC_ENT_NET_LINK: &str = "Link";
pub const SC_ENT_NET_MODEM: &str = "Modem";
pub const SC_ENT_NET_PPP: &str = "PPP";
pub const SC_ENT_NET_PPPOE: &str = "PPPoE";
pub const SC_ENT_NET_PPP_SERIAL: &str = "PPPSerial";
pub const SC_ENT_NET_PPTP: &str = "PPTP";
pub const SC_ENT_NET_PROXIES: &str = "Proxies";
pub const SC_ENT_NET_SMB: &str = "SMB";
pub const SC_ENT_NET_6TO4: &str = "6to4";
pub const SC_ENT_USERS_CONSOLE_USER: &str = "ConsoleUser";

// kSCCompNetwork Properties
pub const SC_PROP_NET_OVERRIDE_PRIMARY: &str = "OverridePrimary";
pub const SC_PROP_NET_SERVICE_ORDER: &str = "ServiceOrder";
pub const SC_PROP_NET_PPP_OVERRIDE_PRIMARY: &str = "PPPOverridePrimary";
pub const SC_PROP_NET_SERVICE_SUB_TYPE: &str = "ServiceSubType";

// kSCCompNetworkInterface Properties
pub const SC_PROP_NET_INTERFACES: &str = "Interfaces";

// kSCCompNetworkHostNames Properties
pub const SC_PROP_NET_LOCAL_HOST_NAME: &str = "LocalHostName";

// DNS
pub const SC_PROP_NET_DNS_DOMAIN_NAME: &str = "DomainName";
pub const SC_PROP_NET_DNS_OPTIONS: &str = "Options";
pub const SC_PROP_NET_DNS_SEARCH_DOMAINS: &str = "SearchDomains";
pub const SC_PROP_NET_DNS_SEARCH_ORDER: &str = "SearchOrder";
pub const SC_PROP_NET_DNS_SERVER_ADDRESSES: &str = "ServerAddresses";
pub const SC_PROP_NET_DNS_SERVER_PORT: &str = "ServerPort";
pub const SC_PROP_NET_DNS_SERVER_TIMEOUT: &str = "ServerTimeout";
pub const SC_PROP_NET_DNS_SORT_LIST: &str = "SortList";
pub const SC_PROP_NET_DNS_SUPPLEMENTAL_MATCH_DOMAINS: &str = "SupplementalMatchDomains";
pub const SC_PROP_NET_DNS_SUPPLEMENTAL_MATCH_ORDERS: &str = "SupplementalMatchOrders";

// Ethernet
pub const SC_PROP_NET_ETHERNET_MEDIA_SUB_TYPE: &str = "MediaSubType";
pub const SC_PROP_NET_ETHERNET_MEDIA_OPTIONS: &str = "MediaOptions";
pub const SC_PROP_NET_ETHERNET_MTU: &str = "MTU";

// Interface entity
pub const SC_PROP_NET_INTERFACE_DEVICE_NAME: &str = "DeviceName";
pub const SC_PROP_NET_INTERFACE_HARDWARE: &str = "Hardware";
pub const SC_PROP_NET_INTERFACE_TYPE: &str = "Type";
pub const SC_PROP_NET_INTERFACE_SUB_TYPE: &str = "SubType";
pub const SC_PROP_NET_INTERFACE_SUPPORTS_MODEM_ON_HOLD: &str = "SupportsModemOnHold";

// Interface type values
pub const SC_VAL_NET_INTERFACE_TYPE_ETHERNET: &str = "Ethernet";
pub const SC_VAL_NET_INTERFACE_TYPE_FIRE_WIRE: &str = "FireWire";
pub const SC_VAL_NET_INTERFACE_TYPE_PPP: &str = "PPP";
pub const SC_VAL_NET_INTERFACE_TYPE_6TO4: &str = "6to4";
pub const SC_VAL_NET_INTERFACE_TYPE_IP_SEC: &str = "IPSec";

// Interface subtype values
pub const SC_VAL_NET_INTERFACE_SUB_TYPE_PPPOE: &str = "PPPoE";
pub const SC_VAL_NET_INTERFACE_SUB_TYPE_PPP_SERIAL: &str = "PPPSerial";
pub const SC_VAL_NET_INTERFACE_SUB_TYPE_PPTP: &str = "PPTP";
pub const SC_VAL_NET_INTERFACE_SUB_TYPE_L2TP: &str = "L2TP";

// IPSec
pub const SC_PROP_NET_IP_SEC_AUTHENTICATION_METHOD: &str = "AuthenticationMethod";
pub const SC_PROP_NET_IP_SEC_LOCAL_CERTIFICATE: &str = "LocalCertificate";
pub const SC_PROP_NET_IP_SEC_LOCAL_IDENTIFIER: &str = "LocalIdentifier";
pub const SC_PROP_NET_IP_SEC_LOCAL_IDENTIFIER_TYPE: &str = "LocalIdentifierType";
pub const SC_PROP_NET_IP_SEC_SHARED_SECRET: &str = "SharedSecret";
pub const SC_PROP_NET_IP_SEC_SHARED_SECRET_ENCRYPTION: &str = "SharedSecretEncryption";
pub const SC_PROP_NET_IP_SEC_CONNECT_TIME: &str = "ConnectTime";
pub const SC_PROP_NET_IP_SEC_REMOTE_ADDRESS: &str = "RemoteAddress";
pub const SC_PROP_NET_IP_SEC_STATUS: &str = "Status";
pub const SC_PROP_NET_IP_SEC_XAUTH_ENABLED: &str = "XAuthEnabled";
pub const SC_PROP_NET_IP_SEC_XAUTH_NAME: &str = "XAuthName";
pub const SC_PROP_NET_IP_SEC_XAUTH_PASSWORD: &str = "XAuthPassword";
pub const SC_PROP_NET_IP_SEC_XAUTH_PASSWORD_ENCRYPTION: &str = "XAuthPasswordEncryption";

pub const SC_VAL_NET_IP_SEC_AUTHENTICATION_METHOD_SHARED_SECRET: &str = "SharedSecret";
pub const SC_VAL_NET_IP_SEC_AUTHENTICATION_METHOD_CERTIFICATE: &str = "Certificate";
pub const SC_VAL_NET_IP_SEC_AUTHENTICATION_METHOD_HYBRID: &str = "Hybrid";
pub const SC_VAL_NET_IP_SEC_LOCAL_IDENTIFIER_TYPE_KEY_ID: &str = "KeyID";
pub const SC_VAL_NET_IP_SEC_SHARED_SECRET_ENCRYPTION_KEYCHAIN: &str = "Keychain";
pub const SC_VAL_NET_IP_SEC_XAUTH_PASSWORD_ENCRYPTION_KEYCHAIN: &str = "Keychain";
pub const SC_VAL_NET_IP_SEC_XAUTH_PASSWORD_ENCRYPTION_PROMPT: &str = "Prompt";

// IPv4
pub const SC_PROP_NET_IPV4_ADDRESSES: &str = "Addresses";
pub const SC_PROP_NET_IPV4_CONFIG_METHOD: &str = "ConfigMethod";
pub const SC_PROP_NET_IPV4_DHCP_CLIENT_ID: &str = "DHCPClientID";
pub const SC_PROP_NET_IPV4_ROUTER: &str = "Router";
pub const SC_PROP_NET_IPV4_SUBNET_MASKS: &str = "SubnetMasks";
pub const SC_PROP_NET_IPV4_DEST_ADDRESSES: &str = "DestAddresses";
pub const SC_PROP_NET_IPV4_BROADCAST_ADDRESSES: &str = "BroadcastAddresses";

pub const SC_VAL_NET_IPV4_CONFIG_METHOD_AUTOMATIC: &str = "Automatic";
pub const SC_VAL_NET_IPV4_CONFIG_METHOD_BOOTP: &str = "BOOTP";
pub const SC_VAL_NET_IPV4_CONFIG_METHOD_DHCP: &str = "DHCP";
pub const SC_VAL_NET_IPV4_CONFIG_METHOD_INFORM: &str = "INFORM";
pub const SC_VAL_NET_IPV4_CONFIG_METHOD_LINK_LOCAL: &str = "LinkLocal";
pub const SC_VAL_NET_IPV4_CONFIG_METHOD_MANUAL: &str = "Manual";
pub const SC_VAL_NET_IPV4_CONFIG_METHOD_PPP: &str = "PPP";

// IPv6
pub const SC_PROP_NET_IPV6_ADDRESSES: &str = "Addresses";
pub const SC_PROP_NET_IPV6_CONFIG_METHOD: &str = "ConfigMethod";
pub const SC_PROP_NET_IPV6_DEST_ADDRESSES: &str = "DestAddresses";
pub const SC_PROP_NET_IPV6_FLAGS: &str = "Flags";
pub const SC_PROP_NET_IPV6_PREFIX_LENGTH: &str = "PrefixLength";
pub const SC_PROP_NET_IPV6_ROUTER: &str = "Router";

pub const SC_VAL_NET_IPV6_CONFIG_METHOD_AUTOMATIC: &str = "Automatic";
pub const SC_VAL_NET_IPV6_CONFIG_METHOD_LINK_LOCAL: &str = "LinkLocal";
pub const SC_VAL_NET_IPV6_CONFIG_METHOD_MANUAL: &str = "Manual";
pub const SC_VAL_NET_IPV6_CONFIG_METHOD_ROUTER_ADVERTISEMENT: &str = "RouterAdvertisement";
pub const SC_VAL_NET_IPV6_CONFIG_METHOD_6TO4: &str = "6to4";

// 6to4
pub const SC_PROP_NET_6TO4_RELAY: &str = "Relay";

// Link
pub const SC_PROP_NET_LINK_ACTIVE: &str = "Active";
pub const SC_PROP_NET_LINK_DETACHING: &str = "Detaching";

// Modem
pub const SC_PROP_NET_MODEM_ACCESS_POINT_NAME: &str = "AccessPointName";
pub const SC_PROP_NET_MODEM_CONNECTION_PERSONALITY: &str = "ConnectionPersonality";
pub const SC_PROP_NET_MODEM_CONNECTION_SCRIPT: &str = "ConnectionScript";
pub const SC_PROP_NET_MODEM_CONNECT_SPEED: &str = "ConnectSpeed";
pub const SC_PROP_NET_MODEM_DATA_COMPRESSION: &str = "DataCompression";
pub const SC_PROP_NET_MODEM_DEVICE_CONTEXT_ID: &str = "DeviceContextID";
pub const SC_PROP_NET_MODEM_DEVICE_MODEL: &str = "DeviceModel";
pub const SC_PROP_NET_MODEM_DEVICE_VENDOR: &str = "DeviceVendor";
pub const SC_PROP_NET_MODEM_DIAL_MODE: &str = "DialMode";
pub const SC_PROP_NET_MODEM_ERROR_CORRECTION: &str = "ErrorCorrection";
pub const SC_PROP_NET_MODEM_HOLD_CALL_WAITING_AUDIBLE_ALERT: &str = "HoldCallWaitingAudibleAlert";
pub const SC_PROP_NET_MODEM_HOLD_DISCONNECT_ON_ANSWER: &str = "HoldDisconnectOnAnswer";
pub const SC_PROP_NET_MODEM_HOLD_ENABLED: &str = "HoldEnabled";
pub const SC_PROP_NET_MODEM_HOLD_REMINDER: &str = "HoldReminder";
pub const SC_PROP_NET_MODEM_HOLD_REMINDER_TIME: &str = "HoldReminderTime";
pub const SC_PROP_NET_MODEM_NOTE: &str = "Note";
pub const SC_PROP_NET_MODEM_PULSE_DIAL: &str = "PulseDial";
pub const SC_PROP_NET_MODEM_SPEAKER: &str = "Speaker";
pub const SC_PROP_NET_MODEM_SPEED: &str = "Speed";

pub const SC_VAL_NET_MODEM_DIAL_MODE_IGNORE_DIAL_TONE: &str = "IgnoreDialTone";
pub const SC_VAL_NET_MODEM_DIAL_MODE_MANUAL: &str = "Manual";
pub const SC_VAL_NET_MODEM_DIAL_MODE_WAIT_FOR_DIAL_TONE: &str = "WaitForDialTone";

// PPP
pub const SC_PROP_NET_PPP_ACSP_ENABLED: &str = "ACSPEnabled";
pub const SC_PROP_NET_PPP_CONNECT_TIME: &str = "ConnectTime";
pub const SC_PROP_NET_PPP_DEVICE_LAST_CAUSE: &str = "DeviceLastCause";
pub const SC_PROP_NET_PPP_DIAL_ON_DEMAND: &str = "DialOnDemand";
pub const SC_PROP_NET_PPP_DISCONNECT_ON_FAST_USER_SWITCH: &str = "DisconnectOnFastUserSwitch";
pub const SC_PROP_NET_PPP_DISCONNECT_ON_IDLE: &str = "DisconnectOnIdle";
pub const SC_PROP_NET_PPP_DISCONNECT_ON_IDLE_TIMER: &str = "DisconnectOnIdleTimer";
pub const SC_PROP_NET_PPP_DISCONNECT_ON_LOGOUT: &str = "DisconnectOnLogout";
pub const SC_PROP_NET_PPP_DISCONNECT_ON_SLEEP: &str = "DisconnectOnSleep";
pub const SC_PROP_NET_PPP_DISCONNECT_TIME: &str = "DisconnectTime";
pub const SC_PROP_NET_PPP_IDLE_REMINDER: &str = "IdleReminder";
pub const SC_PROP_NET_PPP_IDLE_REMINDER_TIMER: &str = "IdleReminderTimer";
pub const SC_PROP_NET_PPP_LAST_CAUSE: &str = "LastCause";
pub const SC_PROP_NET_PPP_LOGFILE: &str = "Logfile";
pub const SC_PROP_NET_PPP_RETRY_CONNECT_TIME: &str = "RetryConnectTime";
pub const SC_PROP_NET_PPP_SESSION_TIMER: &str = "SessionTimer";
pub const SC_PROP_NET_PPP_STATUS: &str = "Status";
pub const SC_PROP_NET_PPP_USE_SESSION_TIMER: &str = "UseSessionTimer";
pub const SC_PROP_NET_PPP_VERBOSE_LOGGING: &str = "VerboseLogging";

// PPP Auth
pub const SC_PROP_NET_PPP_AUTH_EAP_PLUGINS: &str = "AuthEAPPlugins";
pub const SC_PROP_NET_PPP_AUTH_NAME: &str = "AuthName";
pub const SC_PROP_NET_PPP_AUTH_PASSWORD: &str = "AuthPassword";
pub const SC_PROP_NET_PPP_AUTH_PASSWORD_ENCRYPTION: &str = "AuthPasswordEncryption";
pub const SC_PROP_NET_PPP_AUTH_PROMPT: &str = "AuthPrompt";
pub const SC_PROP_NET_PPP_AUTH_PROTOCOL: &str = "AuthProtocol";

pub const SC_VAL_NET_PPP_AUTH_PASSWORD_ENCRYPTION_KEYCHAIN: &str = "Keychain";
pub const SC_VAL_NET_PPP_AUTH_PASSWORD_ENCRYPTION_TOKEN: &str = "Token";
pub const SC_VAL_NET_PPP_AUTH_PROMPT_BEFORE: &str = "Before";
pub const SC_VAL_NET_PPP_AUTH_PROMPT_AFTER: &str = "After";
pub const SC_VAL_NET_PPP_AUTH_PROTOCOL_CHAP: &str = "CHAP";
pub const SC_VAL_NET_PPP_AUTH_PROTOCOL_EAP: &str = "EAP";
pub const SC_VAL_NET_PPP_AUTH_PROTOCOL_MSCHAP1: &str = "MSCHAP1";
pub const SC_VAL_NET_PPP_AUTH_PROTOCOL_MSCHAP2: &str = "MSCHAP2";
pub const SC_VAL_NET_PPP_AUTH_PROTOCOL_PAP: &str = "PAP";

// PPP Comm
pub const SC_PROP_NET_PPP_COMM_ALTERNATE_REMOTE_ADDRESS: &str = "CommAlternateRemoteAddress";
pub const SC_PROP_NET_PPP_COMM_CONNECT_DELAY: &str = "CommConnectDelay";
pub const SC_PROP_NET_PPP_COMM_DISPLAY_TERMINAL_WINDOW: &str = "CommDisplayTerminalWindow";
pub const SC_PROP_NET_PPP_COMM_REDIAL_COUNT: &str = "CommRedialCount";
pub const SC_PROP_NET_PPP_COMM_REDIAL_ENABLED: &str = "CommRedialEnabled";
pub const SC_PROP_NET_PPP_COMM_REDIAL_INTERVAL: &str = "CommRedialInterval";
pub const SC_PROP_NET_PPP_COMM_REMOTE_ADDRESS: &str = "CommRemoteAddress";
pub const SC_PROP_NET_PPP_COMM_TERMINAL_SCRIPT: &str = "CommTerminalScript";
pub const SC_PROP_NET_PPP_COMM_USE_TERMINAL_SCRIPT: &str = "CommUseTerminalScript";

// PPP CCP
pub const SC_PROP_NET_PPP_CCP_ENABLED: &str = "CCPEnabled";
pub const SC_PROP_NET_PPP_CCP_MPPE40_ENABLED: &str = "CCPMPPE40Enabled";
pub const SC_PROP_NET_PPP_CCP_MPPE128_ENABLED: &str = "CCPMPPE128Enabled";

// PPP IPCP
pub const SC_PROP_NET_PPP_IPCP_COMPRESSION_VJ: &str = "IPCPCompressionVJ";
pub const SC_PROP_NET_PPP_IPCP_USE_PEER_DNS: &str = "IPCPUsePeerDNS";

// PPP LCP
pub const SC_PROP_NET_PPP_LCP_COMPRESSION_AC_FIELD: &str = "LCPCompressionACField";
pub const SC_PROP_NET_PPP_LCP_COMPRESSION_P_FIELD: &str = "LCPCompressionPField";
pub const SC_PROP_NET_PPP_LCP_ECHO_ENABLED: &str = "LCPEchoEnabled";
pub const SC_PROP_NET_PPP_LCP_ECHO_FAILURE: &str = "LCPEchoFailure";
pub const SC_PROP_NET_PPP_LCP_ECHO_INTERVAL: &str = "LCPEchoInterval";
pub const SC_PROP_NET_PPP_LCP_MRU: &str = "LCPMRU";
pub const SC_PROP_NET_PPP_LCP_MTU: &str = "LCPMTU";
pub const SC_PROP_NET_PPP_LCP_RECEIVE_ACCM: &str = "LCPReceiveACCM";
pub const SC_PROP_NET_PPP_LCP_TRANSMIT_ACCM: &str = "LCPTransmitACCM";

// PPP plugins
pub const SC_PROP_NET_PPP_PLUGINS: &str = "Plugins";

// Proxies
pub const SC_PROP_NET_PROXIES_EXCEPTION_LIST: &str = "ExceptionsList";
pub const SC_PROP_NET_PROXIES_EXCLUDE_SIMPLE_HOSTNAMES: &str = "ExcludeSimpleHostnames";
pub const SC_PROP_NET_PROXIES_FTP_ENABLE: &str = "FTPEnable";
pub const SC_PROP_NET_PROXIES_FTP_PASSIVE: &str = "FTPPassive";
pub const SC_PROP_NET_PROXIES_FTP_PORT: &str = "FTPPort";
pub const SC_PROP_NET_PROXIES_FTP_PROXY: &str = "FTPProxy";
pub const SC_PROP_NET_PROXIES_FTP_USER: &str = "FTPUser";
pub const SC_PROP_NET_PROXIES_GOPHER_ENABLE: &str = "GopherEnable";
pub const SC_PROP_NET_PROXIES_GOPHER_PORT: &str = "GopherPort";
pub const SC_PROP_NET_PROXIES_GOPHER_PROXY: &str = "GopherProxy";
pub const SC_PROP_NET_PROXIES_GOPHER_USER: &str = "GopherUser";
pub const SC_PROP_NET_PROXIES_HTTP_ENABLE: &str = "HTTPEnable";
pub const SC_PROP_NET_PROXIES_HTTP_PORT: &str = "HTTPPort";
pub const SC_PROP_NET_PROXIES_HTTP_PROXY: &str = "HTTPProxy";
pub const SC_PROP_NET_PROXIES_HTTPS_ENABLE: &str = "HTTPSEnable";
pub const SC_PROP_NET_PROXIES_HTTPS_PORT: &str = "HTTPSPort";
pub const SC_PROP_NET_PROXIES_HTTPS_PROXY: &str = "HTTPSProxy";
pub const SC_PROP_NET_PROXIES_HTTPS_USER: &str = "HTTPSUser";
pub const SC_PROP_NET_PROXIES_HTTP_USER: &str = "HTTPUser";
pub const SC_PROP_NET_PROXIES_PROXY_AUTO_CONFIG_ENABLE: &str = "ProxyAutoConfigEnable";
pub const SC_PROP_NET_PROXIES_PROXY_AUTO_CONFIG_JAVASCRIPT: &str = "ProxyAutoConfigJavaScript";
pub const SC_PROP_NET_PROXIES_PROXY_AUTO_CONFIG_URL_STRING: &str = "ProxyAutoConfigURLString";
pub const SC_PROP_NET_PROXIES_PROXY_AUTO_DISCOVERY_ENABLE: &str = "ProxyAutoDiscoveryEnable";
pub const SC_PROP_NET_PROXIES_RTSP_ENABLE: &str = "RTSPEnable";
pub const SC_PROP_NET_PROXIES_RTSP_PORT: &str = "RTSPPort";
pub const SC_PROP_NET_PROXIES_RTSP_PROXY: &str = "RTSPProxy";
pub const SC_PROP_NET_PROXIES_RTSP_USER: &str = "RTSPUser";
pub const SC_PROP_NET_PROXIES_SOCKS_ENABLE: &str = "SOCKSEnable";
pub const SC_PROP_NET_PROXIES_SOCKS_PORT: &str = "SOCKSPort";
pub const SC_PROP_NET_PROXIES_SOCKS_PROXY: &str = "SOCKSProxy";
pub const SC_PROP_NET_PROXIES_SOCKS_USER: &str = "SOCKSUser";

// SMB
pub const SC_PROP_NET_SMB_NET_BIOS_NAME: &str = "NetBIOSName";
pub const SC_PROP_NET_SMB_NET_BIOS_NODE_TYPE: &str = "NetBIOSNodeType";
pub const SC_PROP_NET_SMB_NET_BIOS_SCOPE: &str = "NetBIOSScope";
pub const SC_PROP_NET_SMB_WINS_ADDRESSES: &str = "WINSAddresses";
pub const SC_PROP_NET_SMB_WORKGROUP: &str = "Workgroup";

pub const SC_VAL_NET_SMB_NET_BIOS_NODE_TYPE_BROADCAST: &str = "Broadcast";
pub const SC_VAL_NET_SMB_NET_BIOS_NODE_TYPE_PEER: &str = "Peer";
pub const SC_VAL_NET_SMB_NET_BIOS_NODE_TYPE_MIXED: &str = "Mixed";
pub const SC_VAL_NET_SMB_NET_BIOS_NODE_TYPE_HYBRID: &str = "Hybrid";

// L2TP
pub const SC_PROP_NET_L2TP_IP_SEC_SHARED_SECRET: &str = "IPSecSharedSecret";
pub const SC_PROP_NET_L2TP_IP_SEC_SHARED_SECRET_ENCRYPTION: &str = "IPSecSharedSecretEncryption";
pub const SC_PROP_NET_L2TP_TRANSPORT: &str = "Transport";

pub const SC_VAL_NET_L2TP_IP_SEC_SHARED_SECRET_ENCRYPTION_KEYCHAIN: &str = "Keychain";
pub const SC_VAL_NET_L2TP_TRANSPORT_IP: &str = "IP";
pub const SC_VAL_NET_L2TP_TRANSPORT_IP_SEC: &str = "IPSec";

// AirPort
pub const SC_PROP_NET_AIR_PORT_ALLOW_NET_CREATION: &str = "AllowNetCreation";
pub const SC_PROP_NET_AIR_PORT_AUTH_PASSWORD: &str = "AuthPassword";
pub const SC_PROP_NET_AIR_PORT_AUTH_PASSWORD_ENCRYPTION: &str = "AuthPasswordEncryption";
pub const SC_PROP_NET_AIR_PORT_JOIN_MODE: &str = "JoinMode";
pub const SC_PROP_NET_AIR_PORT_POWER_ENABLED: &str = "PowerEnabled";
pub const SC_PROP_NET_AIR_PORT_PREFERRED_NETWORK: &str = "PreferredNetwork";
pub const SC_PROP_NET_AIR_PORT_SAVE_PASSWORDS: &str = "SavePasswords";

// System
pub const SC_PROP_SYSTEM_COMPUTER_NAME: &str = "ComputerName";
pub const SC_PROP_SYSTEM_COMPUTER_NAME_ENCODING: &str = "ComputerNameEncoding";

// Users/ConsoleUser
pub const SC_PROP_USERS_CONSOLE_USER_NAME: &str = "Name";
pub const SC_PROP_USERS_CONSOLE_USER_UID: &str = "UID";
pub const SC_PROP_USERS_CONSOLE_USER_GID: &str = "GID";

// Dynamic Store
pub const SC_DYNAMIC_STORE_DOMAIN_FILE: &str = "File";
pub const SC_DYNAMIC_STORE_DOMAIN_PLUGIN: &str = "Plugin";
pub const SC_DYNAMIC_STORE_DOMAIN_PREFS: &str = "Prefs";
pub const SC_DYNAMIC_STORE_DOMAIN_SETUP: &str = "Setup";
pub const SC_DYNAMIC_STORE_DOMAIN_STATE: &str = "State";
pub const SC_DYNAMIC_STORE_PROP_NET_INTERFACES: &str = "Interfaces";
pub const SC_DYNAMIC_STORE_PROP_NET_PRIMARY_INTERFACE: &str = "PrimaryInterface";
pub const SC_DYNAMIC_STORE_PROP_NET_PRIMARY_SERVICE: &str = "PrimaryService";
pub const SC_DYNAMIC_STORE_PROP_NET_SERVICE_IDS: &str = "ServiceIDs";
pub const SC_DYNAMIC_STORE_PROP_SETUP_CURRENT_SET: &str = "CurrentSet";
pub const SC_DYNAMIC_STORE_PROP_SETUP_LAST_UPDATED: &str = "LastUpdated";
pub const SC_DYNAMIC_STORE_USE_SESSION_KEYS: &str = "UseSessionKeys";

// Network Interface Type constants (SCNetworkConfiguration.h)
pub const SC_NETWORK_INTERFACE_TYPE_6TO4: &str = "6to4";
pub const SC_NETWORK_INTERFACE_TYPE_BLUETOOTH: &str = "Bluetooth";
pub const SC_NETWORK_INTERFACE_TYPE_BOND: &str = "Bond";
pub const SC_NETWORK_INTERFACE_TYPE_ETHERNET: &str = "Ethernet";
pub const SC_NETWORK_INTERFACE_TYPE_FIRE_WIRE: &str = "FireWire";
pub const SC_NETWORK_INTERFACE_TYPE_IEEE80211: &str = "IEEE80211";
pub const SC_NETWORK_INTERFACE_TYPE_IP_SEC: &str = "IPSec";
pub const SC_NETWORK_INTERFACE_TYPE_IPV4: &str = "IPv4";
pub const SC_NETWORK_INTERFACE_TYPE_L2TP: &str = "L2TP";
pub const SC_NETWORK_INTERFACE_TYPE_MODEM: &str = "Modem";
pub const SC_NETWORK_INTERFACE_TYPE_PPP: &str = "PPP";
pub const SC_NETWORK_INTERFACE_TYPE_SERIAL: &str = "Serial";
pub const SC_NETWORK_INTERFACE_TYPE_VLAN: &str = "VLAN";
pub const SC_NETWORK_INTERFACE_TYPE_WWAN: &str = "WWAN";
pub const SC_NETWORK_INTERFACE_TYPE_XXX: &str = "XXX";
// The loopback-like IPv4 virtual interface
pub const SC_NETWORK_INTERFACE_IPV4: &str = "lo0";

// Network Protocol Type constants
pub const SC_NETWORK_PROTOCOL_TYPE_DNS: &str = "DNS";
pub const SC_NETWORK_PROTOCOL_TYPE_IPV4: &str = "IPv4";
pub const SC_NETWORK_PROTOCOL_TYPE_IPV6: &str = "IPv6";
pub const SC_NETWORK_PROTOCOL_TYPE_PROXIES: &str = "Proxies";
pub const SC_NETWORK_PROTOCOL_TYPE_SMB: &str = "SMB";
pub const SC_NETWORK_PROTOCOL_TYPE_XXX: &str = "XXX";

// Legacy SCNetworkFlags (deprecated since 10.6 — wrap for completeness)
pub const SC_NETWORK_FLAGS_TRANSIENT_CONNECTION: u32 = 1 << 0;
pub const SC_NETWORK_FLAGS_REACHABLE: u32 = 1 << 1;
pub const SC_NETWORK_FLAGS_CONNECTION_REQUIRED: u32 = 1 << 2;
pub const SC_NETWORK_FLAGS_CONNECTION_AUTOMATIC: u32 = 1 << 3;
pub const SC_NETWORK_FLAGS_INTERVENTION_REQUIRED: u32 = 1 << 4;
pub const SC_NETWORK_FLAGS_IS_LOCAL_ADDRESS: u32 = 1 << 16;
pub const SC_NETWORK_FLAGS_IS_DIRECT: u32 = 1 << 17;

pub const SC_NETWORK_REACHABILITY_FLAGS_CONNECTION_AUTOMATIC: u32 = 1 << 3;

pub const SC_PREFERENCES_NOTIFICATION_COMMIT: u32 = 1 << 0;
pub const SC_PREFERENCES_NOTIFICATION_APPLY: u32 = 1 << 1;

pub const SC_BOND_STATUS_OK: i32 = 0;
pub const SC_BOND_STATUS_LINK_INVALID: i32 = 1;
pub const SC_BOND_STATUS_NO_PARTNER: i32 = 2;
pub const SC_BOND_STATUS_NOT_IN_ACTIVE_GROUP: i32 = 3;
pub const SC_BOND_STATUS_UNKNOWN: i32 = 999;

pub const SC_STATUS_OK: i32 = 0;
pub const SC_STATUS_FAILED: i32 = 1001;
pub const SC_STATUS_INVALID_ARGUMENT: i32 = 1002;
pub const SC_STATUS_ACCESS_ERROR: i32 = 1003;
pub const SC_STATUS_NO_KEY: i32 = 1004;
pub const SC_STATUS_KEY_EXISTS: i32 = 1005;
pub const SC_STATUS_LOCKED: i32 = 1006;
pub const SC_STATUS_NEED_LOCK: i32 = 1007;
pub const SC_STATUS_NO_STORE_SESSION: i32 = 2001;
pub const SC_STATUS_NO_STORE_SERVER: i32 = 2002;
pub const SC_STATUS_NOTIFIER_ACTIVE: i32 = 2003;
pub const SC_STATUS_NO_PREFS_SESSION: i32 = 3001;
pub const SC_STATUS_PREFS_BUSY: i32 = 3002;
pub const SC_STATUS_NO_CONFIG_FILE: i32 = 3003;
pub const SC_STATUS_NO_LINK: i32 = 3004;
pub const SC_STATUS_STALE: i32 = 3005;
pub const SC_STATUS_MAX_LINK: i32 = 3006;
pub const SC_STATUS_REACHABILITY_UNKNOWN: i32 = 4001;
pub const SC_STATUS_CONNECTION_NO_SERVICE: i32 = 5001;
pub const SC_STATUS_CONNECTION_IGNORE: i32 = 5002;

pub const CN_NETWORK_INFO_KEY_BSSID: &str = "BSSID";
pub const CN_NETWORK_INFO_KEY_SSID: &str = "SSID";
pub const CN_NETWORK_INFO_KEY_SSID_DATA: &str = "SSIDDATA";

pub const SC_NETWORK_CONNECTION_BYTES_IN: &str = "BytesIn";
pub const SC_NETWORK_CONNECTION_BYTES_OUT: &str = "BytesOut";
pub const SC_NETWORK_CONNECTION_PACKETS_IN: &str = "PacketsIn";
pub const SC_NETWORK_CONNECTION_PACKETS_OUT: &str = "PacketsOut";
pub const SC_NETWORK_CONNECTION_ERRORS_IN: &str = "ErrorsIn";
pub const SC_NETWORK_CONNECTION_ERRORS_OUT: &str = "ErrorsOut";
pub const SC_NETWORK_CONNECTION_SELECTION_OPTION_ON_DEMAND_HOST_NAME: &str = "OnDemandHostName";
pub const SC_NETWORK_CONNECTION_SELECTION_OPTION_ON_DEMAND_RETRY: &str = "OnDemandRetry";
pub const SC_NETWORK_CONNECTION_DISCONNECTED: &str = "Disconnected";
pub const SC_NETWORK_CONNECTION_CONNECTING: &str = "Connecting";
pub const SC_NETWORK_CONNECTION_CONNECTED: &str = "Connected";
pub const SC_NETWORK_CONNECTION_DISCONNECTING: &str = "Disconnecting";
pub const SC_NETWORK_CONNECTION_INVALID: &str = "Invalid";
pub const SC_NETWORK_CONNECTION_PPP_AUTHENTICATING: &str = "Authenticating";
pub const SC_NETWORK_CONNECTION_PPP_CONNECTED: &str = "Connected";
pub const SC_NETWORK_CONNECTION_PPP_CONNECTING_LINK: &str = "ConnectingLink";
pub const SC_NETWORK_CONNECTION_PPP_DIAL_ON_TRAFFIC: &str = "DialOnTraffic";
pub const SC_NETWORK_CONNECTION_PPP_DISCONNECTED: &str = "Disconnected";
pub const SC_NETWORK_CONNECTION_PPP_DISCONNECTING_LINK: &str = "DisconnectingLink";
pub const SC_NETWORK_CONNECTION_PPP_HOLD_LINK_OFF: &str = "HoldingLinkOff";
pub const SC_NETWORK_CONNECTION_PPP_INITIALIZING: &str = "Initializing";
pub const SC_NETWORK_CONNECTION_PPP_NEGOTIATING_LINK: &str = "NegotiatingLink";
pub const SC_NETWORK_CONNECTION_PPP_NEGOTIATING_NETWORK: &str = "NegotiatingNetwork";
pub const SC_NETWORK_CONNECTION_PPP_SUSPENDED: &str = "Suspended";
pub const SC_NETWORK_CONNECTION_PPP_TERMINATING: &str = "Terminating";
pub const SC_NETWORK_CONNECTION_PPP_WAITING_FOR_CALL_BACK: &str = "WaitingForCallBack";
pub const SC_NETWORK_CONNECTION_PPP_WAITING_FOR_REDIAL: &str = "WaitingForRedial";
