#[doc = "Register `IMSC` reader"]
pub type R = crate::R<ImscSpec>;
#[doc = "Register `IMSC` writer"]
pub type W = crate::W<ImscSpec>;
#[doc = "Field `RXIM` reader - receive interrupt mask control"]
pub type RximR = crate::BitReader;
#[doc = "Field `RXIM` writer - receive interrupt mask control"]
pub type RximW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIM` reader - transmit interrupt mask control"]
pub type TximR = crate::BitReader;
#[doc = "Field `TXIM` writer - transmit interrupt mask control"]
pub type TximW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIM` reader - receive timeout interrupt mask control"]
pub type RtimR = crate::BitReader;
#[doc = "Field `RTIM` writer - receive timeout interrupt mask control"]
pub type RtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEIM` reader - framing error interrupt mask control"]
pub type FeimR = crate::BitReader;
#[doc = "Field `FEIM` writer - framing error interrupt mask control"]
pub type FeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEIM` reader - parity error interrupt mask control"]
pub type PeimR = crate::BitReader;
#[doc = "Field `PEIM` writer - parity error interrupt mask control"]
pub type PeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEIM` reader - break error interrupt mask control"]
pub type BeimR = crate::BitReader;
#[doc = "Field `BEIM` writer - break error interrupt mask control"]
pub type BeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEIM` reader - overrun error interrupt mask control"]
pub type OeimR = crate::BitReader;
#[doc = "Field `OEIM` writer - overrun error interrupt mask control"]
pub type OeimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - receive interrupt mask control"]
    #[inline(always)]
    pub fn rxim(&self) -> RximR {
        RximR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - transmit interrupt mask control"]
    #[inline(always)]
    pub fn txim(&self) -> TximR {
        TximR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - receive timeout interrupt mask control"]
    #[inline(always)]
    pub fn rtim(&self) -> RtimR {
        RtimR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - framing error interrupt mask control"]
    #[inline(always)]
    pub fn feim(&self) -> FeimR {
        FeimR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - parity error interrupt mask control"]
    #[inline(always)]
    pub fn peim(&self) -> PeimR {
        PeimR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - break error interrupt mask control"]
    #[inline(always)]
    pub fn beim(&self) -> BeimR {
        BeimR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - overrun error interrupt mask control"]
    #[inline(always)]
    pub fn oeim(&self) -> OeimR {
        OeimR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - receive interrupt mask control"]
    #[inline(always)]
    pub fn rxim(&mut self) -> RximW<'_, ImscSpec> {
        RximW::new(self, 4)
    }
    #[doc = "Bit 5 - transmit interrupt mask control"]
    #[inline(always)]
    pub fn txim(&mut self) -> TximW<'_, ImscSpec> {
        TximW::new(self, 5)
    }
    #[doc = "Bit 6 - receive timeout interrupt mask control"]
    #[inline(always)]
    pub fn rtim(&mut self) -> RtimW<'_, ImscSpec> {
        RtimW::new(self, 6)
    }
    #[doc = "Bit 7 - framing error interrupt mask control"]
    #[inline(always)]
    pub fn feim(&mut self) -> FeimW<'_, ImscSpec> {
        FeimW::new(self, 7)
    }
    #[doc = "Bit 8 - parity error interrupt mask control"]
    #[inline(always)]
    pub fn peim(&mut self) -> PeimW<'_, ImscSpec> {
        PeimW::new(self, 8)
    }
    #[doc = "Bit 9 - break error interrupt mask control"]
    #[inline(always)]
    pub fn beim(&mut self) -> BeimW<'_, ImscSpec> {
        BeimW::new(self, 9)
    }
    #[doc = "Bit 10 - overrun error interrupt mask control"]
    #[inline(always)]
    pub fn oeim(&mut self) -> OeimW<'_, ImscSpec> {
        OeimW::new(self, 10)
    }
}
#[doc = "interrupt mask set/clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`imsc::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImscSpec;
impl crate::RegisterSpec for ImscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imsc::R`](R) reader structure"]
impl crate::Readable for ImscSpec {}
#[doc = "`write(|w| ..)` method takes [`imsc::W`](W) writer structure"]
impl crate::Writable for ImscSpec {
    type Safety = crate::Unsafe;
}
