#[doc = "Register `CYC_CNT` reader"]
pub type R = crate::R<CycCntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "cyc counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cyc_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CycCntSpec;
impl crate::RegisterSpec for CycCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cyc_cnt::R`](R) reader structure"]
impl crate::Readable for CycCntSpec {}
