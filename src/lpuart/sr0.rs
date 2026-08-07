#[doc = "Register `SR0` reader"]
pub type R = crate::R<Sr0Spec>;
#[doc = "Register `SR0` writer"]
pub type W = crate::W<Sr0Spec>;
#[doc = "Field `START_VALID_STATE` reader - Start valid state"]
pub type StartValidStateR = crate::BitReader;
#[doc = "Field `START_VALID_STATE` writer - Start valid state"]
pub type StartValidStateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DONE_STATE` reader - Rx done state"]
pub type RxDoneStateR = crate::BitReader;
#[doc = "Field `RX_DONE_STATE` writer - Rx done state"]
pub type RxDoneStateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START_INVALID_STATE` reader - Start invalid state"]
pub type StartInvalidStateR = crate::BitReader;
#[doc = "Field `START_INVALID_STATE` writer - Start invalid state"]
pub type StartInvalidStateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY_ERR_STATE` reader - Parity err state"]
pub type ParityErrStateR = crate::BitReader;
#[doc = "Field `PARITY_ERR_STATE` writer - Parity err state"]
pub type ParityErrStateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP_ERR_STATE` reader - Stop err state"]
pub type StopErrStateR = crate::BitReader;
#[doc = "Field `STOP_ERR_STATE` writer - Stop err state"]
pub type StopErrStateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_OVERFLOW_STATE` reader - Rx overflow state"]
pub type RxOverflowStateR = crate::BitReader;
#[doc = "Field `RX_OVERFLOW_STATE` writer - Rx overflow state"]
pub type RxOverflowStateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start valid state"]
    #[inline(always)]
    pub fn start_valid_state(&self) -> StartValidStateR {
        StartValidStateR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx done state"]
    #[inline(always)]
    pub fn rx_done_state(&self) -> RxDoneStateR {
        RxDoneStateR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start invalid state"]
    #[inline(always)]
    pub fn start_invalid_state(&self) -> StartInvalidStateR {
        StartInvalidStateR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity err state"]
    #[inline(always)]
    pub fn parity_err_state(&self) -> ParityErrStateR {
        ParityErrStateR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stop err state"]
    #[inline(always)]
    pub fn stop_err_state(&self) -> StopErrStateR {
        StopErrStateR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx overflow state"]
    #[inline(always)]
    pub fn rx_overflow_state(&self) -> RxOverflowStateR {
        RxOverflowStateR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start valid state"]
    #[inline(always)]
    pub fn start_valid_state(&mut self) -> StartValidStateW<'_, Sr0Spec> {
        StartValidStateW::new(self, 0)
    }
    #[doc = "Bit 1 - Rx done state"]
    #[inline(always)]
    pub fn rx_done_state(&mut self) -> RxDoneStateW<'_, Sr0Spec> {
        RxDoneStateW::new(self, 1)
    }
    #[doc = "Bit 2 - Start invalid state"]
    #[inline(always)]
    pub fn start_invalid_state(&mut self) -> StartInvalidStateW<'_, Sr0Spec> {
        StartInvalidStateW::new(self, 2)
    }
    #[doc = "Bit 3 - Parity err state"]
    #[inline(always)]
    pub fn parity_err_state(&mut self) -> ParityErrStateW<'_, Sr0Spec> {
        ParityErrStateW::new(self, 3)
    }
    #[doc = "Bit 4 - Stop err state"]
    #[inline(always)]
    pub fn stop_err_state(&mut self) -> StopErrStateW<'_, Sr0Spec> {
        StopErrStateW::new(self, 4)
    }
    #[doc = "Bit 5 - Rx overflow state"]
    #[inline(always)]
    pub fn rx_overflow_state(&mut self) -> RxOverflowStateW<'_, Sr0Spec> {
        RxOverflowStateW::new(self, 5)
    }
}
#[doc = "status register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sr0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sr0Spec;
impl crate::RegisterSpec for Sr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr0::R`](R) reader structure"]
impl crate::Readable for Sr0Spec {}
#[doc = "`write(|w| ..)` method takes [`sr0::W`](W) writer structure"]
impl crate::Writable for Sr0Spec {
    type Safety = crate::Unsafe;
}
