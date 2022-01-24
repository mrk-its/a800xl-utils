use crate::cio::{find_available_channel, Cmd, Mode, IOCB};

pub struct File {
    pub channel: u8,
}

impl File {
    pub fn create(filename: &[u8]) -> Result<File, u8> {
        File::open_with_mode(filename, Mode::Write)
    }

    pub fn open(filename: &[u8]) -> Result<File, u8> {
        File::open_with_mode(filename, Mode::Read)
    }

    pub fn open_with_mode(filename: &[u8], mode: Mode) -> Result<File, u8> {
        let channel = find_available_channel()?;
        let ret = IOCB::new(channel)
            .cmd(Cmd::Open as u8)
            .buffer(filename)
            .icax1(mode as u8)
            .icax2(0)
            .call();
        if ret >= 128 {
            return Err(ret);
        }
        Ok(File { channel })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), u8> {
        let ret = IOCB::new(self.channel)
            .cmd(Cmd::PutBytes as u8)
            .buffer(data)
            .call();
        if ret >= 128 {
            return Err(ret);
        }
        Ok(())
    }

    pub fn read(&mut self, data: &mut [u8]) -> Result<usize, u8> {
        let mut iocb = IOCB::new(self.channel);
        iocb.cmd(Cmd::GetBytes as u8).buffer(data);
        let ret = iocb.call();
        if ret >= 128 && ret != 136 {
            return Err(ret);
        }
        Ok(iocb.get_buf_len())
    }

    pub fn close(&mut self) {
        if self.channel < 128 {
            IOCB::new(self.channel).cmd(Cmd::Close as u8).call();
            self.channel |= 128; // mark as closed
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        self.close();
    }
}
