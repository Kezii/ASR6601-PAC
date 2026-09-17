#[doc = "Register `ASYNDATA` reader"]
pub type R = crate::R<AsyndataSpec>;
#[doc = "Field `SYN_DATA` reader - Synchronization time hour/minute/second"]
pub type SynDataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - Synchronization time hour/minute/second"]
    #[inline(always)]
    pub fn syn_data(&self) -> SynDataR {
        SynDataR::new(self.bits & 0x000f_ffff)
    }
}
#[doc = "asynchronization time hour/minute/second\n\nYou can [`read`](crate::Reg::read) this register and get [`asyndata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AsyndataSpec;
impl crate::RegisterSpec for AsyndataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`asyndata::R`](R) reader structure"]
impl crate::Readable for AsyndataSpec {}
