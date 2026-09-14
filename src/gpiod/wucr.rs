#[doc = "Register `WUCR` reader"]
pub type R = crate::R<WucrSpec>;
#[doc = "Register `WUCR` writer"]
pub type W = crate::W<WucrSpec>;
#[doc = "Field `WU_EN` reader - pin\\[15:0\\] wakeup enable from sleep/stop0-2"]
pub type WuEnR = crate::FieldReader<u16>;
#[doc = "Field `WU_EN` writer - pin\\[15:0\\] wakeup enable from sleep/stop0-2"]
pub type WuEnW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - pin\\[15:0\\] wakeup enable from sleep/stop0-2"]
    #[inline(always)]
    pub fn wu_en(&self) -> WuEnR {
        WuEnR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - pin\\[15:0\\] wakeup enable from sleep/stop0-2"]
    #[inline(always)]
    pub fn wu_en(&mut self) -> WuEnW<'_, WucrSpec> {
        WuEnW::new(self, 0)
    }
}
#[doc = "wakeup control register\n\nYou can [`read`](crate::Reg::read) this register and get [`wucr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wucr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WucrSpec;
impl crate::RegisterSpec for WucrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wucr::R`](R) reader structure"]
impl crate::Readable for WucrSpec {}
#[doc = "`write(|w| ..)` method takes [`wucr::W`](W) writer structure"]
impl crate::Writable for WucrSpec {
    type Safety = crate::Unsafe;
}
