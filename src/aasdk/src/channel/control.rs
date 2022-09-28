
pub trait IControlServiceChannel {

}

pub trait IControlServiceChannelEventHandler {
    pub fn onVersionResponse();
    pub fn onHandshake();
    pub fn onServiceDiscoveryRequest();
    pub fn onAudioFocusRequest();
    pub fn onShutdownRequest();
    pub fn onShutdownResponse();
    pub fn onNavigationFocusRequest();
    pub fn onPingResponse();
    pub fn onChannelError();
}

pub struct ControlServiceChannel {

}
