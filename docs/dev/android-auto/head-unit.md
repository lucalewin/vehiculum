# Head Unit

## Components

The architecture on the head unit consists of the following high-level components

> **NOTE**: This document describes high-level components unique to AAP and does not provide a detailed overview of the entire head unit software stack. 

| Component | Description |
|:--|:--|
| Head Unit OS | The operating system (Android, Linux, QNX, Windows, etc.) environment software stack that powers the on-board infotainment experience. |
| OS Adaptation Layer | The OS-specific abstraction layer between the head unit OS and the AAP receiver. The layer implements AAP receiver bindings and translates the message call and data to native head unit OS subsystems. Google provides reference implementations for Android and QNX in source form, but does not provide a commercial implementation. |
| Android Auto Projection Receiver | The head unit implementation of AAP that connects with unit OS interfaces (video, audio, connectivity, input, etc.) through the OS Adaptation Layer to enable an integrated user experience. Google provides the receiver library as portable C++ source code. |

## Requirements

To support AAP, automotive head units (HUs) MUST include the following minimum capabilities (unless explicitly specified otherwise):

|Component | Requirement |
|:--|:--|
| Connectivity | USB 2.0 Hi-Speed Host<br> ● Standard USB-A port<br> ● >50 Mbits/s throughput<br> ● Support USB 2.0 CDP specification<br> ● Host support for [Android Open Accessory (AOA) protocol](https://source.android.com/docs/core/interaction/accessories/protocol)<br> ● Support USB Hub (may be exposed in HU debug mode only) Bluetooth<br> ● Hands-Free Profile (HFP) 1.5<br> ● PIN or numeric comparison pairing methods |
...