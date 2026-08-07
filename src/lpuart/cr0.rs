#[doc = "Register `CR0` reader"]
pub type R = crate::R<Cr0Spec>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<Cr0Spec>;
#[doc = "Field `LOW_LEVEL_WAKEUP` reader - Low level wakeup"]
pub type LowLevelWakeupR = crate::BitReader;
#[doc = "Field `LOW_LEVEL_WAKEUP` writer - Low level wakeup"]
pub type LowLevelWakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START_WAKEUP` reader - Start wakeup"]
pub type StartWakeupR = crate::BitReader;
#[doc = "Field `START_WAKEUP` writer - Start wakeup"]
pub type StartWakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DONE_WAKEUP` reader - Rx done wakeup"]
pub type RxDoneWakeupR = crate::BitReader;
#[doc = "Field `RX_DONE_WAKEUP` writer - Rx done wakeup"]
pub type RxDoneWakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_ENABLE` reader - Rx enable"]
pub type RxEnableR = crate::BitReader;
#[doc = "Field `RX_ENABLE` writer - Rx enable"]
pub type RxEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTS_ENABLE` reader - Rts enable"]
pub type RtsEnableR = crate::BitReader;
#[doc = "Field `RTS_ENABLE` writer - Rts enable"]
pub type RtsEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 22 - Low level wakeup"]
    #[inline(always)]
    pub fn low_level_wakeup(&self) -> LowLevelWakeupR {
        LowLevelWakeupR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Start wakeup"]
    #[inline(always)]
    pub fn start_wakeup(&self) -> StartWakeupR {
        StartWakeupR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Rx done wakeup"]
    #[inline(always)]
    pub fn rx_done_wakeup(&self) -> RxDoneWakeupR {
        RxDoneWakeupR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx enable"]
    #[inline(always)]
    pub fn rx_enable(&self) -> RxEnableR {
        RxEnableR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Rts enable"]
    #[inline(always)]
    pub fn rts_enable(&self) -> RtsEnableR {
        RtsEnableR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 22 - Low level wakeup"]
    #[inline(always)]
    pub fn low_level_wakeup(&mut self) -> LowLevelWakeupW<'_, Cr0Spec> {
        LowLevelWakeupW::new(self, 22)
    }
    #[doc = "Bit 23 - Start wakeup"]
    #[inline(always)]
    pub fn start_wakeup(&mut self) -> StartWakeupW<'_, Cr0Spec> {
        StartWakeupW::new(self, 23)
    }
    #[doc = "Bit 24 - Rx done wakeup"]
    #[inline(always)]
    pub fn rx_done_wakeup(&mut self) -> RxDoneWakeupW<'_, Cr0Spec> {
        RxDoneWakeupW::new(self, 24)
    }
    #[doc = "Bit 25 - Rx enable"]
    #[inline(always)]
    pub fn rx_enable(&mut self) -> RxEnableW<'_, Cr0Spec> {
        RxEnableW::new(self, 25)
    }
    #[doc = "Bit 26 - Rts enable"]
    #[inline(always)]
    pub fn rts_enable(&mut self) -> RtsEnableW<'_, Cr0Spec> {
        RtsEnableW::new(self, 26)
    }
}
#[doc = "control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr0Spec;
impl crate::RegisterSpec for Cr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::Readable for Cr0Spec {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::Writable for Cr0Spec {
    type Safety = crate::Unsafe;
}
