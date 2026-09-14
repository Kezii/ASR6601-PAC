#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `RTC_RET_SRAM_ERASE_EN` reader - Rtc retention sram erase enable"]
pub type RtcRetSramEraseEnR = crate::FieldReader;
#[doc = "Field `RTC_RET_SRAM_ERASE_EN` writer - Rtc retention sram erase enable"]
pub type RtcRetSramEraseEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RTC_OUT_SEL` reader - Rtc output selection"]
pub type RtcOutSelR = crate::FieldReader;
#[doc = "Field `RTC_OUT_SEL` writer - Rtc output selection"]
pub type RtcOutSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RTC_OUT_POL` reader - Rtc output polarity"]
pub type RtcOutPolR = crate::BitReader;
#[doc = "Field `RTC_OUT_POL` writer - Rtc output polarity"]
pub type RtcOutPolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Rtc retention sram erase enable"]
    #[inline(always)]
    pub fn rtc_ret_sram_erase_en(&self) -> RtcRetSramEraseEnR {
        RtcRetSramEraseEnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Rtc output selection"]
    #[inline(always)]
    pub fn rtc_out_sel(&self) -> RtcOutSelR {
        RtcOutSelR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Rtc output polarity"]
    #[inline(always)]
    pub fn rtc_out_pol(&self) -> RtcOutPolR {
        RtcOutPolR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Rtc retention sram erase enable"]
    #[inline(always)]
    pub fn rtc_ret_sram_erase_en(&mut self) -> RtcRetSramEraseEnW<'_, Cr2Spec> {
        RtcRetSramEraseEnW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Rtc output selection"]
    #[inline(always)]
    pub fn rtc_out_sel(&mut self) -> RtcOutSelW<'_, Cr2Spec> {
        RtcOutSelW::new(self, 4)
    }
    #[doc = "Bit 7 - Rtc output polarity"]
    #[inline(always)]
    pub fn rtc_out_pol(&mut self) -> RtcOutPolW<'_, Cr2Spec> {
        RtcOutPolW::new(self, 7)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
