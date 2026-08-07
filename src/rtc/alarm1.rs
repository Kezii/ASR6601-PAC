#[doc = "Register `ALARM1` reader"]
pub type R = crate::R<Alarm1Spec>;
#[doc = "Register `ALARM1` writer"]
pub type W = crate::W<Alarm1Spec>;
#[doc = "Field `IT` reader - It"]
pub type ItR = crate::BitReader;
#[doc = "Field `IT` writer - It"]
pub type ItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKEN` reader - Wken"]
pub type WkenR = crate::BitReader;
#[doc = "Field `WKEN` writer - Wken"]
pub type WkenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - It"]
    #[inline(always)]
    pub fn it(&self) -> ItR {
        ItR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 26 - Wken"]
    #[inline(always)]
    pub fn wken(&self) -> WkenR {
        WkenR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - It"]
    #[inline(always)]
    pub fn it(&mut self) -> ItW<'_, Alarm1Spec> {
        ItW::new(self, 5)
    }
    #[doc = "Bit 26 - Wken"]
    #[inline(always)]
    pub fn wken(&mut self) -> WkenW<'_, Alarm1Spec> {
        WkenW::new(self, 26)
    }
}
#[doc = "alarm 1\n\nYou can [`read`](crate::Reg::read) this register and get [`alarm1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Alarm1Spec;
impl crate::RegisterSpec for Alarm1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarm1::R`](R) reader structure"]
impl crate::Readable for Alarm1Spec {}
#[doc = "`write(|w| ..)` method takes [`alarm1::W`](W) writer structure"]
impl crate::Writable for Alarm1Spec {
    type Safety = crate::Unsafe;
}
