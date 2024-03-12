//! Settings for impersonating the Chrome impersonate

use http::HeaderMap;

use crate::ClientBuilder;
use std::str::FromStr;

use super::ImpersonateSettings;
mod chrome;
mod edge;
mod okhttp;
mod safari;

pub(crate) fn configure_impersonate(ver: Impersonate, builder: ClientBuilder) -> ClientBuilder {
    let settings = get_config_from_ver(ver);
    builder
        .use_boring_tls(settings.tls_builder_func)
        .http2_initial_stream_window_size(settings.http2.initial_stream_window_size)
        .http2_initial_connection_window_size(settings.http2.initial_connection_window_size)
        .http2_max_concurrent_streams(settings.http2.max_concurrent_streams)
        .http2_max_header_list_size(settings.http2.max_header_list_size)
        .http2_header_table_size(settings.http2.header_table_size)
        .http2_enable_push(settings.http2.enable_push)
        .replace_default_headers(settings.headers)
        .brotli(settings.brotli)
        .gzip(settings.gzip)
}

fn create_profile_headers(profile: ClientProfile) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let agent: h2::profile::AgentProfile = profile.into();
    let (name, value) = agent.to_header();
    headers.insert(name, value.parse().unwrap());
    headers
}

fn get_config_from_ver(ver: Impersonate) -> ImpersonateSettings {
    let headers = create_profile_headers(ver.profile());
    match ver {
        Impersonate::Chrome99 => chrome::v99::get_settings(headers),
        Impersonate::Chrome100 => chrome::v100::get_settings(headers),
        Impersonate::Chrome101 => chrome::v101::get_settings(headers),
        Impersonate::Chrome104 => chrome::v104::get_settings(headers),
        Impersonate::Chrome105 => chrome::v105::get_settings(headers),
        Impersonate::Chrome106 => chrome::v106::get_settings(headers),
        Impersonate::Chrome107 => chrome::v107::get_settings(headers),
        Impersonate::Chrome108 => chrome::v108::get_settings(headers),
        Impersonate::Chrome109 => chrome::v109::get_settings(headers),
        Impersonate::Chrome114 => chrome::v114::get_settings(headers),
        Impersonate::Chrome116 => chrome::v116::get_settings(headers),
        Impersonate::Chrome117 => chrome::v117::get_settings(headers),
        Impersonate::Chrome118 => chrome::v118::get_settings(headers),
        Impersonate::Chrome119 => chrome::v119::get_settings(headers),
        Impersonate::Chrome120 => chrome::v120::get_settings(headers),
        Impersonate::Safari12 => safari::safari12::get_settings(headers),
        Impersonate::Safari15_3 => safari::safari15_3::get_settings(headers),
        Impersonate::Safari15_5 => safari::safari15_5::get_settings(headers),
        Impersonate::Safari15_6_1 => safari::safari15_6_1::get_settings(headers),
        Impersonate::Safari16 => safari::safari16::get_settings(headers),
        Impersonate::Safari16_5 => safari::safari16_5::get_settings(headers),
        Impersonate::Safari17_2_1 => safari::safari17_2_1::get_settings(headers),
        Impersonate::OkHttp3_9 => okhttp::okhttp3_9::get_settings(headers),
        Impersonate::OkHttp3_11 => okhttp::okhttp3_11::get_settings(headers),
        Impersonate::OkHttp3_13 => okhttp::okhttp3_13::get_settings(headers),
        Impersonate::OkHttp3_14 => okhttp::okhttp3_14::get_settings(headers),
        Impersonate::OkHttp4_9 => okhttp::okhttp4_9::get_settings(headers),
        Impersonate::OkHttp4_10 => okhttp::okhttp4_10::get_settings(headers),
        Impersonate::OkHttp5 => okhttp::okhttp5::get_settings(headers),
        Impersonate::Edge99 => edge::edge99::get_settings(headers),
        Impersonate::Edge101 => edge::edge101::get_settings(headers),
        Impersonate::Edge122 => edge::edge122::get_settings(headers),
    }
}

/// Defines the Chrome version to chromimic when setting up a builder
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub enum Impersonate {
    Chrome99,
    Chrome100,
    Chrome101,
    Chrome104,
    Chrome105,
    Chrome106,
    Chrome108,
    Chrome107,
    Chrome109,
    Chrome114,
    Chrome116,
    Chrome117,
    Chrome118,
    Chrome119,
    Chrome120,
    Safari12,
    Safari15_3,
    Safari15_5,
    Safari15_6_1,
    Safari16,
    Safari16_5,
    Safari17_2_1,
    OkHttp3_9,
    OkHttp3_11,
    OkHttp3_13,
    OkHttp3_14,
    OkHttp4_9,
    OkHttp4_10,
    OkHttp5,
    Edge99,
    Edge101,
    Edge122,
}

/// Impersonate version from string
impl FromStr for Impersonate {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "chrome99" => Ok(Impersonate::Chrome99),
            "chrome100" => Ok(Impersonate::Chrome100),
            "chrome101" => Ok(Impersonate::Chrome101),
            "chrome104" => Ok(Impersonate::Chrome104),
            "chrome105" => Ok(Impersonate::Chrome105),
            "chrome106" => Ok(Impersonate::Chrome106),
            "chrome108" => Ok(Impersonate::Chrome108),
            "chrome107" => Ok(Impersonate::Chrome107),
            "chrome109" => Ok(Impersonate::Chrome109),
            "chrome114" => Ok(Impersonate::Chrome114),
            "chrome116" => Ok(Impersonate::Chrome116),
            "chrome117" => Ok(Impersonate::Chrome117),
            "chrome118" => Ok(Impersonate::Chrome118),
            "chrome119" => Ok(Impersonate::Chrome119),
            "chrome120" => Ok(Impersonate::Chrome120),
            "safari12" => Ok(Impersonate::Safari12),
            "safari15_3" => Ok(Impersonate::Safari15_3),
            "safari15_5" => Ok(Impersonate::Safari15_5),
            "safari15_6_1" => Ok(Impersonate::Safari15_6_1),
            "safari16" => Ok(Impersonate::Safari16),
            "safari16_5" => Ok(Impersonate::Safari16_5),
            "safari17_2_1" => Ok(Impersonate::Safari17_2_1),
            "okhttp3_9" => Ok(Impersonate::OkHttp3_9),
            "okhttp3_11" => Ok(Impersonate::OkHttp3_11),
            "okhttp3_13" => Ok(Impersonate::OkHttp3_13),
            "okhttp3_14" => Ok(Impersonate::OkHttp3_14),
            "okhttp4_9" => Ok(Impersonate::OkHttp4_9),
            "okhttp4_10" => Ok(Impersonate::OkHttp4_10),
            "okhttp5" => Ok(Impersonate::OkHttp5),
            "edge99" => Ok(Impersonate::Edge99),
            "edge101" => Ok(Impersonate::Edge101),
            "edge120" => Ok(Impersonate::Edge122),
            _ => Err("Invalid Impersonate version"),
        }
    }
}

impl Impersonate {
    /// Get the client profile for the given impersonate version
    pub fn profile(&self) -> ClientProfile {
        match self {
            Impersonate::Chrome99
            | Impersonate::Chrome100
            | Impersonate::Chrome101
            | Impersonate::Chrome104
            | Impersonate::Chrome105
            | Impersonate::Chrome106
            | Impersonate::Chrome108
            | Impersonate::Chrome107
            | Impersonate::Chrome109
            | Impersonate::Chrome114
            | Impersonate::Chrome116
            | Impersonate::Chrome117
            | Impersonate::Chrome118
            | Impersonate::Chrome119
            | Impersonate::Chrome120 => ClientProfile::Chrome,

            Impersonate::Safari12
            | Impersonate::Safari15_3
            | Impersonate::Safari15_5
            | Impersonate::Safari15_6_1
            | Impersonate::Safari16
            | Impersonate::Safari16_5
            | Impersonate::Safari17_2_1 => ClientProfile::Safari,

            Impersonate::OkHttp3_9
            | Impersonate::OkHttp3_11
            | Impersonate::OkHttp3_13
            | Impersonate::OkHttp3_14
            | Impersonate::OkHttp4_9
            | Impersonate::OkHttp4_10
            | Impersonate::OkHttp5 => ClientProfile::OkHttp,

            Impersonate::Edge99 | Impersonate::Edge101 | Impersonate::Edge122 => {
                ClientProfile::Edge
            }
        }
    }
}

/// impersonate client profile
#[derive(Clone, Copy, Debug)]
pub enum ClientProfile {
    /// Chrome impersonate client profile
    Chrome,
    /// OkHttp impersonate client profile
    OkHttp,
    /// Safari impersonate client profile
    Safari,
    /// Foxfire impersonate client profile
    Firefox,
    /// Edge impersonate client profile
    Edge,
}

impl Into<h2::profile::AgentProfile> for ClientProfile {
    fn into(self) -> h2::profile::AgentProfile {
        match self {
            Self::Chrome => h2::profile::AgentProfile::Chrome,
            Self::OkHttp => h2::profile::AgentProfile::OkHttp,
            Self::Safari => h2::profile::AgentProfile::Safari,
            Self::Firefox => h2::profile::AgentProfile::Firefox,
            Self::Edge => h2::profile::AgentProfile::Edge,
        }
    }
}
