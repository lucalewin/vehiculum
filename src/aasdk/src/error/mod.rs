
#[derive(Debug)]
#[allow(dead_code)]
pub struct Error {
    code: ErrorCode,
    native_code: u32,
    message: String,
}

impl Error {
    pub fn new(code: ErrorCode, native_code: u32, message: String) -> Error {
        Error { code, native_code, message }
    }
}

#[derive(Debug)]
pub enum ErrorCode {
    None = 0,
    UsbClaimInterface = 1,
    UsbInvalidConfigDescriptor = 2,
    UsbObtainInterfaceDescriptor = 3,
    UsbEmptyInterfaces = 4,
    UsbInvalidDeviceEndpoints = 5,
    UsbInvalidTransferMethod = 6,
    UsbTransferAllocation = 7,
    UsbListDevices = 8,
    UsbObtainConfigDescriptor = 9,
    UsbTransfer = 10,
    DataSinkCommitOverflow = 11,
    DataSinkConsumeUnderflow = 12,
    UsbAoapProtocolVersion = 13,
    UsbAoapDeviceNotFound = 14,
    SslReadCertificate = 15,
    SslReadPrivateKey = 16,
    SslMethod = 17,
    SslContextCreation = 18,
    SslUseCertificate = 19,
    SslUsePrivateKey = 20,
    SslHandlerCreation = 21,
    SslReadBioCreation = 22,
    SslWriteBioCreation = 23,
    SslHandshake = 24,
    SslWrite = 25,
    SslRead = 26,
    SslBioRead = 27,
    SslBioWrite = 28,
    MessengerIntertwinedChannels = 29,
    OperationAborted = 30,
    OperationInProgress = 31,
    ParsePayload = 32,
    TcpTransfer = 33,
    ControlVersionResponse = 34,
    InvalidPayloadLength = 35,
}
