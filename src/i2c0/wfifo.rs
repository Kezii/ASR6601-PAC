#[doc = "Register `WFIFO` reader"]
pub type R = crate::R<WfifoSpec>;
#[doc = "Register `WFIFO` writer"]
pub type W = crate::W<WfifoSpec>;
#[doc = "Field `CONTROL_START` reader - Control start"]
pub type ControlStartR = crate::BitReader;
#[doc = "Field `CONTROL_START` writer - Control start"]
pub type ControlStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONTROL_STOP` reader - Control stop"]
pub type ControlStopR = crate::BitReader;
#[doc = "Field `CONTROL_STOP` writer - Control stop"]
pub type ControlStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONTROL_ACKNAK` reader - Control acknak"]
pub type ControlAcknakR = crate::BitReader;
#[doc = "Field `CONTROL_ACKNAK` writer - Control acknak"]
pub type ControlAcknakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONTROL_TB` reader - Control tb"]
pub type ControlTbR = crate::BitReader;
#[doc = "Field `CONTROL_TB` writer - Control tb"]
pub type ControlTbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - Control start"]
    #[inline(always)]
    pub fn control_start(&self) -> ControlStartR {
        ControlStartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Control stop"]
    #[inline(always)]
    pub fn control_stop(&self) -> ControlStopR {
        ControlStopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Control acknak"]
    #[inline(always)]
    pub fn control_acknak(&self) -> ControlAcknakR {
        ControlAcknakR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Control tb"]
    #[inline(always)]
    pub fn control_tb(&self) -> ControlTbR {
        ControlTbR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Control start"]
    #[inline(always)]
    pub fn control_start(&mut self) -> ControlStartW<'_, WfifoSpec> {
        ControlStartW::new(self, 8)
    }
    #[doc = "Bit 9 - Control stop"]
    #[inline(always)]
    pub fn control_stop(&mut self) -> ControlStopW<'_, WfifoSpec> {
        ControlStopW::new(self, 9)
    }
    #[doc = "Bit 10 - Control acknak"]
    #[inline(always)]
    pub fn control_acknak(&mut self) -> ControlAcknakW<'_, WfifoSpec> {
        ControlAcknakW::new(self, 10)
    }
    #[doc = "Bit 11 - Control tb"]
    #[inline(always)]
    pub fn control_tb(&mut self) -> ControlTbW<'_, WfifoSpec> {
        ControlTbW::new(self, 11)
    }
}
#[doc = "write fifo register\n\nYou can [`read`](crate::Reg::read) this register and get [`wfifo::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wfifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WfifoSpec;
impl crate::RegisterSpec for WfifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wfifo::R`](R) reader structure"]
impl crate::Readable for WfifoSpec {}
#[doc = "`write(|w| ..)` method takes [`wfifo::W`](W) writer structure"]
impl crate::Writable for WfifoSpec {
    type Safety = crate::Unsafe;
}
