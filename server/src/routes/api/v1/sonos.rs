use {
    rocket::{get, http::Status, post},
    rocket_contrib::json::Json,
    serde::{Deserialize, Serialize},
    sonos,
    std::{net::IpAddr, time::Duration},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SonosDevice {
    ip: IpAddr,
    model: String,
    model_number: String,
    software_version: String,
    hardware_version: String,
    serial_number: String,
    name: String,
    uuid: String,
    volume: u8,
    muted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SonosTrack {
    title: String,
    artist: String,
    album: String,
    queue_position: u64,
    uri: String,
    duration: Duration,
    running_time: Duration,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SonosSpeaker {
    device: SonosDevice,
    track: Option<SonosTrack>,
}

#[get("/sonos")]
pub async fn get_sonos() -> Result<Json<Vec<SonosSpeaker>>, String> {
    let raw_devices = sonos::discover().map_err(|e| format!("{:?}", e))?;
    let mut speakers = Vec::<SonosSpeaker>::new();
    'outer: for rd in raw_devices {
        for s in &speakers {
            if s.device.serial_number == rd.serial_number {
                continue 'outer;
            }
        }

        // No duplicates left
        let track = rd.track();
        let volume = rd.volume().map_err(|e| format!("{:?}", e))?;
        let muted = rd.muted().map_err(|e| format!("{:?}", e))?;
        speakers.push(SonosSpeaker {
            device: SonosDevice {
                ip: rd.ip,
                model: rd.model,
                model_number: rd.model_number,
                software_version: rd.software_version,
                hardware_version: rd.hardware_version,
                serial_number: rd.serial_number,
                name: rd.name,
                uuid: rd.uuid,
                volume,
                muted,
            },
            track: track
                .map(|track| SonosTrack {
                    title: track.title,
                    artist: track.artist,
                    album: track.album,
                    queue_position: track.queue_position,
                    uri: track.uri,
                    duration: track.duration,
                    running_time: track.running_time,
                })
                .ok(),
        });
    }
    println!("{:#?}", speakers);
    Ok(Json(speakers))
}

macro_rules! sonos_simple_endpoint {
    ($name:ident, $action:ident) => {
        #[post("/sonos/<ip>/$action")]
        pub async fn $name(ip: IpAddr) -> Result<Status, Status> {
            sonos::Speaker::from_ip(ip)
                .and_then(|d| d.$action())
                .map_err(|e| match e.0 {
                    sonos::ErrorKind::DeviceNotFound(_) => Status::NotFound,
                    _ => Status::InternalServerError,
                })?;
            Ok(Status::Ok)
        }
    };
}

macro_rules! sonos_endpoint_arg {
    ($name:ident, $action:ident, $arg_type:ty) => {
        #[post("/sonos/<ip>/$action/<value>")]
        // #[post(stringify!(concat!("/sonos/<ip>/", stringify!($action), "/<value>")))]
        pub async fn $name(ip: IpAddr, value: $arg_type) -> Result<Status, Status> {
            sonos::Speaker::from_ip(ip)
                .and_then(|d| d.$action(&value))
                .map_err(|e| match e.0 {
                    sonos::ErrorKind::DeviceNotFound(_) => Status::NotFound,
                    _ => Status::InternalServerError,
                })?;
            Ok(Status::Ok)
        }
    };
}

macro_rules! sonos_endpoint_arg_map {
    ($name:ident, $action:ident, $arg_type:ty, $mapper:tt) => {
        #[post("/sonos/<ip>/$action/<value>")]
        pub async fn $name(ip: IpAddr, value: $arg_type) -> Result<Status, Status> {
            sonos::Speaker::from_ip(ip)
                .and_then(|d| d.$action(&$mapper(value)))
                .map_err(|e| match e.0 {
                    sonos::ErrorKind::DeviceNotFound(_) => Status::NotFound,
                    _ => Status::InternalServerError,
                })?;
            Ok(Status::Ok)
        }
    };
}

sonos_simple_endpoint!(post_sonos_play, play);
sonos_simple_endpoint!(post_sonos_pause, pause);
sonos_simple_endpoint!(post_sonos_stop, stop);
sonos_simple_endpoint!(post_sonos_next, next);
sonos_simple_endpoint!(post_sonos_previous, previous);
sonos_endpoint_arg_map!(post_sonos_seek, seek, u64, (|dur| Duration::from_secs(dur)));
sonos_endpoint_arg!(post_sonos_play_queue_item, play_queue_item, u64);
sonos_endpoint_arg!(post_sonos_remove_track, remove_track, u64);
sonos_endpoint_arg!(post_sonos_queue_track, queue_track, String);
sonos_endpoint_arg!(post_sonos_queue_next, queue_next, String);
sonos_endpoint_arg!(post_sonos_play_track, play_track, String);
sonos_simple_endpoint!(post_sonos_clear_queue, clear_queue);
// TODO:
// get_sonos_volume
// sonos_endpoint_arg!(post_sonos_set_volume, set_volume, u8);
// get_sonos_muted
sonos_simple_endpoint!(post_sonos_mute, mute);
sonos_simple_endpoint!(post_sonos_unmute, unmute);
// get_sonos_transport_state
// get_sonos_track

/*#[post("/sonos/<ip>/play")]
pub async fn post_sonos_play(ip: IpAddr) -> Result<Status, Status> {
    sonos::Speaker::from_ip(ip)
        .and_then(|d| d.play())
        .map_err(|e| match e.0 {
            sonos::ErrorKind::DeviceNotFound(_) => Status::NotFound,
            _ => Status::InternalServerError,
        })?;
    Ok(Status::Ok)
}*/
