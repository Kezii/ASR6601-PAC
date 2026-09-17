#[doc = "Register `WU_EN` reader"]
pub type R = crate::R<WuEnSpec>;
#[doc = "Register `WU_EN` writer"]
pub type W = crate::W<WuEnSpec>;
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
    pub fn wu_en(&mut self) -> WuEnW<'_, WuEnSpec> {
        WuEnW::new(self, 0)
    }
}
#[doc = "wakeup control register\n\nYou can [`read`](crate::Reg::read) this register and get [`wu_en::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wu_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WuEnSpec;
impl crate::RegisterSpec for WuEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wu_en::R`](R) reader structure"]
impl crate::Readable for WuEnSpec {}
#[doc = "`write(|w| ..)` method takes [`wu_en::W`](W) writer structure"]
impl crate::Writable for WuEnSpec {
    type Safety = crate::Unsafe;
}
