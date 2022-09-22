# Android Auto Projection (AAP) Channels

*(Note: information taken from [this cache](./Head%20Unit%20Integration%20Guide.html))*

> The platform implementation supports USB 2.0, but the protocol itself is transport-agnostic and runs over any transport with sufficient bandwidth (future implementations may utilize Wi-Fi Direct). The AAP protocol includes separate channels to manage data streams between mobile device and vehicle

## Channels

| Channel Type | Direction  | Description |
|:--|:----|:--|
| Control | bi-directional | Manages link initialization and setup of channels for media, input, etc. |
| Video Output | device --> vehicle | Sends H.264 video from the mobile device to the vehicle for display on the main console display. |
| Audio Output | device --> vehicle | Carries audio from the mobile device to the vehicle for output through the vehicle speakers (AAC with 48k, AAC or PCM for 16k). |
| Vehicle Data | vehicle --> device | Carries vehicle-associated (GPS, wheel speed, etc.) data from the vehicle to the mobile device. |
| Input | vehicle --> device | Sends input events to the mobile device from input devices on the vehicle, such as touchscreens, buttons, controllers, etc. |
| Microphone Audio | vehicle --> device | Used by the mobile device to receive audio captured by the vehicle microphone. |
| Bluetooth | bi-directional | Used for communication of Bluetooth data.|
| Navigation Status | device --> vehicle | Used for communication of Navigation status and information. |
| Media Browse | device --> vehicle | Used for communication of Media browsing information. |
