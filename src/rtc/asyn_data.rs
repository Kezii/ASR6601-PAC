#[doc = "Register `ASYN_DATA` reader"]
pub type R = crate::R<AsynDataSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "asynchronization time hour/minute/second\n\nYou can [`read`](crate::Reg::read) this register and get [`asyn_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AsynDataSpec;
impl crate::RegisterSpec for AsynDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`asyn_data::R`](R) reader structure"]
impl crate::Readable for AsynDataSpec {}
