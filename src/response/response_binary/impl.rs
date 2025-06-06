use crate::*;

impl ResponseTrait for TcpResponseBinary {
    type OutputText = TcpResponseText;
    type OutputBinary = TcpResponseBinary;

    fn from(response: &[u8]) -> Self
    where
        Self: Sized,
    {
        response.to_vec()
    }

    fn binary(&self) -> Self::OutputBinary {
        self.clone()
    }

    fn text(&self) -> TcpResponseText {
        let data: String = String::from_utf8_lossy(&self).to_string();
        data
    }
}
