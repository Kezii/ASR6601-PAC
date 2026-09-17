#[doc = "Register `RNGDET` reader"]
pub type R = crate::R<RngdetSpec>;
#[doc = "Register `RNGDET` writer"]
pub type W = crate::W<RngdetSpec>;
#[doc = "Field `DATA_CLOCK_ENABLE` reader - Data clock enable"]
pub type DataClockEnableR = crate::BitReader;
#[doc = "Field `DATA_CLOCK_ENABLE` writer - Data clock enable"]
pub type DataClockEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POKER_ENABLE` reader - Poker enable"]
pub type PokerEnableR = crate::BitReader;
#[doc = "Field `POKER_ENABLE` writer - Poker enable"]
pub type PokerEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POKER_SOURCE_DATA` reader - Poker source data"]
pub type PokerSourceDataR = crate::BitReader;
#[doc = "Field `POKER_SOURCE_DATA` writer - Poker source data"]
pub type PokerSourceDataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLOW_REINITIALIZE` reader - Flow reinitialize"]
pub type FlowReinitializeR = crate::BitReader;
#[doc = "Field `FLOW_REINITIALIZE` writer - Flow reinitialize"]
pub type FlowReinitializeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data clock enable"]
    #[inline(always)]
    pub fn data_clock_enable(&self) -> DataClockEnableR {
        DataClockEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Poker enable"]
    #[inline(always)]
    pub fn poker_enable(&self) -> PokerEnableR {
        PokerEnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Poker source data"]
    #[inline(always)]
    pub fn poker_source_data(&self) -> PokerSourceDataR {
        PokerSourceDataR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Flow reinitialize"]
    #[inline(always)]
    pub fn flow_reinitialize(&self) -> FlowReinitializeR {
        FlowReinitializeR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data clock enable"]
    #[inline(always)]
    pub fn data_clock_enable(&mut self) -> DataClockEnableW<'_, RngdetSpec> {
        DataClockEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Poker enable"]
    #[inline(always)]
    pub fn poker_enable(&mut self) -> PokerEnableW<'_, RngdetSpec> {
        PokerEnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Poker source data"]
    #[inline(always)]
    pub fn poker_source_data(&mut self) -> PokerSourceDataW<'_, RngdetSpec> {
        PokerSourceDataW::new(self, 2)
    }
    #[doc = "Bit 3 - Flow reinitialize"]
    #[inline(always)]
    pub fn flow_reinitialize(&mut self) -> FlowReinitializeW<'_, RngdetSpec> {
        FlowReinitializeW::new(self, 3)
    }
}
#[doc = "Detection control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rngdet::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngdet::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngdetSpec;
impl crate::RegisterSpec for RngdetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngdet::R`](R) reader structure"]
impl crate::Readable for RngdetSpec {}
#[doc = "`write(|w| ..)` method takes [`rngdet::W`](W) writer structure"]
impl crate::Writable for RngdetSpec {
    type Safety = crate::Unsafe;
}
