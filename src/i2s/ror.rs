#[doc = "Register `ROR` reader"]
pub type R = crate::R<RorSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "receiver overrun register\n\nYou can [`read`](crate::Reg::read) this register and get [`ror::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RorSpec;
impl crate::RegisterSpec for RorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ror::R`](R) reader structure"]
impl crate::Readable for RorSpec {}
