#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `RORIC` writer - receive overrun interrupt clear"]
pub type RoricW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIC` writer - receive timeout interrupt clear"]
pub type RticW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - receive overrun interrupt clear"]
    #[inline(always)]
    pub fn roric(&mut self) -> RoricW<'_, IcrSpec> {
        RoricW::new(self, 0)
    }
    #[doc = "Bit 1 - receive timeout interrupt clear"]
    #[inline(always)]
    pub fn rtic(&mut self) -> RticW<'_, IcrSpec> {
        RticW::new(self, 1)
    }
}
#[doc = "interrupt clear register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
