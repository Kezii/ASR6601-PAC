#[doc = "Register `ALARM1` reader"]
pub type R = crate::R<Alarm1Spec>;
#[doc = "Register `ALARM1` writer"]
pub type W = crate::W<Alarm1Spec>;
#[doc = "Field `ALARM1_VALUE` reader - Alarm 1 value"]
pub type Alarm1ValueR = crate::FieldReader<u32>;
#[doc = "Field `ALARM1_VALUE` writer - Alarm 1 value"]
pub type Alarm1ValueW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
#[doc = "Field `ALARM1_MASK` reader - Alarm 1 mask"]
pub type Alarm1MaskR = crate::FieldReader;
#[doc = "Field `ALARM1_MASK` writer - Alarm 1 mask"]
pub type Alarm1MaskW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ALARM1_WEEK_SEL` reader - Alarm 1 date or day of week selection"]
pub type Alarm1WeekSelR = crate::BitReader;
#[doc = "Field `ALARM1_WEEK_SEL` writer - Alarm 1 date or day of week selection"]
pub type Alarm1WeekSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALARM1_EN` reader - Alarm 1 enable"]
pub type Alarm1EnR = crate::BitReader;
#[doc = "Field `ALARM1_EN` writer - Alarm 1 enable"]
pub type Alarm1EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:25 - Alarm 1 value"]
    #[inline(always)]
    pub fn alarm1_value(&self) -> Alarm1ValueR {
        Alarm1ValueR::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bits 26:29 - Alarm 1 mask"]
    #[inline(always)]
    pub fn alarm1_mask(&self) -> Alarm1MaskR {
        Alarm1MaskR::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Alarm 1 date or day of week selection"]
    #[inline(always)]
    pub fn alarm1_week_sel(&self) -> Alarm1WeekSelR {
        Alarm1WeekSelR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Alarm 1 enable"]
    #[inline(always)]
    pub fn alarm1_en(&self) -> Alarm1EnR {
        Alarm1EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:25 - Alarm 1 value"]
    #[inline(always)]
    pub fn alarm1_value(&mut self) -> Alarm1ValueW<'_, Alarm1Spec> {
        Alarm1ValueW::new(self, 0)
    }
    #[doc = "Bits 26:29 - Alarm 1 mask"]
    #[inline(always)]
    pub fn alarm1_mask(&mut self) -> Alarm1MaskW<'_, Alarm1Spec> {
        Alarm1MaskW::new(self, 26)
    }
    #[doc = "Bit 30 - Alarm 1 date or day of week selection"]
    #[inline(always)]
    pub fn alarm1_week_sel(&mut self) -> Alarm1WeekSelW<'_, Alarm1Spec> {
        Alarm1WeekSelW::new(self, 30)
    }
    #[doc = "Bit 31 - Alarm 1 enable"]
    #[inline(always)]
    pub fn alarm1_en(&mut self) -> Alarm1EnW<'_, Alarm1Spec> {
        Alarm1EnW::new(self, 31)
    }
}
#[doc = "alarm 1\n\nYou can [`read`](crate::Reg::read) this register and get [`alarm1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Alarm1Spec;
impl crate::RegisterSpec for Alarm1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarm1::R`](R) reader structure"]
impl crate::Readable for Alarm1Spec {}
#[doc = "`write(|w| ..)` method takes [`alarm1::W`](W) writer structure"]
impl crate::Writable for Alarm1Spec {
    type Safety = crate::Unsafe;
}
