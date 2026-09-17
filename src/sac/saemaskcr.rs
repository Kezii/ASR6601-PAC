#[doc = "Register `SAEMASKCR` reader"]
pub type R = crate::R<SaemaskcrSpec>;
#[doc = "Register `SAEMASKCR` writer"]
pub type W = crate::W<SaemaskcrSpec>;
#[doc = "Field `MASK_ENABLE` reader - Mask enable"]
pub type MaskEnableR = crate::BitReader;
#[doc = "Field `MASK_ENABLE` writer - Mask enable"]
pub type MaskEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask enable"]
    #[inline(always)]
    pub fn mask_enable(&self) -> MaskEnableR {
        MaskEnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask enable"]
    #[inline(always)]
    pub fn mask_enable(&mut self) -> MaskEnableW<'_, SaemaskcrSpec> {
        MaskEnableW::new(self, 0)
    }
}
#[doc = "Mask control register\n\nYou can [`read`](crate::Reg::read) this register and get [`saemaskcr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saemaskcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaemaskcrSpec;
impl crate::RegisterSpec for SaemaskcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saemaskcr::R`](R) reader structure"]
impl crate::Readable for SaemaskcrSpec {}
#[doc = "`write(|w| ..)` method takes [`saemaskcr::W`](W) writer structure"]
impl crate::Writable for SaemaskcrSpec {
    type Safety = crate::Unsafe;
}
