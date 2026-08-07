#[doc = "Register `TOR` reader"]
pub type R = crate::R<TorSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "transmitter overrun register\n\nYou can [`read`](crate::Reg::read) this register and get [`tor::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TorSpec;
impl crate::RegisterSpec for TorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tor::R`](R) reader structure"]
impl crate::Readable for TorSpec {}
