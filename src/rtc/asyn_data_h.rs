#[doc = "Register `ASYN_DATA_H` reader"]
pub type R = crate::R<AsynDataHSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "asynchronization time year/month/date\n\nYou can [`read`](crate::Reg::read) this register and get [`asyn_data_h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AsynDataHSpec;
impl crate::RegisterSpec for AsynDataHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`asyn_data_h::R`](R) reader structure"]
impl crate::Readable for AsynDataHSpec {}
