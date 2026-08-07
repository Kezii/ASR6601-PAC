#[doc = "Register `DIER` reader"]
pub type R = crate::R<DierSpec>;
#[doc = "Register `DIER` writer"]
pub type W = crate::W<DierSpec>;
#[doc = "Field `UIE` reader - Uie"]
pub type UieR = crate::BitReader;
#[doc = "Field `UIE` writer - Uie"]
pub type UieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC0IE` reader - Cc0ie"]
pub type Cc0ieR = crate::BitReader;
#[doc = "Field `CC0IE` writer - Cc0ie"]
pub type Cc0ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1IE` reader - Cc1ie"]
pub type Cc1ieR = crate::BitReader;
#[doc = "Field `CC1IE` writer - Cc1ie"]
pub type Cc1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2IE` reader - Cc2ie"]
pub type Cc2ieR = crate::BitReader;
#[doc = "Field `CC2IE` writer - Cc2ie"]
pub type Cc2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3IE` reader - Cc3ie"]
pub type Cc3ieR = crate::BitReader;
#[doc = "Field `CC3IE` writer - Cc3ie"]
pub type Cc3ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE` reader - Tie"]
pub type TieR = crate::BitReader;
#[doc = "Field `TIE` writer - Tie"]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDE` reader - Ude"]
pub type UdeR = crate::BitReader;
#[doc = "Field `UDE` writer - Ude"]
pub type UdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC0DE` reader - Cc0de"]
pub type Cc0deR = crate::BitReader;
#[doc = "Field `CC0DE` writer - Cc0de"]
pub type Cc0deW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1DE` reader - Cc1de"]
pub type Cc1deR = crate::BitReader;
#[doc = "Field `CC1DE` writer - Cc1de"]
pub type Cc1deW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2DE` reader - Cc2de"]
pub type Cc2deR = crate::BitReader;
#[doc = "Field `CC2DE` writer - Cc2de"]
pub type Cc2deW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3DE` reader - Cc3de"]
pub type Cc3deR = crate::BitReader;
#[doc = "Field `CC3DE` writer - Cc3de"]
pub type Cc3deW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDE` reader - Tde"]
pub type TdeR = crate::BitReader;
#[doc = "Field `TDE` writer - Tde"]
pub type TdeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Uie"]
    #[inline(always)]
    pub fn uie(&self) -> UieR {
        UieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Cc0ie"]
    #[inline(always)]
    pub fn cc0ie(&self) -> Cc0ieR {
        Cc0ieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cc1ie"]
    #[inline(always)]
    pub fn cc1ie(&self) -> Cc1ieR {
        Cc1ieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Cc2ie"]
    #[inline(always)]
    pub fn cc2ie(&self) -> Cc2ieR {
        Cc2ieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Cc3ie"]
    #[inline(always)]
    pub fn cc3ie(&self) -> Cc3ieR {
        Cc3ieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Tie"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Ude"]
    #[inline(always)]
    pub fn ude(&self) -> UdeR {
        UdeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Cc0de"]
    #[inline(always)]
    pub fn cc0de(&self) -> Cc0deR {
        Cc0deR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Cc1de"]
    #[inline(always)]
    pub fn cc1de(&self) -> Cc1deR {
        Cc1deR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Cc2de"]
    #[inline(always)]
    pub fn cc2de(&self) -> Cc2deR {
        Cc2deR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Cc3de"]
    #[inline(always)]
    pub fn cc3de(&self) -> Cc3deR {
        Cc3deR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Tde"]
    #[inline(always)]
    pub fn tde(&self) -> TdeR {
        TdeR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Uie"]
    #[inline(always)]
    pub fn uie(&mut self) -> UieW<'_, DierSpec> {
        UieW::new(self, 0)
    }
    #[doc = "Bit 1 - Cc0ie"]
    #[inline(always)]
    pub fn cc0ie(&mut self) -> Cc0ieW<'_, DierSpec> {
        Cc0ieW::new(self, 1)
    }
    #[doc = "Bit 2 - Cc1ie"]
    #[inline(always)]
    pub fn cc1ie(&mut self) -> Cc1ieW<'_, DierSpec> {
        Cc1ieW::new(self, 2)
    }
    #[doc = "Bit 3 - Cc2ie"]
    #[inline(always)]
    pub fn cc2ie(&mut self) -> Cc2ieW<'_, DierSpec> {
        Cc2ieW::new(self, 3)
    }
    #[doc = "Bit 4 - Cc3ie"]
    #[inline(always)]
    pub fn cc3ie(&mut self) -> Cc3ieW<'_, DierSpec> {
        Cc3ieW::new(self, 4)
    }
    #[doc = "Bit 6 - Tie"]
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<'_, DierSpec> {
        TieW::new(self, 6)
    }
    #[doc = "Bit 8 - Ude"]
    #[inline(always)]
    pub fn ude(&mut self) -> UdeW<'_, DierSpec> {
        UdeW::new(self, 8)
    }
    #[doc = "Bit 9 - Cc0de"]
    #[inline(always)]
    pub fn cc0de(&mut self) -> Cc0deW<'_, DierSpec> {
        Cc0deW::new(self, 9)
    }
    #[doc = "Bit 10 - Cc1de"]
    #[inline(always)]
    pub fn cc1de(&mut self) -> Cc1deW<'_, DierSpec> {
        Cc1deW::new(self, 10)
    }
    #[doc = "Bit 11 - Cc2de"]
    #[inline(always)]
    pub fn cc2de(&mut self) -> Cc2deW<'_, DierSpec> {
        Cc2deW::new(self, 11)
    }
    #[doc = "Bit 12 - Cc3de"]
    #[inline(always)]
    pub fn cc3de(&mut self) -> Cc3deW<'_, DierSpec> {
        Cc3deW::new(self, 12)
    }
    #[doc = "Bit 14 - Tde"]
    #[inline(always)]
    pub fn tde(&mut self) -> TdeW<'_, DierSpec> {
        TdeW::new(self, 14)
    }
}
#[doc = "TIMER DMA/interrupt enable register, Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dier::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DierSpec;
impl crate::RegisterSpec for DierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dier::R`](R) reader structure"]
impl crate::Readable for DierSpec {}
#[doc = "`write(|w| ..)` method takes [`dier::W`](W) writer structure"]
impl crate::Writable for DierSpec {
    type Safety = crate::Unsafe;
}
