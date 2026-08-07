#[doc = "Register `EGR` writer"]
pub type W = crate::W<EgrSpec>;
#[doc = "Field `UG` writer - Ug"]
pub type UgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC0G` writer - Cc0g"]
pub type Cc0gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1G` writer - Cc1g"]
pub type Cc1gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2G` writer - Cc2g"]
pub type Cc2gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3G` writer - Cc3g"]
pub type Cc3gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG` writer - Tg"]
pub type TgW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Ug"]
    #[inline(always)]
    pub fn ug(&mut self) -> UgW<'_, EgrSpec> {
        UgW::new(self, 0)
    }
    #[doc = "Bit 1 - Cc0g"]
    #[inline(always)]
    pub fn cc0g(&mut self) -> Cc0gW<'_, EgrSpec> {
        Cc0gW::new(self, 1)
    }
    #[doc = "Bit 2 - Cc1g"]
    #[inline(always)]
    pub fn cc1g(&mut self) -> Cc1gW<'_, EgrSpec> {
        Cc1gW::new(self, 2)
    }
    #[doc = "Bit 3 - Cc2g"]
    #[inline(always)]
    pub fn cc2g(&mut self) -> Cc2gW<'_, EgrSpec> {
        Cc2gW::new(self, 3)
    }
    #[doc = "Bit 4 - Cc3g"]
    #[inline(always)]
    pub fn cc3g(&mut self) -> Cc3gW<'_, EgrSpec> {
        Cc3gW::new(self, 4)
    }
    #[doc = "Bit 6 - Tg"]
    #[inline(always)]
    pub fn tg(&mut self) -> TgW<'_, EgrSpec> {
        TgW::new(self, 6)
    }
}
#[doc = "TIMER event generation register, Address\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EgrSpec;
impl crate::RegisterSpec for EgrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`egr::W`](W) writer structure"]
impl crate::Writable for EgrSpec {
    type Safety = crate::Unsafe;
}
