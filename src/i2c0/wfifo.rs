#[doc = "Register `WFIFO` writer"]
pub type W = crate::W<WfifoSpec>;
#[doc = "Field `DATA` writer - i2c bus send data for write transactions"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONTROL_START` writer - Control start"]
pub type ControlStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONTROL_STOP` writer - Control stop"]
pub type ControlStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONTROL_ACKNAK` writer - Control acknak"]
pub type ControlAcknakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONTROL_TB` writer - Control tb"]
pub type ControlTbW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:7 - i2c bus send data for write transactions"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, WfifoSpec> {
        DataW::new(self, 0)
    }
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
#[doc = "write fifo register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wfifo::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WfifoSpec;
impl crate::RegisterSpec for WfifoSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wfifo::W`](W) writer structure"]
impl crate::Writable for WfifoSpec {
    type Safety = crate::Unsafe;
}
