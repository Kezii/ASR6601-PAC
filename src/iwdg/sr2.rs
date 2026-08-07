#[doc = "Register `SR2` reader"]
pub type R = crate::R<Sr2Spec>;
#[doc = "Register `SR2` writer"]
pub type W = crate::W<Sr2Spec>;
#[doc = "Field `RESET_REQ_SR` reader - Reset req sr"]
pub type ResetReqSrR = crate::BitReader;
#[doc = "Field `RESET_REQ_SR` writer - Reset req sr"]
pub type ResetReqSrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reset req sr"]
    #[inline(always)]
    pub fn reset_req_sr(&self) -> ResetReqSrR {
        ResetReqSrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset req sr"]
    #[inline(always)]
    pub fn reset_req_sr(&mut self) -> ResetReqSrW<'_, Sr2Spec> {
        ResetReqSrW::new(self, 0)
    }
}
#[doc = "status register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sr2::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sr2Spec;
impl crate::RegisterSpec for Sr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr2::R`](R) reader structure"]
impl crate::Readable for Sr2Spec {}
#[doc = "`write(|w| ..)` method takes [`sr2::W`](W) writer structure"]
impl crate::Writable for Sr2Spec {
    type Safety = crate::Unsafe;
}
