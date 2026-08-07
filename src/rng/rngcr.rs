#[doc = "Register `RNGCR` reader"]
pub type R = crate::R<RngcrSpec>;
#[doc = "Register `RNGCR` writer"]
pub type W = crate::W<RngcrSpec>;
#[doc = "Field `TRUE_RANDOM_MODE` reader - True random mode"]
pub type TrueRandomModeR = crate::BitReader;
#[doc = "Field `TRUE_RANDOM_MODE` writer - True random mode"]
pub type TrueRandomModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - True random mode"]
    #[inline(always)]
    pub fn true_random_mode(&self) -> TrueRandomModeR {
        TrueRandomModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - True random mode"]
    #[inline(always)]
    pub fn true_random_mode(&mut self) -> TrueRandomModeW<'_, RngcrSpec> {
        TrueRandomModeW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, RngcrSpec> {
        EnableW::new(self, 7)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngcr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngcrSpec;
impl crate::RegisterSpec for RngcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngcr::R`](R) reader structure"]
impl crate::Readable for RngcrSpec {}
#[doc = "`write(|w| ..)` method takes [`rngcr::W`](W) writer structure"]
impl crate::Writable for RngcrSpec {
    type Safety = crate::Unsafe;
}
