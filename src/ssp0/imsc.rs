#[doc = "Register `IMSC` reader"]
pub type R = crate::R<ImscSpec>;
#[doc = "Register `IMSC` writer"]
pub type W = crate::W<ImscSpec>;
#[doc = "Field `RORIM` reader - receive overrun interrupt mask"]
pub type RorimR = crate::BitReader;
#[doc = "Field `RORIM` writer - receive overrun interrupt mask"]
pub type RorimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIM` reader - receive timeout interrupt mask"]
pub type RtimR = crate::BitReader;
#[doc = "Field `RTIM` writer - receive timeout interrupt mask"]
pub type RtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIM` reader - receive interrupt mask"]
pub type RximR = crate::BitReader;
#[doc = "Field `RXIM` writer - receive interrupt mask"]
pub type RximW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIM` reader - transmit interrupt mask"]
pub type TximR = crate::BitReader;
#[doc = "Field `TXIM` writer - transmit interrupt mask"]
pub type TximW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - receive overrun interrupt mask"]
    #[inline(always)]
    pub fn rorim(&self) -> RorimR {
        RorimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - receive timeout interrupt mask"]
    #[inline(always)]
    pub fn rtim(&self) -> RtimR {
        RtimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - receive interrupt mask"]
    #[inline(always)]
    pub fn rxim(&self) -> RximR {
        RximR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - transmit interrupt mask"]
    #[inline(always)]
    pub fn txim(&self) -> TximR {
        TximR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - receive overrun interrupt mask"]
    #[inline(always)]
    pub fn rorim(&mut self) -> RorimW<'_, ImscSpec> {
        RorimW::new(self, 0)
    }
    #[doc = "Bit 1 - receive timeout interrupt mask"]
    #[inline(always)]
    pub fn rtim(&mut self) -> RtimW<'_, ImscSpec> {
        RtimW::new(self, 1)
    }
    #[doc = "Bit 2 - receive interrupt mask"]
    #[inline(always)]
    pub fn rxim(&mut self) -> RximW<'_, ImscSpec> {
        RximW::new(self, 2)
    }
    #[doc = "Bit 3 - transmit interrupt mask"]
    #[inline(always)]
    pub fn txim(&mut self) -> TximW<'_, ImscSpec> {
        TximW::new(self, 3)
    }
}
#[doc = "interrupt mask set or clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`imsc::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
