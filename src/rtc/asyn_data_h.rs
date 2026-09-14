#[doc = "Register `ASYN_DATA_H` reader"]
pub type R = crate::R<AsynDataHSpec>;
#[doc = "Field `SYN_DATA_H` reader - Synchronization time year/month/date"]
pub type SynDataHR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - Synchronization time year/month/date"]
    #[inline(always)]
    pub fn syn_data_h(&self) -> SynDataHR {
        SynDataHR::new(self.bits & 0x003f_ffff)
    }
}
#[doc = "asynchronization time year/month/date\n\nYou can [`read`](crate::Reg::read) this register and get [`asyn_data_h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AsynDataHSpec;
impl crate::RegisterSpec for AsynDataHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`asyn_data_h::R`](R) reader structure"]
impl crate::Readable for AsynDataHSpec {}
