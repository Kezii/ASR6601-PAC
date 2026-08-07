#[doc = "Register `SUB_SECOND_CNT` reader"]
pub type R = crate::R<SubSecondCntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "subsecond counter\n\nYou can [`read`](crate::Reg::read) this register and get [`sub_second_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SubSecondCntSpec;
impl crate::RegisterSpec for SubSecondCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sub_second_cnt::R`](R) reader structure"]
impl crate::Readable for SubSecondCntSpec {}
