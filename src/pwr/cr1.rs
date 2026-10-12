#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `TRIM` reader - Regulator trim value"]
pub type TrimR = crate::FieldReader;
#[doc = "Field `TRIM` writer - Regulator trim value"]
pub type TrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LP_MODE_EXT` reader - Low-power mode extension, Stop3 versus Standby select"]
pub type LpModeExtR = crate::BitReader;
#[doc = "Field `LP_MODE_EXT` writer - Low-power mode extension, Stop3 versus Standby select"]
pub type LpModeExtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 20:23 - Regulator trim value"]
    #[inline(always)]
    pub fn trim(&self) -> TrimR {
        TrimR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Low-power mode extension, Stop3 versus Standby select"]
    #[inline(always)]
    pub fn lp_mode_ext(&self) -> LpModeExtR {
        LpModeExtR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 20:23 - Regulator trim value"]
    #[inline(always)]
    pub fn trim(&mut self) -> TrimW<'_, Cr1Spec> {
        TrimW::new(self, 20)
    }
    #[doc = "Bit 24 - Low-power mode extension, Stop3 versus Standby select"]
    #[inline(always)]
    pub fn lp_mode_ext(&mut self) -> LpModeExtW<'_, Cr1Spec> {
        LpModeExtW::new(self, 24)
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
