#[doc = "Register `ALARM0` reader"]
pub type R = crate::R<Alarm0Spec>;
#[doc = "Register `ALARM0` writer"]
pub type W = crate::W<Alarm0Spec>;
#[doc = "Field `ALARM0_VALUE` reader - Alarm 0 value"]
pub type Alarm0ValueR = crate::FieldReader<u32>;
#[doc = "Field `ALARM0_VALUE` writer - Alarm 0 value"]
pub type Alarm0ValueW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
#[doc = "Field `ALARM0_MASK` reader - Alarm 0 mask"]
pub type Alarm0MaskR = crate::FieldReader;
#[doc = "Field `ALARM0_MASK` writer - Alarm 0 mask"]
pub type Alarm0MaskW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ALARM0_WEEK_SEL` reader - Alarm 0 date or day of week selection"]
pub type Alarm0WeekSelR = crate::BitReader;
#[doc = "Field `ALARM0_WEEK_SEL` writer - Alarm 0 date or day of week selection"]
pub type Alarm0WeekSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALARM0_EN` reader - Alarm 0 enable"]
pub type Alarm0EnR = crate::BitReader;
#[doc = "Field `ALARM0_EN` writer - Alarm 0 enable"]
pub type Alarm0EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:25 - Alarm 0 value"]
    #[inline(always)]
    pub fn alarm0_value(&self) -> Alarm0ValueR {
        Alarm0ValueR::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bits 26:29 - Alarm 0 mask"]
    #[inline(always)]
    pub fn alarm0_mask(&self) -> Alarm0MaskR {
        Alarm0MaskR::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Alarm 0 date or day of week selection"]
    #[inline(always)]
    pub fn alarm0_week_sel(&self) -> Alarm0WeekSelR {
        Alarm0WeekSelR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Alarm 0 enable"]
    #[inline(always)]
    pub fn alarm0_en(&self) -> Alarm0EnR {
        Alarm0EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:25 - Alarm 0 value"]
    #[inline(always)]
    pub fn alarm0_value(&mut self) -> Alarm0ValueW<'_, Alarm0Spec> {
        Alarm0ValueW::new(self, 0)
    }
    #[doc = "Bits 26:29 - Alarm 0 mask"]
    #[inline(always)]
    pub fn alarm0_mask(&mut self) -> Alarm0MaskW<'_, Alarm0Spec> {
        Alarm0MaskW::new(self, 26)
    }
    #[doc = "Bit 30 - Alarm 0 date or day of week selection"]
    #[inline(always)]
    pub fn alarm0_week_sel(&mut self) -> Alarm0WeekSelW<'_, Alarm0Spec> {
        Alarm0WeekSelW::new(self, 30)
    }
    #[doc = "Bit 31 - Alarm 0 enable"]
    #[inline(always)]
    pub fn alarm0_en(&mut self) -> Alarm0EnW<'_, Alarm0Spec> {
        Alarm0EnW::new(self, 31)
    }
}
#[doc = "alarm 0\n\nYou can [`read`](crate::Reg::read) this register and get [`alarm0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Alarm0Spec;
impl crate::RegisterSpec for Alarm0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarm0::R`](R) reader structure"]
impl crate::Readable for Alarm0Spec {}
#[doc = "`write(|w| ..)` method takes [`alarm0::W`](W) writer structure"]
impl crate::Writable for Alarm0Spec {
    type Safety = crate::Unsafe;
}
