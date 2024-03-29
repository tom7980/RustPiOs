use crate::console::{self, ConsoleError};

const SOH: u8 = 0x01;
const EOT: u8 = 0x04;
const ACK: u8 = 0x06;
const NAK: u8 = 0x15;
const CAN: u8 = 0x18;

type MResult<T> = core::result::Result<T, ModemError>;

#[derive(Debug)]
pub struct ModemError {
    kind: ErrorKind,
}

impl ModemError {
    fn new(kind: ErrorKind) -> Self {
        Self {
            kind
        }
    }

    pub fn kind(&self) -> &ErrorKind {        
        &self.kind
    }
}
#[derive(PartialEq, Debug)]
pub enum ErrorKind {
    Interrupted,
    BrokenPipe,
    ConnectionAborted,
    InvalidData,
    UnexpectedEof,
    ConsoleError
}

impl From<ConsoleError> for ModemError {
    fn from(error:ConsoleError) -> Self {
        ModemError{ kind: ErrorKind::ConsoleError }
    }
}

/// Implementation of the XMODEM protocol.
pub struct Xmodem<R> {
    packet: u8,
    inner: R,
    started: bool
}

impl Xmodem<()> {

    //Unlikely to ever need to transmit using Xmodem from the Pi, if I do I'll
    //just reimplement it again when I'm not being lazy.


    /// Transmits `data` to the receiver `to` using the XMODEM protocol. If the
    /// length of the total data yielded by `data` is not a multiple of 128
    /// bytes, the data is padded with zeroes and sent to the receiver.
    ///
    /// Returns the number of bytes written to `to`, excluding padding zeroes.
    // #[inline]
    // pub fn transmit<R, W>(data: R, to: W) -> MResult<usize>
    //     where W: console::Read + console::Write, R: console::Read
    // {
    //     Xmodem::transmit_with_progress(data, to)
    // }

    /// Transmits `data` to the receiver `to` using the XMODEM protocol. If the
    /// length of the total data yielded by `data` is not a multiple of 128
    /// bytes, the data is padded with zeroes and sent to the receiver.
    ///
    /// The function `f` is used as a callback to indicate progress throughout
    /// the transmission. See the [`Progress`] enum for more information.
    ///
    /// Returns the number of bytes written to `to`, excluding padding zeroes.
    // pub fn transmit_with_progress<R, W>(mut data: R, to: W) -> MResult<usize>
    //     where W: console::Read + console::Write, R: console::Read
    // {
    //     let mut transmitter = Xmodem::new(to);
    //     let mut packet = [0u8; 128];
    //     let mut written = 0;
    //     'next_packet: loop {
    //         let n = data.read_max(&mut packet)?;
    //         packet[n..].iter_mut().for_each(|b| *b = 0);

    //         if n == 0 {
    //             transmitter.write_packet(&[])?;
    //             return Ok(written);
    //         }

    //         for _ in 0..10 {
    //             match transmitter.write_packet(&packet) {
    //                 Err(ref e) if e.kind() == &ErrorKind::Interrupted => continue,
    //                 Err(e) => return Err(e),
    //                 Ok(_) => {
    //                     written += n;
    //                     continue 'next_packet;
    //                 }
    //             }
    //         }

    //         return Err(ModemError::new(ErrorKind::BrokenPipe));
    //     }
    // }

    /// Receives `data` from `from` using the XMODEM protocol and writes it into
    /// `into`. Returns the number of bytes read from `from`, a multiple of 128.
    #[inline]
    pub fn receive<R, W>(from: R, into: W) -> MResult<usize>
       where R: console::Read + console::Write, W: console::Write
    {
        Xmodem::receive_with_progress(from, into)
    }

    /// Receives `data` from `from` using the XMODEM protocol and writes it into
    /// `into`. Returns the number of bytes read from `from`, a multiple of 128.
    ///
    /// The function `f` is used as a callback to indicate progress throughout
    /// the reception. See the [`Progress`] enum for more information.
    pub fn receive_with_progress<R, W>(from: R, mut into: W) -> MResult<usize>
       where R: console::Read + console::Write, W: console::Write
    {
        let mut receiver = Xmodem::new(from);
        let mut packet = [0u8; 128];
        let mut received = 0;
        'next_packet: loop {
            for _ in 0..10 {
                match receiver.read_packet(&mut packet) {
                    Err(ref e) if e.kind() == &ErrorKind::Interrupted => continue,
                    Err(e) => return Err(e),
                    Ok(0) => break 'next_packet,
                    Ok(n) => {
                        received += n;
                        into.write(&packet);
                        continue 'next_packet;
                    }
                }
            }

            return Err(ModemError::new(ErrorKind::BrokenPipe));
        }

        Ok(received)
    }
}

impl<T: console::Read + console::Write> Xmodem<T> {
    /// Returns a new `Xmodem` instance with the internal reader/writer set to
    /// `inner`. The returned instance can be used for both receiving
    /// (downloading) and sending (uploading).
    pub fn new(inner: T) -> Self {
        Xmodem { packet: 1, started: false, inner}
    }

    /// Reads a single byte from the inner I/O stream. If `abort_on_can` is
    /// `true`, an error of `ConnectionAborted` is returned if the read byte is
    /// `CAN`.
    ///
    /// # Errors
    ///
    /// Returns an error if reading from the inner stream fails or if
    /// `abort_on_can` is `true` and the read byte is `CAN`.
    fn read_byte(&mut self, abort_on_can: bool) -> MResult<u8> {
        let byte = self.inner.read_byte()?;

        if abort_on_can && byte == CAN {
            return Err(ModemError::new(ErrorKind::ConnectionAborted));
        }

        Ok(byte)
    }

    /// Writes a single byte to the inner I/O stream.
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the inner stream fails.
    fn write_byte(&mut self, byte: u8) -> MResult<()> {
        self.inner.write(&[byte]);

        Ok(())
    }

    /// Reads a single byte from the inner I/O stream and compares it to `byte`.
    /// If the bytes match, the byte is returned as an `Ok`. If they differ and
    /// the read byte is not `CAN`, an error of `InvalidData` with the message
    /// `expected` is returned. If they differ and the read byte is `CAN`, an
    /// error of `ConnectionAborted` is returned. In either case, if they bytes
    /// differ, a `CAN` byte is written out to the inner stream.
    ///
    /// # Errors
    ///
    /// Returns an error if reading from the inner stream fails, if the read
    /// byte was not `byte`, if the read byte was `CAN` and `byte` is not `CAN`,
    /// or if writing the `CAN` byte failed on byte mismatch.
    fn expect_byte_or_cancel(&mut self, byte: u8, expected: &'static str) -> MResult<u8> {
        let read = self.inner.read_byte()?;

        if read != byte {
            self.write_byte(CAN)?;
            if read == CAN {
                return Err(ModemError::new(ErrorKind::ConnectionAborted));
            }
            return Err(ModemError::new(ErrorKind::InvalidData));
        }

        Ok(byte)
    }

    /// Reads a single byte from the inner I/O stream and compares it to `byte`.
    /// If they differ, an error of `InvalidData` with the message `expected` is
    /// returned. Otherwise the byte is returned. If `byte` is not `CAN` and the
    /// read byte is `CAN`, a `ConnectionAborted` error is returned.
    ///
    /// # Errors
    ///
    /// Returns an error if reading from the inner stream fails, or if the read
    /// byte was not `byte`. If the read byte differed and was `CAN`, an error
    /// of `ConnectionAborted` is returned. Otherwise, the error kind is
    /// `InvalidData`.
    fn expect_byte(&mut self, byte: u8, expected: &'static str) -> MResult<u8> {
        let read = self.inner.read_byte()?;

        if read != byte {
            if read == CAN {
                return Err(ModemError::new(ErrorKind::ConnectionAborted));
            }
            return Err(ModemError::new(ErrorKind::InvalidData));
        }

        Ok(byte)
    }

    /// Reads (downloads) a single packet from the inner stream using the XMODEM
    /// protocol. On success, returns the number of bytes read (always 128).
    ///
    /// The progress callback is called with `Progress::Start` when reception
    /// for the first packet has started and subsequently with
    /// `Progress::Packet` when a packet is received successfully.
    ///
    /// # Errors
    ///
    /// Returns an error if reading or writing to the inner stream fails at any
    /// point. Also returns an error if the XMODEM protocol indicates an error.
    /// In particular, an `InvalidData` error is returned when:
    ///
    ///   * The sender's first byte for a packet isn't `EOT` or `SOH`.
    ///   * The sender doesn't send a second `EOT` after the first.
    ///   * The received packet numbers don't match the expected values.
    ///
    /// An error of kind `Interrupted` is returned if a packet checksum fails.
    ///
    /// An error of kind `ConnectionAborted` is returned if a `CAN` byte is
    /// received when not expected.
    ///
    /// An error of kind `UnexpectedEof` is returned if `buf.len() < 128`.
    pub fn read_packet(&mut self, buf: &mut [u8]) -> MResult<usize> {
        let mut bytes_read = 0;
        if buf.len() < 128 {
            return Err(ModemError::new(ErrorKind::UnexpectedEof))
        }

        if !self.started {
            self.write_byte(NAK)?;
            self.started = true;
            self.packet = 1;
        }

        match self.read_byte(true) {
            Ok(EOT) => {
                self.write_byte(NAK)?;
                self.expect_byte_or_cancel(EOT, "Expect second EOT")?;
                self.write_byte(ACK)?;
                return Ok(bytes_read);
                }
            Ok(SOH) => {
                self.expect_byte_or_cancel(self.packet, "Wrong packet number")?;
                self.expect_byte_or_cancel(255-self.packet, "One complement incorrect")?;
                
                let mut checksum: u8 = 0;
                for i in 0..128 {
                    buf[i] = self.read_byte(false)?;
                    let (n, _) = checksum.overflowing_add(buf[i]);
                    checksum = n;
                    bytes_read += 1;
                }
                match self.read_byte(false) {
                    Ok(checksum_byte) if checksum_byte == checksum => {
                        self.write_byte(ACK)?;
                        self.packet +=1;
                        Ok(128)
                    },
                    _ => {
                        self.write_byte(NAK)?;
                        return Err(ModemError::new(ErrorKind::Interrupted))
                    }
                }
            }
            Ok(CAN) => Err(ModemError::new(ErrorKind::ConnectionAborted)),
            Ok(_) => Err(ModemError::new(ErrorKind::InvalidData)),
            Err(e) => Err(e),
        }
    }
    // Sends (uploads) a single packet to the inner stream using the XMODEM
    // protocol. If `buf` is empty, end of transmissions is sent. Users of this
    // interface should ensure that `write_packet(&[])` is called when data
    // transmission is complete. On success, returns the number of bytes
    // written.
    //
    // The progress callback is called with `Progress::Waiting` before waiting
    // for the receiver's `NAK`, `Progress::Start` when transmission of the
    // first packet has started and subsequently with `Progress::Packet` when a
    // packet is sent successfully.
    //
    // # Errors
    //
    // Returns an error if reading or writing to the inner stream fails at any
    // point. Also returns an error if the XMODEM protocol indicates an error.
    // In particular, an `InvalidData` error is returned when:
    //
    //   * The receiver's first byte isn't a `NAK`.
    //   * The receiver doesn't respond with a `NAK` to the first `EOT`.
    //   * The receiver doesn't respond with an `ACK` to the second `EOT`.
    //   * The receiver responds to a complete packet with something besides
    //     `ACK` or `NAK`.
    //
    // An error of kind `UnexpectedEof` is returned if `buf.len() < 128 &&
    // buf.len() != 0`.
    //
    // An error of kind `ConnectionAborted` is returned if a `CAN` byte is
    // received when not expected.
    //
    // An error of kind `Interrupted` is returned if a packet checksum fails.
    // pub fn write_packet(&mut self, buf: &[u8]) -> MResult<usize> {
    //     let mut bytes_written = 0;
    //     if buf.len() < 128 && buf.len() != 0 {
    //         return Err(io::Error::new(io::ErrorKind::UnexpectedEof,"invalid buf length"))
    //     }

    //     if !self.started {
    //         (self.progress)(Progress::Waiting);
    //         self.expect_byte(NAK,"expected starting NAK")?;
    //         self.started = true;
    //         self.packet = 1;
    //         (self.progress)(Progress::Started);
    //     }

    //     if buf.is_empty() {
    //         self.write_byte(EOT)?;
    //         self.expect_byte(NAK, "expect NAK for EOT")?;
    //         self.write_byte(EOT)?;
    //         self.expect_byte(ACK, "expect ACK for second EOT")?;
    //         bytes_written += 2;
    //         return Ok(bytes_written);
    //     }

    //     self.write_byte(SOH)?;
    //     self.write_byte(self.packet)?;
    //     self.write_byte(255-self.packet)?;
    //     bytes_written += 3;

    //     let mut checksum: u8 = 0;
    //     for b in buf {
    //         self.write_byte(*b)?;
    //         bytes_written += 1;
    //         let (n, _) = checksum.overflowing_add(*b);
    //         checksum = n;
    //     }
    //     self.write_byte(checksum)?;
    //     bytes_written +=1;
    //     match self.read_byte(true) {
    //         Ok(ACK) => {
    //             (self.progress)(Progress::Packet(self.packet));
    //             self.packet += 1;
    //             Ok(bytes_written)
    //         }
    //         Ok(NAK) => Err(io::Error::new(io::ErrorKind::Interrupted, "ACK expected for checksum")),
    //         Ok(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "No ACK or NAK for checksum")),
    //         Err(e) => Err(e)
    //     }
    // }

    // Flush this output stream, ensuring that all intermediately buffered
    // contents reach their destination.
    //
    // # Errors
    //
    // It is considered an error if not all bytes could be written due to I/O
    // errors or EOF being reached.
//     pub fn flush(&mut self) -> MResult<()> {
//         self.inner.flush()
//     }
}
