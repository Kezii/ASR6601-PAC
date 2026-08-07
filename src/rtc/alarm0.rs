#[doc = "Register `ALARM0` reader"]
pub type R = crate::R<Alarm0Spec>;
#[doc = "Register `ALARM0` writer"]
pub type W = crate::W<Alarm0Spec>;
#[doc = "Field `IT` reader - It"]
pub type ItR = crate::BitReader;
#[doc = "Field `IT` writer - It"]
pub type ItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKEN` reader - Wken"]
pub type WkenR = crate::BitReader;
#[doc = "Field `WKEN` writer - Wken"]
pub type WkenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - It"]
    #[inline(always)]
    pub fn it(&self) -> ItR {
        ItR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 27 - Wken"]
    #[inline(always)]
    pub fn wken(&self) -> WkenR {
        WkenR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - It"]
    #[inline(always)]
    pub fn it(&mut self) -> ItW<'_, Alarm0Spec> {
        ItW::new(self, 6)
    }
    #[doc = "Bit 27 - Wken"]
    #[inline(always)]
    pub fn wken(&mut self) -> WkenW<'_, Alarm0Spec> {
        WkenW::new(self, 27)
    }
}
#[doc = "alarm 0\n\nYou can [`read`](crate::Reg::read) this register and get [`alarm0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Alarm0Spec;
impl crate::RegisterSpec for Alarm0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarm0::R`](R) reader structure"]
impl crate::Readable for Alarm0Spec {}
#[doc = "`write(|w| ..)` method takes [`alarm0::W`](W) writer structure"]
impl crate::Writable for Alarm0Spec {
    type Safety = crate::Unsafe;
}
