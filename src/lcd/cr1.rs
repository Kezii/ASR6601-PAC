#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `DONE_STATUS` reader - Done status"]
pub type DoneStatusR = crate::BitReader;
#[doc = "Field `DONE_STATUS` writer - Done status"]
pub type DoneStatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Done status"]
    #[inline(always)]
    pub fn done_status(&self) -> DoneStatusR {
        DoneStatusR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Done status"]
    #[inline(always)]
    pub fn done_status(&mut self) -> DoneStatusW<'_, Cr1Spec> {
        DoneStatusW::new(self, 1)
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
