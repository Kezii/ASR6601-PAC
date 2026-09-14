#[doc = "Register `WULVL` reader"]
pub type R = crate::R<WulvlSpec>;
#[doc = "Register `WULVL` writer"]
pub type W = crate::W<WulvlSpec>;
#[doc = "Field `WU_LVL` reader - pin\\[15:0\\] wakeup level from sleep/stop0-2"]
pub type WuLvlR = crate::FieldReader<u16>;
#[doc = "Field `WU_LVL` writer - pin\\[15:0\\] wakeup level from sleep/stop0-2"]
pub type WuLvlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - pin\\[15:0\\] wakeup level from sleep/stop0-2"]
    #[inline(always)]
    pub fn wu_lvl(&self) -> WuLvlR {
        WuLvlR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - pin\\[15:0\\] wakeup level from sleep/stop0-2"]
    #[inline(always)]
    pub fn wu_lvl(&mut self) -> WuLvlW<'_, WulvlSpec> {
        WuLvlW::new(self, 0)
    }
}
#[doc = "wakeup level register\n\nYou can [`read`](crate::Reg::read) this register and get [`wulvl::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wulvl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WulvlSpec;
impl crate::RegisterSpec for WulvlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wulvl::R`](R) reader structure"]
impl crate::Readable for WulvlSpec {}
#[doc = "`write(|w| ..)` method takes [`wulvl::W`](W) writer structure"]
impl crate::Writable for WulvlSpec {
    type Safety = crate::Unsafe;
}
