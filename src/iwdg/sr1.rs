#[doc = "Register `SR1` reader"]
pub type R = crate::R<Sr1Spec>;
#[doc = "Field `RESET_REQ_SYNC` reader - Reset req sync"]
pub type ResetReqSyncR = crate::BitReader;
impl R {
    #[doc = "Bit 12 - Reset req sync"]
    #[inline(always)]
    pub fn reset_req_sync(&self) -> ResetReqSyncR {
        ResetReqSyncR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "status register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sr1Spec;
impl crate::RegisterSpec for Sr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr1::R`](R) reader structure"]
impl crate::Readable for Sr1Spec {}
