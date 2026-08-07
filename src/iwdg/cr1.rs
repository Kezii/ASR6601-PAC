#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `RESET_REQ_INT_EN` reader - Reset req int en"]
pub type ResetReqIntEnR = crate::BitReader;
#[doc = "Field `RESET_REQ_INT_EN` writer - Reset req int en"]
pub type ResetReqIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_REQ_RST_EN` reader - Reset req rst en"]
pub type ResetReqRstEnR = crate::BitReader;
#[doc = "Field `RESET_REQ_RST_EN` writer - Reset req rst en"]
pub type ResetReqRstEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reset req int en"]
    #[inline(always)]
    pub fn reset_req_int_en(&self) -> ResetReqIntEnR {
        ResetReqIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset req rst en"]
    #[inline(always)]
    pub fn reset_req_rst_en(&self) -> ResetReqRstEnR {
        ResetReqRstEnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset req int en"]
    #[inline(always)]
    pub fn reset_req_int_en(&mut self) -> ResetReqIntEnW<'_, Cr1Spec> {
        ResetReqIntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset req rst en"]
    #[inline(always)]
    pub fn reset_req_rst_en(&mut self) -> ResetReqRstEnW<'_, Cr1Spec> {
        ResetReqRstEnW::new(self, 1)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
