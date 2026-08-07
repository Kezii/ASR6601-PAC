#[doc = "Register `DOR` reader"]
pub type R = crate::R<DorSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "data output register\n\nYou can [`read`](crate::Reg::read) this register and get [`dor::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DorSpec;
impl crate::RegisterSpec for DorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dor::R`](R) reader structure"]
impl crate::Readable for DorSpec {}
