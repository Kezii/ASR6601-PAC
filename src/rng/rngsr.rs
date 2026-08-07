#[doc = "Register `RNGSR` reader"]
pub type R = crate::R<RngsrSpec>;
#[doc = "Register `RNGSR` writer"]
pub type W = crate::W<RngsrSpec>;
#[doc = "Field `DATA_READY` reader - Data ready"]
pub type DataReadyR = crate::BitReader;
#[doc = "Field `DATA_READY` writer - Data ready"]
pub type DataReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_CLOCK_ERROR` reader - Data clock error"]
pub type DataClockErrorR = crate::BitReader;
#[doc = "Field `DATA_CLOCK_ERROR` writer - Data clock error"]
pub type DataClockErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POKER_CHECK_END` reader - Poker check end"]
pub type PokerCheckEndR = crate::BitReader;
#[doc = "Field `POKER_CHECK_END` writer - Poker check end"]
pub type PokerCheckEndW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data ready"]
    #[inline(always)]
    pub fn data_ready(&self) -> DataReadyR {
        DataReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data clock error"]
    #[inline(always)]
    pub fn data_clock_error(&self) -> DataClockErrorR {
        DataClockErrorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Poker check end"]
    #[inline(always)]
    pub fn poker_check_end(&self) -> PokerCheckEndR {
        PokerCheckEndR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data ready"]
    #[inline(always)]
    pub fn data_ready(&mut self) -> DataReadyW<'_, RngsrSpec> {
        DataReadyW::new(self, 0)
    }
    #[doc = "Bit 1 - Data clock error"]
    #[inline(always)]
    pub fn data_clock_error(&mut self) -> DataClockErrorW<'_, RngsrSpec> {
        DataClockErrorW::new(self, 1)
    }
    #[doc = "Bit 2 - Poker check end"]
    #[inline(always)]
    pub fn poker_check_end(&mut self) -> PokerCheckEndW<'_, RngsrSpec> {
        PokerCheckEndW::new(self, 2)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngsr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngsrSpec;
impl crate::RegisterSpec for RngsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngsr::R`](R) reader structure"]
impl crate::Readable for RngsrSpec {}
#[doc = "`write(|w| ..)` method takes [`rngsr::W`](W) writer structure"]
impl crate::Writable for RngsrSpec {
    type Safety = crate::Unsafe;
}
