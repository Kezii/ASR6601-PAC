#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `INTEN` reader - Watchdog interrupt enable"]
pub type IntenR = crate::BitReader;
#[doc = "Field `INTEN` writer - Watchdog interrupt enable"]
pub type IntenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESEN` reader - Watchdog reset enable"]
pub type ResenR = crate::BitReader;
#[doc = "Field `RESEN` writer - Watchdog reset enable"]
pub type ResenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Watchdog interrupt enable"]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog reset enable"]
    #[inline(always)]
    pub fn resen(&self) -> ResenR {
        ResenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog interrupt enable"]
    #[inline(always)]
    pub fn inten(&mut self) -> IntenW<'_, ControlSpec> {
        IntenW::new(self, 0)
    }
    #[doc = "Bit 1 - Watchdog reset enable"]
    #[inline(always)]
    pub fn resen(&mut self) -> ResenW<'_, ControlSpec> {
        ResenW::new(self, 1)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
}
